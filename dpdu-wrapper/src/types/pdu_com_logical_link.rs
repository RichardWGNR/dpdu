use crate::api::{ApiError, ApiResult, PduApi};
use crate::constants::COP_EVENTS_QUEUE_SIZE;
use crate::error::{GeneralError, GeneralResult};
use crate::handle_manager::PduHandleManager;
use crate::types::pdu_com_param::table::{IntoPduComParam, MapTarget, PduComParamTable, SetTarget};
use crate::types::pdu_com_primitive::{
    ComParamBuffer, ExpectedResponse, PduPrimitive, PduPrimitiveParams, ReceiveCycles,
    ResponseType, SendCycles, TransmitFlags,
};
use crate::types::pdu_event::PduEvent;
use crate::types::pdu_resource::{BusSource, ProtocolSource, TargetPin};
use crate::types::pdu_status::{PduStatusData, PduStatusTarget};
use crate::types::{PduCllHandle, PduModuleHandle, PduObjectId, PduUniqueCllTag, PduUniqueCopTag};
use crate::utils::random_non_zero_usize;
use crate::worker::{PduAsyncWorker, Query};
use dpdu_api_types::{PduCopt, PduError, PduStatus};
use parking_lot::Mutex as ParkingLotMutex;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::{Arc, OnceLock, Weak};
use std::thread::spawn;
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
use tracing::{debug, error};

#[derive(Debug, Clone)]
pub struct PduLogicalLink {
    pub(crate) me: Weak<PduLogicalLink>,

    pub(crate) api: Arc<PduApi>,

    pub(crate) worker: OnceLock<Arc<PduAsyncWorker>>,

    pub(crate) unique_tag: PduUniqueCllTag,

    pub(crate) cll_data: Arc<PduCllData>,

    pub(crate) event_tx: Arc<mpsc::Sender<PduEvent>>,

    pub(crate) event_rx: Arc<mpsc::Receiver<PduEvent>>,

    pub(crate) sync: Arc<ParkingLotMutex<()>>,
}

impl PduLogicalLink {
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

    fn take_me_expect(&self) -> Arc<PduLogicalLink> {
        self.me
            .upgrade()
            .expect("internal error: PduLogicalLink self-reference is no longer valid")
    }

    pub fn blocking_get_status(&self) -> ApiResult<CllStatus> {
        let _sync_guard = self.sync.lock();
        let target = PduStatusTarget::LogicalLink(self.get_module_handle(), self.get_cll_handle());
        let result = self.api.pdu_get_status(&target)?;
        Ok(CllStatus(result))
    }

    pub async fn get_status(&self) -> GeneralResult<CllStatus> {
        match self.worker.get() {
            Some(worker) => {
                let target =
                    PduStatusTarget::LogicalLink(self.get_module_handle(), self.get_cll_handle());
                let result = worker.pdu_get_status(target).await?;
                Ok(CllStatus(result))
            }
            None => {
                let me = self.take_me_expect();
                let result = spawn_blocking(move || me.blocking_get_status())
                    .await
                    .expect(
                        "internal error: PduLogicalLink::blocking_get_status() task panicked",
                    )?;

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
        self.api
            .pdu_connect(self.get_module_handle(), self.get_cll_handle())?;
        Ok(true)
    }

    pub async fn connect(&self) -> GeneralResult<bool> {
        match self.worker.get() {
            Some(worker) => {
                let status = self.get_status().await?;
                if !status.is_offline() {
                    return Ok(false);
                }
                worker
                    .pdu_connect(self.get_module_handle(), self.get_cll_handle())
                    .await?;
                Ok(true)
            }
            None => {
                let me = self.take_me_expect();
                let result = spawn_blocking(move || me.blocking_connect())
                    .await
                    .expect("internal error: PduLogicalLink::blocking_connect() task panicked")?;

                Ok(result)
            }
        }
    }

    pub fn blocking_disconnect(&self) -> GeneralResult<bool> {
        let status = self.blocking_get_status()?;
        if status.is_offline() {
            return Ok(false);
        }

        let _sync_guard = self.sync.lock();
        self.api
            .pdu_disconnect(self.get_module_handle(), self.get_cll_handle())?;
        Ok(true)
    }

    pub async fn disconnect(&self) -> GeneralResult<bool> {
        match self.worker.get() {
            Some(worker) => {
                let status = self.get_status().await?;
                if status.is_offline() {
                    return Ok(false);
                }
                worker
                    .pdu_disconnect(self.get_module_handle(), self.get_cll_handle())
                    .await?;
                Ok(true)
            }
            None => {
                let me = self.take_me_expect();
                let result = spawn_blocking(move || me.blocking_connect()).await.expect(
                    "internal error: PduLogicalLink::blocking_disconnect() task panicked",
                )?;

                Ok(result)
            }
        }
    }

    pub fn blocking_start_primitive(
        &self,
        cop_type: &PduCopt,
        data: &[u8],
        params: Option<&PduPrimitiveParams>,
        events_queue_size: Option<usize>,
    ) -> GeneralResult<Arc<PduPrimitive>> {
        let _sync_guard = self.sync.lock();

        let events_queue_size = events_queue_size.unwrap_or(COP_EVENTS_QUEUE_SIZE);
        let unique_tag: PduUniqueCopTag = random_non_zero_usize();
        let (tx, rx) = mpsc::channel(events_queue_size);
        let tx = Arc::new(tx);

        // Register event tx for unique tag.
        PduHandleManager::register_cop(
            self.api.unique_tag,
            unique_tag,
            Some(Arc::downgrade(&tx)),
            None,
        );

        let h_mod = self.get_module_handle();
        let h_cll = self.get_cll_handle();
        let cop_data = self.api.pdu_start_com_primitive(
            h_mod,
            h_cll,
            cop_type.to_owned(),
            data,
            params,
            Some(unique_tag),
        )?;

        let cop = Arc::new_cyclic(|weak| PduPrimitive {
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
        PduHandleManager::register_cop(
            self.api.unique_tag,
            unique_tag,
            None,
            Some(Arc::downgrade(&cop)),
        );

        Ok(cop)
    }

    pub async fn start_primitive(
        &self,
        cop_type: &PduCopt,
        data: &[u8],
        params: Option<&PduPrimitiveParams>,
        events_queue_size: Option<usize>,
    ) -> GeneralResult<Arc<PduPrimitive>> {
        let events_queue_size = events_queue_size.unwrap_or(COP_EVENTS_QUEUE_SIZE);
        match self.worker.get() {
            Some(worker) => {
                let unique_tag: PduUniqueCopTag = random_non_zero_usize();
                let (tx, rx) = mpsc::channel(events_queue_size);
                let tx = Arc::new(tx);

                // Register event tx for unique tag.
                PduHandleManager::register_cop(
                    self.api.unique_tag,
                    unique_tag,
                    Some(Arc::downgrade(&tx)),
                    None,
                );

                let h_mod = self.get_module_handle();
                let h_cll = self.get_cll_handle();
                let cop_data = worker
                    .pdu_start_com_primitive(
                        h_mod,
                        h_cll,
                        cop_type.to_owned(),
                        data.to_vec(),
                        params.cloned(),
                        Some(unique_tag),
                    )
                    .await?;

                let cop = Arc::new_cyclic(|weak| PduPrimitive {
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
                PduHandleManager::register_cop(
                    self.api.unique_tag,
                    unique_tag,
                    None,
                    Some(Arc::downgrade(&cop)),
                );

                Ok(cop)
            }
            None => {
                let me = self.take_me_expect();

                let cop_type = cop_type.to_owned();
                let data = data.to_vec();
                let params = params.cloned();

                let thread = move || {
                    me.blocking_start_primitive(
                        &cop_type,
                        &data,
                        params.as_ref(),
                        Some(events_queue_size),
                    )
                };

                let cop = spawn_blocking(thread).await.expect(
                    "internal error: ComLogicalLink::blocking_start_com_primitive() task panicked",
                )?;

                Ok(cop)
            }
        }
    }

    pub fn blocking_start_comm(&self, builder: StartComm) -> GeneralResult<Arc<PduPrimitive>> {
        self.blocking_start_primitive(
            &PduCopt::StartComm,
            &builder.data,
            Some(&builder.build()),
            builder.events_queue_size,
        )
    }

    pub async fn start_comm(&self, builder: StartComm) -> GeneralResult<Arc<PduPrimitive>> {
        self.start_primitive(
            &PduCopt::StartComm,
            &builder.data,
            Some(&builder.build()),
            builder.events_queue_size,
        )
        .await
    }

    pub fn blocking_stop_comm(&self, builder: StopComm) -> GeneralResult<Arc<PduPrimitive>> {
        self.blocking_start_primitive(
            &PduCopt::StopComm,
            &builder.data,
            Some(&builder.build()),
            builder.events_queue_size,
        )
    }

    pub async fn stop_comm(&self, builder: StopComm) -> GeneralResult<Arc<PduPrimitive>> {
        self.start_primitive(
            &PduCopt::StopComm,
            &builder.data,
            Some(&builder.build()),
            builder.events_queue_size,
        )
        .await
    }

    pub fn blocking_send_recv(&self, builder: SendRecv) -> GeneralResult<Arc<PduPrimitive>> {
        self.blocking_start_primitive(
            &PduCopt::SendRecv,
            &builder.data,
            Some(&builder.build()),
            builder.events_queue_size,
        )
    }

    pub async fn send_recv(&self, builder: SendRecv) -> GeneralResult<Arc<PduPrimitive>> {
        self.start_primitive(
            &PduCopt::SendRecv,
            &builder.data,
            Some(&builder.build()),
            builder.events_queue_size,
        )
        .await
    }

    pub fn blocking_update_param(&self) -> GeneralResult<Arc<PduPrimitive>> {
        self.blocking_start_primitive(&PduCopt::UpdateParam, &[], None, None)
    }

    pub async fn update_param(&self) -> GeneralResult<Arc<PduPrimitive>> {
        self.start_primitive(&PduCopt::UpdateParam, &[], None, None)
            .await
    }

    pub fn blocking_restore_param(&self) -> GeneralResult<Arc<PduPrimitive>> {
        self.blocking_start_primitive(&PduCopt::RestoreParam, &[], None, None)
    }

    pub async fn restore_param(&self) -> GeneralResult<Arc<PduPrimitive>> {
        self.start_primitive(&PduCopt::RestoreParam, &[], None, None)
            .await
    }

    pub fn blocking_set_com_params(&self, set_target: impl Into<SetTarget>) -> GeneralResult<()> {
        let h_mod = self.get_module_handle();
        let h_cll = self.get_cll_handle();

        match set_target.into() {
            SetTarget::Definitions(v) => {
                for def in v.iter() {
                    let cp = match def.blocking_build(&self.api) {
                        Ok(v) => v,
                        Err(GeneralError::ApiError(ApiError::PduError(
                            PduError::ComParamNotSupported,
                        ))) => {
                            continue;
                        }
                        Err(err) => {
                            return Err(err)?;
                        }
                    };
                    match self.api.pdu_set_com_param(h_mod, h_cll, &cp) {
                        Ok(()) => {}
                        Err(ApiError::PduError(PduError::ComParamNotSupported)) => {
                            continue;
                        }
                        Err(err) => Err(err)?,
                    }
                }
            }
            SetTarget::ComParams(v) => {
                for cp in v.iter() {
                    match self.api.pdu_set_com_param(h_mod, h_cll, cp) {
                        Ok(()) => {}
                        Err(ApiError::PduError(PduError::ComParamNotSupported)) => {}
                        Err(err) => Err(err)?,
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn set_com_params(&self, set_target: impl Into<SetTarget>) -> GeneralResult<()> {
        let h_mod = self.get_module_handle();
        let h_cll = self.get_cll_handle();

        match self.worker.get() {
            Some(worker) => match set_target.into() {
                SetTarget::Definitions(v) => {
                    for def in v.iter() {
                        let cp = match def.build(worker.as_ref()).await {
                            Ok(v) => v,
                            Err(GeneralError::ApiError(ApiError::PduError(
                                PduError::ComParamNotSupported,
                            ))) => {
                                continue;
                            }
                            Err(err) => Err(err)?,
                        };

                        match worker.pdu_set_com_param(h_mod, h_cll, cp).await {
                            Ok(()) => {}
                            Err(GeneralError::ApiError(ApiError::PduError(
                                PduError::ComParamNotSupported,
                            ))) => {
                                continue;
                            }
                            Err(err) => Err(err)?,
                        }
                    }
                }
                SetTarget::ComParams(v) => {
                    for cp in v.iter() {
                        match worker.pdu_set_com_param(h_mod, h_cll, cp.clone()).await {
                            Ok(()) => {}
                            Err(GeneralError::ApiError(ApiError::PduError(
                                PduError::ComParamNotSupported,
                            ))) => {}
                            Err(err) => Err(err)?,
                        }
                    }
                }
            },
            None => {
                let me = self.take_me_expect();

                let set_target = set_target.into();
                let thread = move || me.blocking_set_com_params(set_target);

                spawn_blocking(thread).await.expect(
                    "internal error: ComLogicalLink::blocking_set_com_params() task panicked",
                )?;
            }
        }

        Ok(())
    }

    pub fn blocking_set_unique_com_params_table(
        &self,
        map_target: impl Into<MapTarget>,
    ) -> GeneralResult<()> {
        let h_mod = self.get_module_handle();
        let h_cll = self.get_cll_handle();

        match map_target.into() {
            MapTarget::Definitions(v) => {
                let mut table = PduComParamTable::with_capacity(v.len());

                for (unique_id, set) in v.iter() {
                    for def in set.iter() {
                        let cp = def.blocking_build(&self.api)?;
                        table = table.add(unique_id.to_owned(), cp);
                    }
                }

                self.api
                    .pdu_set_unique_resp_id_table(h_mod, h_cll, &table)?;
            }
            MapTarget::ComParams(v) => {
                self.api.pdu_set_unique_resp_id_table(h_mod, h_cll, &v)?;
            }
        }

        Ok(())
    }

    pub async fn set_unique_com_params_table(
        &self,
        map_target: impl Into<MapTarget>,
    ) -> GeneralResult<()> {
        let h_mod = self.get_module_handle();
        let h_cll = self.get_cll_handle();

        match self.worker.get() {
            Some(worker) => match map_target.into() {
                MapTarget::Definitions(v) => {
                    let mut table = PduComParamTable::with_capacity(v.len());

                    for (unique_id, set) in v.iter() {
                        for def in set.iter() {
                            let cp = def.build(worker.as_ref()).await?;
                            table = table.add(unique_id.to_owned(), cp);
                        }
                    }

                    worker
                        .pdu_set_unique_resp_id_table(h_mod, h_cll, table)
                        .await?;
                }
                MapTarget::ComParams(v) => {
                    worker.pdu_set_unique_resp_id_table(h_mod, h_cll, v).await?;
                }
            },
            None => {
                let me = self.take_me_expect();

                let map_target = map_target.into();
                let thread = move || me.blocking_set_unique_com_params_table(map_target);

                spawn_blocking(thread)
                    .await
                    .expect("internal error: ComLogicalLink::blocking_set_unique_com_params_table() task panicked")?;
            }
        }

        Ok(())
    }
}

impl Drop for PduLogicalLink {
    fn drop(&mut self) {
        debug!(
            h_mod = self.get_module_handle(),
            h_cll = self.get_cll_handle(),
            "Disconnecting the PduComLogicalLink via destructor..."
        );

        match self.worker.get() {
            Some(worker) => {
                let query = Query::VtCllDestructor(self.get_module_handle(), self.get_cll_handle());
                match worker.request(query, None) {
                    Ok(_) => {}
                    Err(err) => {
                        error!(
                            h_mod = self.get_module_handle(),
                            h_cll = self.get_cll_handle(),
                            "Error when disconnecting the PduComLogicalLink via destructor: {err}"
                        );
                    }
                }
            }
            None => {
                let api = self.api.clone();
                let h_mod = self.get_module_handle();
                let h_cll = self.get_cll_handle();
                spawn(move || api.vt_cll_destructor(h_mod, h_cll));
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

impl CllCreateType {
    pub fn raw_dw_can_on_obd() -> Self {
        CllCreateType::ResourceData {
            bus: BusSource::dual_wire_can(),
            protocol: ProtocolSource::iso_11898_raw(),
            pins: TargetPin::obd_dual_wire_can(),
        }
    }

    pub fn uds_on_iso_tp_on_dw_can() -> Self {
        CllCreateType::ResourceData {
            bus: BusSource::dual_wire_can(),
            protocol: ProtocolSource::uds_on_iso_tp(),
            pins: TargetPin::obd_dual_wire_can(),
        }
    }

    pub fn kwp_on_iso_tp_on_dw_can() -> Self {
        CllCreateType::ResourceData {
            bus: BusSource::dual_wire_can(),
            protocol: ProtocolSource::kwp_on_iso_tp(),
            pins: TargetPin::obd_dual_wire_can(),
        }
    }
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
    pub fn raw() -> Self {
        Self {
            raw_mode: true,
            monitor_mode: false,
            checksum_mode: false,
        }
    }

    pub fn raw_with_monitor() -> Self {
        Self {
            raw_mode: true,
            monitor_mode: true,
            checksum_mode: false,
        }
    }

    pub fn recommended() -> Self {
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

    pub(crate) fn zb(&self) -> u8 {
        let mut b = 0;

        // Chapter D.2.3.

        // byte pos 0, bit pos 7: RawMode.
        if self.raw_mode {
            b |= 0x80; // 0 - OFF; 1 - ON
        }

        // bye pos 0, bit pos 6: ChecksumMode
        if self.checksum_mode {
            b |= 0x40; // 0 - OFF; 1 - ON
        }

        b
    }

    pub(crate) fn sb(&self) -> u8 {
        let mut b = 0;

        if self.monitor_mode {
            b |= 0x01;
        }

        b
    }

    /// Рассчитывает байтовый массив с учётом используемых режимов.
    pub(crate) fn get_pdu_flag_data(&self) -> [u8; 4] {
        [self.zb(), 0, self.sb(), 0]
    }
}

mod sealed {
    pub trait Sealed {}
}

trait CopParamsBuilder: sealed::Sealed {
    fn build(&self) -> PduPrimitiveParams;
}

#[derive(Debug, Default)]
pub struct StartComm {
    pub data: Vec<u8>,

    pub tx_flags: TransmitFlags,

    pub receive_cycles: ReceiveCycles,

    pub param_buffer: ComParamBuffer,

    pub filters: Vec<ExpectedResponse>,

    pub events_queue_size: Option<usize>,
}

impl StartComm {
    /// Use case.
    ///
    /// В основном для работы с CAN. Для первоначального Tester Present.
    pub fn initial() -> Self {
        StartComm::default()
    }

    /// Use case.
    pub fn monitor() -> Self {
        Self::initial().with_receive_cycles(ReceiveCycles::Infinite)
    }

    /// Use case.
    pub fn initial_with_recv_expected_ids(ids: Vec<u32>) -> Self {
        Self::initial().with_expected_ids(ids)
    }

    /// Use case.
    pub fn send_with_recv_expected_ids(data: &[u8], ids: Vec<u32>) -> Self {
        Self::initial().with_data(data).with_expected_ids(ids)
    }

    pub fn with_events_queue_size(mut self, size: usize) -> Self {
        self.events_queue_size = Some(size);
        self
    }

    pub fn with_data(mut self, data: &[u8]) -> Self {
        self.data = data.to_vec();
        self
    }

    pub fn with_tx_flags(mut self, flags: TransmitFlags) -> Self {
        self.tx_flags = flags;
        self
    }

    pub fn with_receive_cycles(mut self, cycles: ReceiveCycles) -> Self {
        self.receive_cycles = cycles;
        self
    }

    pub fn with_param_buffer(mut self, param_buffer: ComParamBuffer) -> Self {
        self.param_buffer = param_buffer;
        self
    }

    pub fn with_expected_ids(mut self, ids: Vec<u32>) -> Self {
        self.filters = vec![
            ExpectedResponse {
                response_type: ResponseType::Positive,
                acceptance_id: 0,
                mask_data: Default::default(),
                unique_response_ids: ids.clone(),
            },
            ExpectedResponse {
                response_type: ResponseType::Negative,
                acceptance_id: 0,
                mask_data: Default::default(),
                unique_response_ids: ids,
            },
        ];
        self
    }

    pub fn with_filters(mut self, vec: Vec<ExpectedResponse>) -> Self {
        self.filters = vec;
        self
    }
}

impl sealed::Sealed for StartComm {}
impl CopParamsBuilder for StartComm {
    fn build(&self) -> PduPrimitiveParams {
        let mut params = PduPrimitiveParams::default();

        params.send_cycles = SendCycles::Normal(if self.data.len() > 0 { 1 } else { 0 });
        params.receive_cycles = self.receive_cycles.clone();
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

    pub filters: Vec<ExpectedResponse>,

    pub events_queue_size: Option<usize>,
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
        Self::now().with_data(data).with_expected_ids(ids)
    }

    pub fn with_events_queue_size(mut self, size: usize) -> Self {
        self.events_queue_size = Some(size);
        self
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
        self.param_buffer = param_buffer;
        self
    }

    pub fn with_expected_ids(mut self, ids: Vec<u32>) -> Self {
        self.filters = vec![
            ExpectedResponse {
                response_type: ResponseType::Positive,
                acceptance_id: 0,
                mask_data: Default::default(),
                unique_response_ids: ids.clone(),
            },
            ExpectedResponse {
                response_type: ResponseType::Negative,
                acceptance_id: 0,
                mask_data: Default::default(),
                unique_response_ids: ids,
            },
        ];
        self
    }

    pub fn with_filters(mut self, vec: Vec<ExpectedResponse>) -> Self {
        self.filters = vec;
        self
    }
}

impl sealed::Sealed for StopComm {}
impl CopParamsBuilder for StopComm {
    fn build(&self) -> PduPrimitiveParams {
        let mut params = PduPrimitiveParams::default();

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

    pub delay: Duration,

    pub events_queue_size: Option<usize>,
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
            events_queue_size: None,
        }
    }

    pub fn send_only(data: &[u8]) -> Self {
        SendRecv::new(data).with_receive_cycles(ReceiveCycles::Normal(0))
    }

    /// Use case.
    pub fn monitor() -> Self {
        Self::new(&[]).with_receive_cycles(ReceiveCycles::Infinite)
    }

    pub fn with_events_queue_size(mut self, size: usize) -> Self {
        self.events_queue_size = Some(size);
        self
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
        //if cycles.to_i32() == 0 {
        //    panic!("internal error: when PduCopt = SendRecv, receive cycles must not be zero");
        //}
        self.receive_cycles = cycles;
        self
    }

    pub fn with_param_buffer(mut self, param_buffer: ComParamBuffer) -> Self {
        self.param_buffer = param_buffer;
        self
    }

    pub fn with_expected_ids(mut self, ids: Vec<u32>) -> Self {
        self.filters = vec![
            ExpectedResponse {
                response_type: ResponseType::Positive,
                acceptance_id: 0,
                mask_data: Default::default(),
                unique_response_ids: ids.clone(),
            },
            ExpectedResponse {
                response_type: ResponseType::Negative,
                acceptance_id: 0,
                mask_data: Default::default(),
                unique_response_ids: ids,
            },
        ];
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
    fn build(&self) -> PduPrimitiveParams {
        let mut params = PduPrimitiveParams::default();

        if self.send_cycles.to_i32() == 0 {
            panic!("internal error: when PduCopt = SendRecv, send cycles must not be zero");
        } // else if self.receive_cycles.to_i32() == 0 {
        //    panic!("internal error: when PduCopt = SendRecv, receive cycles must not be zero");
        //}

        params.send_cycles = self.send_cycles.clone();
        params.receive_cycles = self.receive_cycles.clone();
        params.temp_param_update = self.param_buffer;
        params.tx_flag = self.tx_flags.clone();
        params.expected_responses = self.filters.clone();
        params.time = self.delay.as_millis() as _;

        params
    }
}
