use crate::api::PduApi;
use crate::error::{GeneralError, GeneralResult};
use crate::types::pdu_event::{PduErrorEvent, PduEvent, PduEventData};
use crate::types::pdu_status::{PduStatusData, PduStatusTarget};
use crate::types::{PduCllHandle, PduCopHandle, PduModuleHandle, PduUniqueCopTag};
use crate::worker::{PduAsyncWorker, Query};
use dpdu_api_types::{PduCopt, PduStatus};
use parking_lot::Mutex as ParkingLotMutex;
use std::ops::{Bound, Deref, RangeBounds};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock, Weak};
use std::thread::spawn;
use tokio::sync::Mutex as TokioMutex;
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
use tracing::{debug, error};

pub type CopResult<T> = std::result::Result<T, CopError>;

pub type StopReceive = bool;

#[non_exhaustive]
#[derive(Debug, Clone, thiserror::Error)]
pub enum CopError {
    #[error("{}", .0.code.as_str())]
    CopError(PduErrorEvent),

    #[error("internal error: {0}")]
    InternalError(#[from] InternalCopError),
}

#[non_exhaustive]
#[derive(Debug, Clone, thiserror::Error)]
pub enum InternalCopError {
    #[error("cop is dead")]
    DeadError,

    #[error("invalid type of cop")]
    InvalidCopTypeError,

    #[error("invalid receive cycles")]
    InvalidReceiveCyclesError,

    #[error("no cop params")]
    NoParamsError,

    #[error("event channel closed")]
    EventChannelClosed,

    #[error("invalid cop event type: {}", .0.data.as_str())]
    InvalidEventTypeError(PduEvent),
}

impl From<InternalCopError> for GeneralError {
    fn from(value: InternalCopError) -> Self {
        GeneralError::CopError(CopError::InternalError(value))
    }
}

#[derive(Debug, Clone)]
pub struct PduPrimitive {
    pub(crate) me: Weak<PduPrimitive>,

    pub(crate) api: Arc<PduApi>,

    pub(crate) worker: OnceLock<Arc<PduAsyncWorker>>,

    pub(crate) unique_tag: PduUniqueCopTag,

    pub(crate) h_mod: PduModuleHandle,

    pub(crate) h_cll: PduCllHandle,

    pub(crate) cop_data: PduCopData,

    pub(crate) params: Arc<Option<PduPrimitiveParams>>,

    pub(crate) event_tx: Arc<mpsc::Sender<PduEvent>>,

    pub(crate) event_rx: Arc<TokioMutex<mpsc::Receiver<PduEvent>>>,

    pub(crate) dead: Arc<AtomicBool>,

    /// Синхронизация при обращениях к D-PDU API.
    pub(crate) pdu_sync: Arc<ParkingLotMutex<()>>,

    /// Синхронизация для получения сообщений из приёмника.
    pub(crate) event_sync: Arc<TokioMutex<()>>,
}

impl PduPrimitive {
    pub fn get_module_handle(&self) -> PduModuleHandle {
        self.h_mod
    }

    pub fn get_cll_handle(&self) -> PduCllHandle {
        self.h_cll
    }

    pub fn get_cop_handle(&self) -> PduCopHandle {
        self.cop_data.h_cop
    }

    pub fn get_params(&self) -> Option<&PduPrimitiveParams> {
        self.params.as_ref().as_ref()
    }

    pub fn get_unique_tag(&self) -> PduUniqueCopTag {
        self.unique_tag
    }

    fn take_me_expect(&self) -> Arc<PduPrimitive> {
        self.me
            .upgrade()
            .expect("internal error: ComPrimitive self-reference is no longer valid")
    }

    pub fn blocking_get_status(&self) -> GeneralResult<CopStatus> {
        let _sync_guard = self.pdu_sync.lock();
        let target = PduStatusTarget::Primitive(
            self.get_module_handle(),
            self.get_cll_handle(),
            self.get_cop_handle(),
        );
        let result = self.api.pdu_get_status(&target)?;
        Ok(CopStatus(result))
    }

    pub async fn get_status(&self) -> GeneralResult<CopStatus> {
        let h_mod = self.get_module_handle();
        let h_cll = self.get_cll_handle();
        let h_cop = self.get_cop_handle();

        match self.worker.get() {
            Some(worker) => {
                let target = PduStatusTarget::Primitive(h_mod, h_cll, h_cop);
                let result = worker.pdu_get_status(target).await?;
                Ok(CopStatus(result))
            }
            None => {
                let me = self.take_me_expect();
                let result = spawn_blocking(move || me.blocking_get_status())
                    .await
                    .expect("internal error: ComPrimitive::blocking_get_status() task panicked")?;

                Ok(result)
            }
        }
    }

    pub fn blocking_cancel(&self) -> GeneralResult<()> {
        let _sync_guard = self.pdu_sync.lock();
        let result = self
            .api
            .pdu_cancel_com_primitive(self.h_mod, self.h_cll, self.cop_data.h_cop);

        if result.is_ok() {
            self.mark_as_dead();
        }

        Ok(())
    }

    pub async fn cancel(&self) -> GeneralResult<()> {
        let h_mod = self.get_module_handle();
        let h_cll = self.get_cll_handle();
        let h_cop = self.get_cop_handle();

        match self.worker.get() {
            Some(worker) => {
                worker.pdu_cancel_com_primitive(h_mod, h_cll, h_cop).await?;
                self.mark_as_dead();
                Ok(())
            }
            None => {
                let me = self.take_me_expect();

                spawn_blocking(move || me.blocking_cancel())
                    .await
                    .expect("internal error: ComPrimitive::blocking_cancel() task panicked")?;

                Ok(())
            }
        }
    }

    pub fn is_dead(&self) -> bool {
        self.dead.load(Ordering::Acquire)
    }

    fn mark_as_dead(&self) {
        self.dead.store(true, Ordering::Release)
    }

    fn assert_dead(&self) -> CopResult<()> {
        if self.is_dead() {
            return Err(InternalCopError::DeadError)?;
        }
        Ok(())
    }

    fn assert_type(&self, allowed: &[PduCopt]) -> CopResult<()> {
        if !allowed.contains(&self.cop_data.cop_type) {
            return Err(InternalCopError::InvalidCopTypeError)?;
        }
        Ok(())
    }

    fn assert_get_params(&self) -> CopResult<&PduPrimitiveParams> {
        self.params
            .deref()
            .as_ref()
            .ok_or(CopError::InternalError(InternalCopError::NoParamsError))
    }

    fn assert_receive_cycles<R>(&self, range: R) -> CopResult<()>
    where
        R: RangeBounds<i32>,
    {
        let params = self.assert_get_params()?;
        let cycles = params.receive_cycles.to_i32();

        let min = match range.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
            Bound::Unbounded => 0,
        };

        let max = match range.end_bound() {
            Bound::Included(&x) => Some(x),
            Bound::Excluded(&x) => Some(x - 1),
            Bound::Unbounded => None,
        };

        if let Some(max) = max
            && min > max
        {
            panic!("internal error: start ({min}) is greater than end ({max})");
        }

        if cycles < min {
            return Err(InternalCopError::InvalidReceiveCyclesError)?;
        } else if let Some(max) = max
            && cycles > max
        {
            return Err(InternalCopError::InvalidReceiveCyclesError)?;
        }

        Ok(())
    }

    fn handle_single_receive_cycle(
        &self,
        events: &mut CopEvents,
        rx: &mut mpsc::Receiver<PduEvent>,
        event: &PduEvent,
    ) -> GeneralResult<StopReceive> {
        match &event.data {
            PduEventData::Status(status) => {
                let status = &status.0;

                let is_finished = matches!(status, PduStatus::CopstFinished);
                let is_cancelled = matches!(status, PduStatus::CopstCancelled);

                if is_finished || is_cancelled {
                    self.mark_as_dead();
                    rx.close();
                    return Ok(true);
                }
            }
            PduEventData::Error(data) => {
                self.mark_as_dead();
                rx.close();
                return Err(CopError::CopError(data.to_owned()))?;
            }
            PduEventData::Result(..) => {
                events.store_event(event.to_owned());
            }
            _ => {
                return Err(InternalCopError::InvalidEventTypeError(event.to_owned()))?;
            }
        }

        Ok(false)
    }

    /// Только если
    /// - cop_type = SendRecv || cop_type == StartComm
    /// - receive_cycles = 1.
    pub fn blocking_get_single_result(&self) -> GeneralResult<Vec<u8>> {
        let _event_sync = self.event_sync.blocking_lock();

        self.assert_dead()?;
        self.assert_type(&[PduCopt::StartComm, PduCopt::SendRecv, PduCopt::StopComm])?;
        self.assert_receive_cycles(1..=1)?;

        let mut rx = self.event_rx.blocking_lock();
        let mut events = CopEvents::default();

        loop {
            let event = rx
                .blocking_recv()
                .ok_or(InternalCopError::EventChannelClosed)?;

            if self.handle_single_receive_cycle(&mut events, &mut rx, &event)? {
                break;
            }
        }

        Ok(events.get_data())
    }

    pub async fn get_single_result(&self) -> GeneralResult<Vec<u8>> {
        let _event_sync = self.event_sync.lock().await;

        self.assert_dead()?;
        self.assert_type(&[PduCopt::StartComm, PduCopt::SendRecv, PduCopt::StopComm])?;
        self.assert_receive_cycles(1..=1)?;

        let mut rx = self.event_rx.lock().await;
        let mut events = CopEvents::default();

        loop {
            let event = rx
                .recv()
                .await
                .ok_or(InternalCopError::EventChannelClosed)?;

            if self.handle_single_receive_cycle(&mut events, &mut rx, &event)? {
                break;
            }
        }

        Ok(events.get_data())
    }

    fn handle_next_receive_cycle(
        &self,
        events: &mut CopEvents,
        rx: &mut mpsc::Receiver<PduEvent>,
        event: &PduEvent,
    ) -> GeneralResult<StopReceive> {
        let events_len_before = events.0.len();

        let stop_receive = self.handle_single_receive_cycle(events, rx, event)?;
        if stop_receive {
            return Ok(true);
        }

        let events_len_after = events.0.len();

        if events_len_before != events_len_after {
            return Ok(true);
        }

        Ok(false)
    }

    pub fn blocking_get_next_result(&self) -> GeneralResult<Vec<u8>> {
        let _event_sync = self.event_sync.blocking_lock();

        self.assert_dead()?;
        self.assert_type(&[PduCopt::StartComm, PduCopt::SendRecv, PduCopt::StopComm])?;
        self.assert_receive_cycles(-2..)?;

        let mut rx = self.event_rx.blocking_lock();
        let mut events = CopEvents::default();

        loop {
            let event = rx
                .blocking_recv()
                .ok_or(InternalCopError::EventChannelClosed)?;

            if self.handle_next_receive_cycle(&mut events, &mut rx, &event)? {
                break;
            }
        }

        Ok(events.get_data())
    }

    pub async fn get_next_result(&self) -> GeneralResult<Vec<u8>> {
        let _event_sync = self.event_sync.lock();

        self.assert_dead()?;
        self.assert_type(&[PduCopt::StartComm, PduCopt::SendRecv, PduCopt::StopComm])?;
        self.assert_receive_cycles(-2..)?;

        let mut rx = self.event_rx.lock().await;
        let mut events = CopEvents::default();

        loop {
            let event = rx
                .recv()
                .await
                .ok_or(InternalCopError::EventChannelClosed)?;

            if self.handle_next_receive_cycle(&mut events, &mut rx, &event)? {
                break;
            }
        }

        Ok(events.get_data())
    }
}

impl Drop for PduPrimitive {
    fn drop(&mut self) {
        if self.is_dead() {
            return;
        }

        let h_mod = self.get_module_handle();
        let h_cll = self.get_cll_handle();
        let h_cop = self.cop_data.h_cop;

        debug!(
            h_mod,
            h_cll, h_cop, "Cancelling the PduPrimitive via destructor..."
        );

        match self.worker.get() {
            Some(worker) => {
                let query = Query::VtCopDestructor(h_mod, h_cll, h_cop);
                match worker.request(query, None) {
                    Ok(_) => {}
                    Err(err) => {
                        error!(
                            h_mod,
                            h_cll,
                            h_cop,
                            "Error when cancelling the PduPrimitive via destructor: {err}"
                        );
                    }
                }
            }
            None => {
                let api = self.api.clone();
                spawn(move || api.vt_cop_destructor(h_mod, h_cll, h_cop));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CopStatus(PduStatusData);

impl CopStatus {
    pub fn is_idle(&self) -> bool {
        matches!(self.0.status_code, PduStatus::CopstIdle)
    }

    pub fn is_executing(&self) -> bool {
        matches!(self.0.status_code, PduStatus::CopstExecuting)
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.0.status_code, PduStatus::CopstFinished)
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self.0.status_code, PduStatus::CopstCancelled)
    }

    pub fn is_waiting(&self) -> bool {
        matches!(self.0.status_code, PduStatus::CopstWaiting)
    }
}

#[derive(Debug, Clone)]
pub struct PduPrimitiveParams {
    /// Время цикла в мс для циклической операции отправки или время задержки для PDU_COPT_DELAY.
    pub time: u32,

    /// Количество выполняемых циклов отправки.
    pub send_cycles: SendCycles,

    /// Количество выполняемых циклов приёма.
    pub receive_cycles: ReceiveCycles,

    /// See the [`ComParamBuffer`].
    pub temp_param_update: ComParamBuffer,

    pub tx_flag: TransmitFlags,

    pub expected_responses: Vec<ExpectedResponse>,
}

impl Default for PduPrimitiveParams {
    fn default() -> Self {
        Self {
            time: 0,
            send_cycles: SendCycles::default(),
            receive_cycles: ReceiveCycles::default(),
            temp_param_update: ComParamBuffer::default(),
            tx_flag: TransmitFlags::default(),
            expected_responses: vec![ExpectedResponse {
                response_type: ResponseType::Positive,
                acceptance_id: 1,
                mask_data: MaskData::default(),
                unique_response_ids: vec![],
            }],
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransmitFlags {
    /// Подавление положительных ответов (часть ODX).
    ///
    /// Применимо для: ISO 15765-3/ISO14229-3.
    pub suppress_positive_response: bool,

    /// Добавлять дополнительную информацию к результирующим данным.
    /// Эта информация может быть полезна для отладки ответов от ECU.
    pub enable_extra_info: bool,

    /// Используется для уменьшения времени программирования, если приложение знает, что
    /// будет получен только один ответ.
    ///
    /// Не влияет на тайминги ответов на функциональные запросы.
    ///
    /// Действует только в сочетании с флагом [raw mode] при создании ComLogicalLink.
    ///
    /// | Значение | Описание |
    /// |----------|-------------------------------|
    /// | true | Время передачи сообщений интерфейса, как указано в стандарте ISO 14230 (по умолчанию). |
    /// | false | После получения ответа на физический запрос время ожидания сокращается до P3_MIN. |
    pub wait_p3_min_only: bool,

    /// Тип CAN идентификатора сообщения для ISO 11898, SAE J1939 и ISO 15765.
    ///
    /// Действует только в сочетании с флагом [raw mode] при создании ComLogicalLink.
    ///
    /// CAN ID содержится в первых 4-х байтах PDU Data.
    ///
    /// | Значение | Описание |
    /// |----------|----------|
    /// | true | 29-bit |
    /// | false | 11-bit (по умолчанию) |
    pub can_29_bit: bool,

    /// Метод адресации ISO 15765-2.
    ///
    /// Определяет, будет ли расширенный адрес CAN содержаться в байте, следующем за CAN ID в
    /// данных PDU.
    ///
    /// Действует только в сочетании с флагом [raw mode] при создании ComLogicalLink.
    ///
    /// | Значение | Описание |
    /// |----------|----------|
    /// | true | используется расширенная адресация |
    /// | false | нет расширенной адресации (по умолчанию) |
    pub iso_15765_addr_type: bool,

    /// Дополнение (padding) фреймов для ISO 15765-2.
    ///
    /// Действует только в сочетании с флагом [raw mode] при создании ComLogicalLink.
    ///
    /// | Значение | Описание |
    /// |----------|----------|
    /// | true | дополнять CAN сообщение до его DLC, используя ComParam CP_CanFillerByte |
    /// | false | дополнение отключено (по умолчанию) |
    pub iso_15765_frame_pad: bool,
}

impl Default for TransmitFlags {
    fn default() -> Self {
        Self {
            suppress_positive_response: false,
            enable_extra_info: false,
            wait_p3_min_only: false,
            can_29_bit: false,
            iso_15765_addr_type: false,
            iso_15765_frame_pad: false,
        }
    }
}

impl TransmitFlags {
    pub(crate) fn zero_byte(&self) -> u8 {
        let mut accumulator = 0;

        match self.suppress_positive_response {
            true => accumulator |= 0x40,
            false => accumulator &= 0xBF,
        }

        match self.enable_extra_info {
            true => accumulator |= 0x20,
            false => accumulator &= 0xDF,
        }

        accumulator
    }

    pub(crate) fn second_byte(&self) -> u8 {
        let mut accumulator = 0;

        match self.wait_p3_min_only {
            true => accumulator |= 0x02,
            false => accumulator &= 0xFD,
        }

        match self.can_29_bit {
            true => accumulator |= 0x01,
            false => accumulator &= 0xFE,
        }

        accumulator
    }

    pub(crate) fn third_byte(&self) -> u8 {
        let mut accumulator = 0;

        match self.iso_15765_addr_type {
            true => accumulator |= 0x80,
            false => accumulator &= 0x7F,
        }

        match self.iso_15765_frame_pad {
            true => accumulator |= 0x40,
            false => accumulator &= 0xBF,
        }

        accumulator
    }

    pub(crate) fn get_pdu_flag_data(&self) -> [u8; 4] {
        [self.zero_byte(), 0, self.second_byte(), self.third_byte()]
    }
}

#[derive(Debug, Clone)]
pub struct ExpectedResponse {
    pub response_type: ResponseType,

    /// ID assigned by application to be returned in PDU_RESULT_DATA,
    /// which indicates which expected response matched
    pub acceptance_id: u32,

    pub mask_data: MaskData,

    /// Массив уникальных идентификаторов ответов может использоваться, если ожидаемый ответ
    /// появляется только для определенных уникальных идентификаторов ответов.
    ///
    /// Такая ситуация может возникнуть в случае функциональной адресации,
    /// когда возможные ответы не являются общими для всех ЭБУ.
    ///
    /// Количество уникальных идентификаторов ответа может быть равно 0.
    /// В этом случае массив pUniqueRespIds не используется, и все ответы с любым уникальным
    /// идентификатором ответа учитываются при попытке сопоставить фактические данные ответа
    /// с ожидаемыми данными ответа.
    pub unique_response_ids: Vec<u32>,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, strum::AsRefStr)]
pub enum ResponseType {
    Positive = 0,
    Negative = 1,
}

impl ResponseType {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

/// При фильтрации по приему API D-PDU пытается сопоставить байты данных полученного ответа с
/// байтами шаблона ожидаемого ответа (всегда с учетом байтов маски).
/// Количество байтов данных в полученном ответе может отличаться от количества байтов
/// маски и шаблона в ожидаемом ответе.
///
/// Фильтрация приема использует следующие правила:
///
///   - Если количество полученных байтов данных меньше NumMaskPatternBytes, то ответ не
///     соответствует ожидаемому.
///
///   - Если количество полученных байтов данных равно NumMaskPatternBytes, все байты данных
///     сравниваются с байтами данных шаблона.
///
///   - Если количество полученных байтов данных превышает NumMaskPatternBytes, только начальные
///     байты данных полученного ответа сравниваются со всеми байтами данных шаблона ожидаемого
///     ответа. Все последующие байты данных в полученном ответе являются «безразличными».
#[derive(Debug, Clone, Default)]
pub struct MaskData {
    pub(crate) mask: Vec<u8>,
    pub(crate) pattern: Vec<u8>,
}

impl MaskData {
    pub fn new(mask: &[u8], pattern: &[u8]) -> Option<Self> {
        if mask.len() != pattern.len() {
            return None;
        }

        Some(Self {
            mask: mask.to_vec(),
            pattern: pattern.to_vec(),
        })
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.mask.len(), self.pattern.len());
        self.mask.len()
    }

    pub fn get_mask(&self) -> &[u8] {
        assert_eq!(self.mask.len(), self.pattern.len());
        &self.mask
    }

    pub fn get_pattern(&self) -> &[u8] {
        assert_eq!(self.mask.len(), self.pattern.len());
        &self.pattern
    }
}

#[derive(Debug, Clone)]
pub enum SendCycles {
    Normal(u32),
    Infinite,
}

impl Default for SendCycles {
    fn default() -> Self {
        SendCycles::Normal(0)
    }
}

impl SendCycles {
    pub fn to_i32(&self) -> i32 {
        match self {
            SendCycles::Normal(v) => {
                i32::try_from(*v).expect("SendCycles value is too large for i32: {v}")
            }
            SendCycles::Infinite => -1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReceiveCycles {
    Normal(u32),

    Infinite,

    /// When a ComPrimitive is configured with NumSendCycles set to 1 and NumReceiveCycles set to -2
    /// (IS-MULTIPLE), the MVCI protocol module expects responses from one or more ECUs. Until a specific
    /// timeout expires, the MVCI protocol module receives responses and tries to match the Unique Response ID for
    /// each response (see the following subclauses for details). The Unique Response Id is saved and then returned
    /// in a result item when the payload data is matched to a ComPrimitive expected response. The application
    /// retrieves the result items from the ComLogicalLink's Event Queue (details of event notification are not shown
    /// in the diagram). When processing the result data, the application is able to assign the data to a certain ECU
    /// via the Unique Response ID.
    Multiple,
}

impl Default for ReceiveCycles {
    fn default() -> Self {
        ReceiveCycles::Normal(0)
    }
}

impl ReceiveCycles {
    pub fn to_i32(&self) -> i32 {
        match self {
            ReceiveCycles::Normal(v) => {
                i32::try_from(*v).expect("ReceiveCycles value is too large for i32: {v}")
            }
            ReceiveCycles::Infinite => -1,
            ReceiveCycles::Multiple => -2,
        }
    }
}

/// Temporary ComParam settings for the ComPrimitive.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Default, strum::AsRefStr)]
pub enum ComParamBuffer {
    /// Do not use temporary ComParams for this ComPrimitive. The
    /// ComPrimitive shall attach the “Active” ComParam buffer to the
    /// ComPrimitive. This buffer shall be in effect for the ComPrimitive until it is
    /// finished. The ComParams for the ComPrimitive will not change even if
    /// the “Active” buffer is modified by a subsequent ComPrimitive type of
    /// PDU_COPT_UPDATEPARAM
    #[default]
    Active = 0,

    /// Use temporary ComParams for this ComPrimitive; The
    /// ComPrimitive shall attach the ComParam “Working” buffer to the
    /// ComPrimitive. This buffer shall be in effect for the ComPrimitive until it is
    /// finished. The ComParams for the ComPrimitive will not change even if
    /// the “Active” or “Working” buffers are modified by any subsequent calls to
    /// PDUSetComParam.
    Working = 1,
}

impl ComParamBuffer {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

#[derive(Debug, Default)]
pub struct CopEvents(Vec<PduEvent>);

impl CopEvents {
    pub(crate) fn store_event(&mut self, event: PduEvent) {
        self.0.push(event);
    }

    pub fn get_data_parts(&self) -> Vec<&[u8]> {
        self.0
            .iter()
            .filter_map(|v| match &v.data {
                PduEventData::Result(result) => Some(result.data.as_slice()),
                _ => None,
            })
            .collect()
    }

    pub fn get_data(&self) -> Vec<u8> {
        let capacity: usize = self
            .0
            .iter()
            .filter_map(|v| match &v.data {
                PduEventData::Result(result) => Some(result.data.len()),
                _ => None,
            })
            .sum();

        let mut vec = Vec::with_capacity(capacity);

        for event in &self.0 {
            if let PduEventData::Result(result) = &event.data {
                vec.extend_from_slice(&result.data);
            }
        }

        vec
    }

    pub fn get_errors(&self) -> Vec<&PduErrorEvent> {
        self.0
            .iter()
            .filter_map(|v| match &v.data {
                PduEventData::Error(err) => Some(err),
                _ => None,
            })
            .collect()
    }

    pub fn has_errors(&self) -> bool {
        self.0
            .iter()
            .any(|i| matches!(i.data, PduEventData::Error { .. }))
    }
}

#[derive(Debug, Clone)]
pub struct PduCopData {
    pub(crate) h_cop: PduCopHandle,

    pub(crate) cop_type: PduCopt,
}

impl PduCopData {
    pub fn get_cop_handle(&self) -> PduCopHandle {
        self.h_cop
    }

    pub fn get_type(&self) -> PduCopt {
        self.cop_type
    }

    pub fn is_start_comm(&self) -> bool {
        matches!(self.cop_type, PduCopt::StartComm)
    }

    pub fn is_stop_comm(&self) -> bool {
        matches!(self.cop_type, PduCopt::StopComm)
    }

    pub fn is_update_param(&self) -> bool {
        matches!(self.cop_type, PduCopt::UpdateParam)
    }

    pub fn is_send_recv(&self) -> bool {
        matches!(self.cop_type, PduCopt::SendRecv)
    }

    pub fn is_delay(&self) -> bool {
        matches!(self.cop_type, PduCopt::Delay)
    }

    pub fn is_restore_param(&self) -> bool {
        matches!(self.cop_type, PduCopt::RestoreParam)
    }
}
