use crate::types::pdu_resource::{BusSource, ProtocolSource, TargetPin};
use crate::types::{PduCllHandle, PduModuleHandle, PduObjectId, PduUniqueApiTag, PduUniqueCllTag, PduUniqueCopTag};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::{Arc, OnceLock, Weak};
use std::thread::spawn;
use std::time::Duration;
use dpdu_api_types::{PduCopt, PduStatus};
use parking_lot::Mutex as ParkingLotMutex;
use tokio::sync::Mutex as TokioMutex;
use rand::random;
use tokio::task::spawn_blocking;
use tracing::{debug, error};
use tokio::sync::{mpsc};
use crate::api::{ApiResult, PduApi};
use crate::event_callback::event_callback;
use crate::handle_manager::PduHandleManager;
use crate::types::pdu_com_primitive::{ComParamBuffer, ExpectedResponse, PduComPrimitive, PduComPrimitiveParams, ReceiveCycles, ResponseType, SendCycles, TransmitFlags};
use crate::types::pdu_event::{PduEvent, PduEventTarget};
use crate::types::pdu_status::{PduStatusData, PduStatusTarget};
use crate::types::pdu_vci::{PduVci, VciStatus};
use crate::utils::random_non_zero_usize;
use crate::worker::{PduAsyncWorker, Query, WorkerResult};

#[derive(Debug, Clone)]
pub struct PduComLogicalLink {
    pub(crate) me: Weak<PduComLogicalLink>,

    pub(crate) api: Arc<PduApi>,

    pub(crate) worker: OnceLock<Arc<PduAsyncWorker>>,

    pub(crate) unique_tag: PduUniqueCllTag,

    pub(crate) cll_data: Arc<PduCllData>,

    pub(crate) event_tx: Arc<mpsc::Sender<PduEvent>>,

    pub(crate) event_rx: Arc<mpsc::Receiver<PduEvent>>,

    pub(crate) sync: Arc<ParkingLotMutex<()>>,
}

impl PduComLogicalLink {
    const DEFAULT_COP_EVENT_QUEUE_SIZE: usize = 4096;

    pub(crate) fn set_worker(&self, worker: Arc<PduAsyncWorker>) {
        let _ = self.worker.set(worker);
    }

    pub fn get_module_handle(&self) -> PduModuleHandle {
        self.cll_data.h_mod
    }

    pub fn get_cll_handle(&self) -> PduCllHandle {
        self.cll_data.h_cll
    }

    pub fn get_create_flags(&self) -> &CllCreateFlags {
        &self.cll_data.create_flags
    }

    pub fn get_create_type(&self) -> &CllCreateType {
        &self.cll_data.create_type
    }

    pub fn get_unique_tag(&self) -> PduUniqueCllTag {
        self.unique_tag
    }

    fn take_me_expect(&self) -> Arc<PduComLogicalLink> {
        self.me
            .upgrade()
            .expect("internal error: ComLogicalLink self-reference is no longer valid")
    }

    pub fn blocking_get_status(&self) -> ApiResult<CllStatus> {
        let _sync_guard = self.sync.lock();
        let target = PduStatusTarget::ComLogicalLink(
            self.get_module_handle(),
            self.get_cll_handle()
        );
        let result = self.api.pdu_get_status(&target)?;
        Ok(CllStatus(result))
    }

    pub async fn get_status(&self) -> WorkerResult<CllStatus> {
        match self.worker.get() {
            Some(worker) => {
                let target = PduStatusTarget::ComLogicalLink(
                    self.get_module_handle(),
                    self.get_cll_handle()
                );
                let result = worker.pdu_get_status(target).await?;
                Ok(CllStatus(result))
            },
            None => {
                debug!(
                    h_mod = self.get_module_handle(),
                    h_cll = self.get_cll_handle(),
                    "The use of asynchronous functions is not recommended outside of PduAsyncWorker"
                );

                let me = self.take_me_expect();
                let result = spawn_blocking(move || me.blocking_get_status())
                    .await
                    .expect("internal error: ComLogicalLink::blocking_get_status() task panicked")?;

                Ok(result)
            }
        }
    }

    pub fn blocking_connect(&self) -> ApiResult<bool> {
        let status = self.blocking_get_status()?;
        if !status.is_offline() {
            return Ok(false);
        }

        let _sync_guard = self.sync.lock();
        self.api.pdu_connect(self.get_module_handle(), self.get_cll_handle())?;
        Ok(true)
    }

    pub async fn connect(&self) -> WorkerResult<bool> {
        match self.worker.get() {
            Some(worker) => {
                let status = self.get_status().await?;
                if !status.is_offline() {
                    return Ok(false);
                }
                worker.pdu_connect(self.get_module_handle(), self.get_cll_handle()).await?;
                Ok(true)
            },
            None => {
                debug!(
                    h_mod = self.get_module_handle(),
                    h_cll = self.get_cll_handle(),
                    "The use of asynchronous functions is not recommended outside of PduAsyncWorker"
                );

                let me = self.take_me_expect();
                let result = spawn_blocking(move || me.blocking_connect())
                    .await
                    .expect("internal error: ComLogicalLink::blocking_connect() task panicked")?;

                Ok(result)
            }
        }
    }

    pub fn blocking_disconnect(&self) -> ApiResult<bool> {
        let status = self.blocking_get_status()?;
        if status.is_offline() {
            return Ok(false);
        }

        let _sync_guard = self.sync.lock();
        self.api.pdu_disconnect(self.get_module_handle(), self.get_cll_handle())?;
        Ok(true)
    }

    pub async fn disconnect(&self) -> WorkerResult<bool> {
        match self.worker.get() {
            Some(worker) => {
                let status = self.get_status().await?;
                if status.is_offline() {
                    return Ok(false);
                }
                worker.pdu_disconnect(self.get_module_handle(), self.get_cll_handle()).await?;
                Ok(true)
            },
            None => {
                debug!(
                    h_mod = self.get_module_handle(),
                    h_cll = self.get_cll_handle(),
                    "The use of asynchronous functions is not recommended outside of PduAsyncWorker"
                );

                let me = self.take_me_expect();
                let result = spawn_blocking(move || me.blocking_connect())
                    .await
                    .expect("internal error: ComLogicalLink::blocking_disconnect() task panicked")?;

                Ok(result)
            }
        }
    }

    pub fn blocking_start_com_primitive(
        &self,
        cop_type: &PduCopt,
        data: &[u8],
        params: Option<&PduComPrimitiveParams>
    ) -> ApiResult<Arc<PduComPrimitive>> {
        let _sync_guard = self.sync.lock();

        let unique_tag: PduUniqueCopTag = random_non_zero_usize();
        let (tx, rx) = mpsc::channel(Self::DEFAULT_COP_EVENT_QUEUE_SIZE);
        let tx = Arc::new(tx);

        // Register event tx for unique tag.
        PduHandleManager::register_cop(self.api.unique_tag, unique_tag, Some(Arc::downgrade(&tx)), None);

        let h_mod = self.get_module_handle();
        let h_cll = self.get_cll_handle();
        let cop_data = self.api.pdu_start_com_primitive(
            h_mod,
            h_cll,
            cop_type.to_owned(),
            data,
            params,
            Some(unique_tag)
        )?;

        let cop = Arc::new_cyclic(|weak| PduComPrimitive {
            me: weak.clone(),
            api: self.api.clone(),
            worker: OnceLock::default(),
            unique_tag,
            h_mod,
            h_cll,
            cop_data,
            params: params.cloned().into(),
            event_tx: tx,
            event_rx: Arc::new(TokioMutex::new(rx)),
            dead: Arc::default(),
            pdu_sync: Arc::default(),
            event_sync: Arc::default(),
        });

        // Register cop reference for unique tag.
        PduHandleManager::register_cop(self.api.unique_tag, unique_tag, None, Some(Arc::downgrade(&cop)));

        Ok(cop)
    }

    pub async fn start_com_primitive(
        &self,
        cop_type: &PduCopt,
        data: &[u8],
        params: Option<&PduComPrimitiveParams>
    ) -> WorkerResult<Arc<PduComPrimitive>> {
        match self.worker.get() {
            Some(worker) => {
                let unique_tag: PduUniqueCopTag = random_non_zero_usize();
                let (tx, rx) = mpsc::channel(Self::DEFAULT_COP_EVENT_QUEUE_SIZE);
                let tx = Arc::new(tx);

                // Register event tx for unique tag.
                PduHandleManager::register_cop(self.api.unique_tag, unique_tag, Some(Arc::downgrade(&tx)), None);

                let h_mod = self.get_module_handle();
                let h_cll = self.get_cll_handle();
                let cop_data = worker.pdu_start_com_primitive(
                    h_mod,
                    h_cll,
                    cop_type.to_owned(),
                    data.to_vec(),
                    params.cloned(),
                    Some(unique_tag)
                ).await?;

                let cop = Arc::new_cyclic(|weak| PduComPrimitive {
                    me: weak.clone(),
                    api: self.api.clone(),
                    worker: OnceLock::default(),
                    unique_tag,
                    h_mod,
                    h_cll,
                    cop_data,
                    params: params.cloned().into(),
                    event_tx: tx,
                    event_rx: Arc::new(TokioMutex::new(rx)),
                    dead: Arc::default(),
                    pdu_sync: Arc::default(),
                    event_sync: Arc::default(),
                });

                cop.set_worker(worker.clone());

                // Register cop reference for unique tag.
                PduHandleManager::register_cop(self.api.unique_tag, unique_tag, None, Some(Arc::downgrade(&cop)));

                Ok(cop)
            },
            None => {
                debug!("The use of asynchronous functions is not recommended outside of PduAsyncWorker");
                let me = self.take_me_expect();

                let cop_type = cop_type.to_owned();
                let data = data.to_vec();
                let params = params.cloned();

                let thread = move || {
                    me.blocking_start_com_primitive(
                        &cop_type,
                        &data,
                        params.as_ref()
                    )
                };

                let cop = spawn_blocking(thread)
                    .await
                    .expect("internal error: ComLogicalLink::blocking_start_com_primitive task panicked")?;

                Ok(cop)
            }
        }
    }

    pub fn blocking_start_comm(&self, builder: StartComm) -> ApiResult<Arc<PduComPrimitive>> {
        self.blocking_start_com_primitive(
            &PduCopt::StartComm,
            &builder.data,
            Some(&builder.build())
        )
    }

    pub async fn start_comm(&self, builder: StartComm) -> WorkerResult<Arc<PduComPrimitive>> {
        self.start_com_primitive(
            &PduCopt::StartComm,
            &builder.data,
            Some(&builder.build())
        ).await
    }

    pub fn blocking_stop_comm(&self, builder: StopComm) -> ApiResult<Arc<PduComPrimitive>> {
        self.blocking_start_com_primitive(
            &PduCopt::StopComm,
            &builder.data,
            Some(&builder.build())
        )
    }

    pub async fn stop_comm(&self, builder: StopComm) -> WorkerResult<Arc<PduComPrimitive>> {
        self.start_com_primitive(
            &PduCopt::StopComm,
            &builder.data,
            Some(&builder.build())
        ).await
    }

    pub fn blocking_send_recv(&self, builder: SendRecv) -> ApiResult<Arc<PduComPrimitive>> {
        self.blocking_start_com_primitive(
            &PduCopt::SendRecv,
            &builder.data,
            Some(&builder.build())
        )
    }

    pub async fn send_recv(&self, builder: SendRecv) -> WorkerResult<Arc<PduComPrimitive>> {
        self.start_com_primitive(
            &PduCopt::SendRecv,
            &builder.data,
            Some(&builder.build())
        ).await
    }

    pub fn blocking_update_param(&self) -> ApiResult<Arc<PduComPrimitive>> {
        self.blocking_start_com_primitive(
            &PduCopt::UpdateParam,
            &[],
            None
        )
    }

    pub async fn update_param(&self) -> WorkerResult<Arc<PduComPrimitive>> {
        self.start_com_primitive(
            &PduCopt::UpdateParam,
            &[],
            None
        ).await
    }

    pub fn blocking_restore_param(&self) -> ApiResult<Arc<PduComPrimitive>> {
        self.blocking_start_com_primitive(
            &PduCopt::RestoreParam,
            &[],
            None
        )
    }

    pub async fn restore_param(&self) -> WorkerResult<Arc<PduComPrimitive>> {
        self.start_com_primitive(
            &PduCopt::RestoreParam,
            &[],
            None
        ).await
    }
}

impl Drop for PduComLogicalLink {
    fn drop(&mut self) {
        debug!(
            h_mod = self.get_module_handle(),
            h_cll = self.get_cll_handle(),
            "Disconnecting the PduComLogicalLink via destructor..."
        );

        match self.worker.get() {
            Some(worker) => {
                let query = Query::PduDisconnect(self.get_module_handle(), self.get_cll_handle());
                match worker.request(query, None) {
                    Ok(_) => {},
                    Err(err) => {
                        error!(
                            h_mod = self.get_module_handle(),
                            h_cll = self.get_cll_handle(),
                            "Error when disconnecting the PduComLogicalLink via destructor: {err}"
                        );
                    }
                }
            },
            None => {
                debug!(
                    h_mod = self.get_module_handle(),
                    h_cll = self.get_cll_handle(),
                    "The use of asynchronous functions is not recommended outside of PduAsyncWorker"
                );
                let api = self.api.clone();
                let h_mod = self.get_module_handle();
                let h_cll = self.get_cll_handle();
                spawn(move || api.pdu_disconnect(h_mod, h_cll));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CllStatus(PduStatusData);

impl Deref for CllStatus {
    type Target = PduStatusData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CllStatus {
    pub fn is_offline(&self) -> bool {
        matches!(self.0.status_code, PduStatus::CllstOffline)
    }

    pub fn is_online(&self) -> bool {
        matches!(self.0.status_code, PduStatus::CllstOnline)
    }

    pub fn is_communication_started(&self) -> bool {
        matches!(self.0.status_code, PduStatus::CllstCommStarted)
    }
}

#[derive(Debug, Clone)]
pub struct PduCllData {
    pub(crate) h_mod: PduModuleHandle,

    pub(crate) h_cll: PduCllHandle,

    pub(crate) create_type: CllCreateType,

    pub(crate) create_flags: CllCreateFlags,
}

impl PduCllData {
    pub fn get_module_handle(&self) -> PduModuleHandle {
        self.h_mod.clone()
    }

    pub fn get_cll_handle(&self) -> PduCllHandle {
        self.h_cll.clone()
    }

    pub fn get_create_type(&self) -> &CllCreateType {
        &self.create_type
    }

    pub fn get_create_flags(&self) -> &CllCreateFlags {
        &self.create_flags
    }
}

#[derive(Debug, Clone)]
pub enum CllCreateType {
    /// ComLogicalLink will be created by resource ID.
    ///
    /// Not recommended.
    ResourceId(PduObjectId),

    /// ComLogicalLink will be created by
    ///  - bus type ID
    ///  - protocol ID
    ///  - information about the pins on VCI (can't be empty).
    ///
    /// Recommended.
    ResourceData {
        bus: BusSource,
        protocol: ProtocolSource,
        pins: Vec<TargetPin>,
    },
}

/// Вспомогательная структура, используемая в функции [`PduWrapper::create_com_logical_link()`] при
/// создании "логической связи".
/// См. ISO 22900-2, глава D.2.3, таблица D.6.
#[derive(Debug, Clone)]
pub struct CllCreateFlags {
    /// Byte 0, bit 7.
    ///
    /// Обеспечивает возможность передачи всех принятых сообщений без изменений по каналу связи
    /// (переданных и принятых).
    ///
    /// Эта функция зависит от протокола!
    ///
    /// [FALSE]: указывает, что API D-PDU будет удалять байты заголовка и контрольные суммы перед
    /// возвратом (TxFlag ENABLE_EXTRA_INFO может быть использован для получения дополнительной
    /// информации о заголовке/футере сообщения).
    ///
    /// [TRUE]: указывает, что байты заголовка и контрольные суммы будут оставлены в возвращаемом
    /// элементе результата
    pub raw_mode: bool,

    /// Byte 0, bit 6.
    ///
    /// API D-PDU будет создавать контрольную сумму для передачи сообщений.
    ///
    /// Этот флаг игнорируется, если для [raw_mode] установлено значение [false].
    pub checksum_mode: bool,

    /// Byte 3, bit 0.
    ///
    /// Действительно только для Softing D-PDU-API (не является частью ISO22900-2)!
    ///
    /// Создает "мониторную" связь при вызове PDUCreateComLogicalLink, а не логическую связь.
    ///
    /// Используется только в протоколах:
    ///   - ISO_11898_RAW
    ///   - ISO_14230_3_on_ISO_14230_2
    ///
    /// И только если для [raw_mode] установлено значение false.
    pub monitor_mode: bool,
}

impl Display for CllCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "raw_mode={:?}, checksum_mode={:?}, monitor_mode={:?}",
            self.raw_mode, self.checksum_mode, self.monitor_mode
        )
    }
}

impl CllCreateFlags {
    pub fn new_with_recommended() -> Self {
        // Объяснение почему лучше иметь по умолчанию включенный режим "checksum":
        //
        // Природа D-PDU API заключается в том, чтобы управлять протоколами транспортного средства.
        // Поэтому D-PDU API по своей природе делает контрольную сумму.
        //
        // Но если вы, например, хотите получить больше информации о глубоких слоях,
        // вы можете включить режим RawMode.
        //
        // Многие удивляются, что, например, протоколы K-line больше не работают.
        //
        // Это происходит потому, что режим "Checksum" работает только тогда, когда
        // включён режим "Raw".
        //
        // Так что, если вы хотите включить режим "Raw" только для некоторых протоколов, вам так
        // же нужно включить режим "Checksum", чтобы всё работало, как и раньше.
        //
        // Поэтому лучше, чтобы режим "Checksum" был включён по умолчанию.

        Self {
            raw_mode: false,
            checksum_mode: true,
            monitor_mode: false,
        }
    }

    pub(crate) fn zero_byte(&self) -> u8 {
        let mut b = 0;

        if self.raw_mode {
            b |= 0x8;
        }

        if self.checksum_mode {
            b |= 0x4;
        }

        b
    }

    pub(crate) fn third_byte(&self) -> u8 {
        let mut b = 0;

        if self.monitor_mode {
            b |= 0x01;
        }

        b
    }

    /// Рассчитывает байтовый массив с учётом используемых режимов.
    pub(crate) fn get_pdu_flag_data(&self) -> [u8; 4] {
        [self.zero_byte(), 0, self.third_byte(), 0]
    }
}

mod sealed {
    pub trait Sealed {}
}

trait CopParamsBuilder: sealed::Sealed {
    fn build(&self) -> PduComPrimitiveParams;
}

#[derive(Debug, Default)]
pub struct StartComm {
    pub data: Vec<u8>,

    pub tx_flags: TransmitFlags,

    pub receive_cycles: u32,

    pub param_buffer: ComParamBuffer,

    pub filters: Vec<ExpectedResponse>
}

impl StartComm {
    /// UseCase.
    ///
    /// В основном для работы с CAN. Для первоначального Tester Present.
    pub fn initial() -> Self {
        StartComm::default()
    }

    /// UseCase.
    pub fn initial_with_recv_expected_ids(ids: Vec<u32>) -> Self {
        Self::initial().with_expected_ids(ids)
    }

    /// UseCase.
    pub fn send_with_recv_expected_ids(data: &[u8], ids: Vec<u32>) -> Self {
        Self::initial()
            .with_data(data)
            .with_expected_ids(ids)
    }

    pub fn with_data(mut self, data: &[u8]) -> Self {
        self.data = data.to_vec();
        self
    }

    pub fn with_tx_flags(mut self, flags: TransmitFlags) -> Self {
        self.tx_flags = flags;
        self
    }

    pub fn with_receive_cycles(mut self, cycles: u32) -> Self {
        self.receive_cycles = cycles;
        self
    }

    pub fn with_param_buffer(mut self, param_buffer: ComParamBuffer) -> Self {
        self.param_buffer = self.param_buffer;
        self
    }

    pub fn with_expected_ids(mut self, ids: Vec<u32>) -> Self {
        self.filters = vec![ExpectedResponse {
            response_type: ResponseType::Positive,
            acceptance_id: 0,
            mask_data: Default::default(),
            unique_response_ids: ids.clone(),
        }, ExpectedResponse {
            response_type: ResponseType::Negative,
            acceptance_id: 0,
            mask_data: Default::default(),
            unique_response_ids: ids,
        }];
        self
    }

    pub fn with_filters(mut self, vec: Vec<ExpectedResponse>) -> Self {
        self.filters = vec;
        self
    }
}

impl sealed::Sealed for StartComm {}
impl CopParamsBuilder for StartComm {
    fn build(&self) -> PduComPrimitiveParams {
        let mut params = PduComPrimitiveParams::default();

        params.send_cycles = SendCycles::Normal(if self.data.len() > 0 { 1 } else { 0 });
        params.receive_cycles = ReceiveCycles::Normal(self.receive_cycles);
        params.temp_param_update = self.param_buffer;
        params.tx_flag = self.tx_flags.clone();
        params.expected_responses = self.filters.clone();

        params
    }
}

#[derive(Debug, Default)]
pub struct StopComm {
    pub data: Vec<u8>,

    pub tx_flags: TransmitFlags,

    pub receive: bool,

    pub param_buffer: ComParamBuffer,

    pub filters: Vec<ExpectedResponse>
}

impl StopComm {
    /// Use case.
    pub fn now() -> Self {
        StopComm::default()
    }

    /// Use case.
    pub fn now_with_send(data: &[u8]) -> Self {
        Self::now().with_data(data)
    }

    /// Use case.
    pub fn later_with_send_and_recv_expected_ids(data: &[u8], ids: Vec<u32>) -> Self {
        Self::now()
            .with_data(data)
            .with_expected_ids(ids)
    }

    pub fn with_data(mut self, data: &[u8]) -> Self {
        self.data = data.to_vec();
        self
    }

    pub fn with_tx_flags(mut self, flags: TransmitFlags) -> Self {
        self.tx_flags = flags;
        self
    }

    pub fn with_receive(mut self, status: bool) -> Self {
        self.receive = status;
        self
    }

    pub fn with_param_buffer(mut self, param_buffer: ComParamBuffer) -> Self {
        self.param_buffer = self.param_buffer;
        self
    }

    pub fn with_expected_ids(mut self, ids: Vec<u32>) -> Self {
        self.filters = vec![ExpectedResponse {
            response_type: ResponseType::Positive,
            acceptance_id: 0,
            mask_data: Default::default(),
            unique_response_ids: ids.clone(),
        }, ExpectedResponse {
            response_type: ResponseType::Negative,
            acceptance_id: 0,
            mask_data: Default::default(),
            unique_response_ids: ids,
        }];
        self
    }

    pub fn with_filters(mut self, vec: Vec<ExpectedResponse>) -> Self {
        self.filters = vec;
        self
    }
}

impl sealed::Sealed for StopComm {}
impl CopParamsBuilder for StopComm {
    fn build(&self) -> PduComPrimitiveParams {
        let mut params = PduComPrimitiveParams::default();

        params.send_cycles = SendCycles::Normal(if self.data.len() > 0 { 1 } else { 0 });
        params.receive_cycles = ReceiveCycles::Normal(if self.receive { 1 } else { 0 });
        params.temp_param_update = self.param_buffer;
        params.tx_flag = self.tx_flags.clone();
        params.expected_responses = self.filters.clone();

        params
    }
}

#[derive(Debug)]
pub struct SendRecv {
    pub data: Vec<u8>,

    pub tx_flags: TransmitFlags,

    pub send_cycles: SendCycles,

    pub receive_cycles: ReceiveCycles,

    pub param_buffer: ComParamBuffer,

    pub filters: Vec<ExpectedResponse>,

    pub delay: Duration
}

impl SendRecv {
    /// Use case.
    pub fn new(data: &[u8]) -> Self {
        SendRecv {
            data: data.to_vec(),
            tx_flags: TransmitFlags::default(),
            send_cycles: SendCycles::Normal(1),
            receive_cycles: ReceiveCycles::Normal(1),
            param_buffer: ComParamBuffer::default(),
            filters: Vec::default(),
            delay: Duration::from_millis(0),
        }
    }

    pub fn with_tx_flags(mut self, flags: TransmitFlags) -> Self {
        self.tx_flags = flags;
        self
    }

    pub fn with_send_cycles(mut self, cycles: SendCycles) -> Self {
        self.send_cycles = cycles;
        self
    }

    pub fn with_receive_cycles(mut self, cycles: ReceiveCycles) -> Self {
        if cycles.to_i32() == 0 {
            panic!("internal error: when PduCopt = SendRecv, receive cycles must not be zero");
        }
        self.receive_cycles = cycles;
        self
    }

    pub fn with_param_buffer(mut self, param_buffer: ComParamBuffer) -> Self {
        self.param_buffer = self.param_buffer;
        self
    }

    pub fn with_expected_ids(mut self, ids: Vec<u32>) -> Self {
        self.filters = vec![ExpectedResponse {
            response_type: ResponseType::Positive,
            acceptance_id: 0,
            mask_data: Default::default(),
            unique_response_ids: ids.clone(),
        }, ExpectedResponse {
            response_type: ResponseType::Negative,
            acceptance_id: 0,
            mask_data: Default::default(),
            unique_response_ids: ids,
        }];
        self
    }

    pub fn with_filters(mut self, vec: Vec<ExpectedResponse>) -> Self {
        self.filters = vec;
        self
    }

    pub fn with_delay(mut self, duration: Duration) -> Self {
        self.delay = duration;
        self
    }
}

impl sealed::Sealed for SendRecv {}
impl CopParamsBuilder for SendRecv {
    fn build(&self) -> PduComPrimitiveParams {
        let mut params = PduComPrimitiveParams::default();

        if self.send_cycles.to_i32() == 0 {
            panic!("internal error: when PduCopt = SendRecv, send cycles must not be zero");
        } else if self.receive_cycles.to_i32() == 0 {
            panic!("internal error: when PduCopt = SendRecv, receive cycles must not be zero");
        }

        params.send_cycles = self.send_cycles.clone();
        params.receive_cycles = self.receive_cycles.clone();
        params.temp_param_update = self.param_buffer;
        params.tx_flag = self.tx_flags.clone();
        params.expected_responses = self.filters.clone();
        params.time = self.delay.as_millis() as _;

        params
    }
}