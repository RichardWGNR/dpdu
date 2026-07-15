use crate::api::{ApiResult, PduApi};
use crate::event_callback::event_callback;
use crate::handle_manager::PduHandleManager;
use crate::types::pdu_com_logical_link::{CllCreateFlags, CllCreateType, PduLogicalLink};
use crate::types::pdu_event::PduEventTarget;
use crate::types::pdu_module::PduModuleData;
use crate::types::pdu_status::{PduStatusData, PduStatusTarget};
use crate::types::{PduModuleHandle, PduUniqueCllTag};
use crate::utils::random_non_zero_usize;
use crate::worker::{PduAsyncWorker, Query, WorkerResult};
use dpdu_api_types::PduStatus;
use parking_lot::Mutex;
use rand::random;
use regex::Regex;
use std::collections::HashMap;
use std::ffi::CString;
use std::ops::Deref;
use std::sync::{Arc, LazyLock, OnceLock, Weak};
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
use tracing::{debug, error};

pub type VciList = Vec<Arc<PduVci>>;

#[derive(Debug, Clone)]
pub struct PduVci {
    pub(crate) me: Weak<PduVci>,

    pub(crate) api: Arc<PduApi>,

    pub(crate) worker: OnceLock<Arc<PduAsyncWorker>>,

    pub(crate) module_data: Arc<PduModuleData>,

    pub(crate) sync: Arc<Mutex<()>>,
    //pub(crate) clls_tags: Arc<Vec<CString>>,

    //pub(crate) clls: Arc<Mutex<HashMap<String, Arc<ComLogicalLink>>>>
}

impl PduVci {
    const DEFAULT_CLL_EVENT_QUEUE_SIZE: usize = 4096;

    pub(crate) fn set_worker(&self, worker: Arc<PduAsyncWorker>) {
        let _ = self.worker.set(worker);
    }

    pub fn get_module_handle(&self) -> PduModuleHandle {
        self.module_data.h_mod
    }

    pub fn get_name(&self) -> Option<&String> {
        self.module_data.vendor_module_name.as_ref()
    }

    pub fn get_additional_info(&self) -> Option<&String> {
        self.module_data.vendor_module_name.as_ref()
    }

    pub fn get_normalized_name(&self) -> Option<String> {
        static EDIC_RGX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"(?U)ModuleName='(?<name>.+)'"#).unwrap());
        static ACTIA_RGX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"(?U)MVCIFriendlyName='(?<name>.+)'"#).unwrap());

        let module_name = self
            .module_data
            .vendor_module_name
            .clone()
            .unwrap_or_else(|| "VCI".to_string());

        let normalize = |name: &str| {
            if !name.is_empty() && name.chars().all(|c| c.is_ascii_digit()) {
                format!("VCI S/N: {name}")
            } else {
                name.to_owned()
            }
        };

        for regex in [&*EDIC_RGX, &*ACTIA_RGX] {
            if let Some(caps) = regex.captures(&module_name) {
                return Some(normalize(caps.name("name").unwrap().as_str()));
            }
        }

        Some(normalize(&module_name))
    }

    fn take_me_expect(&self) -> Arc<PduVci> {
        self.me
            .upgrade()
            .expect("internal error: Vci self-reference is no longer valid")
    }

    pub fn blocking_get_status(&self) -> ApiResult<VciStatus> {
        let _sync_guard = self.sync.lock();
        let target = PduStatusTarget::Module(self.module_data.h_mod);
        let result = self.api.pdu_get_status(&target)?;
        Ok(VciStatus(result))
    }
    
    pub async fn get_status(&self) -> WorkerResult<VciStatus> {
        match self.worker.get() {
            Some(worker) => {
                let target = PduStatusTarget::Module(self.module_data.h_mod);
                let result = worker.pdu_get_status(target).await?;
                Ok(VciStatus(result))
            }
            None => {
                debug!(
                    h_mod = self.module_data.h_mod,
                    "The use of asynchronous functions is not recommended outside of PduAsyncWorker"
                );
                let me = self.take_me_expect();
                let result = spawn_blocking(move || me.blocking_get_status())
                    .await
                    .expect("internal error: Vci::blocking_get_status() task panicked")?;
                Ok(result)
            }
        }
    }

    pub fn blocking_connect(&self) -> ApiResult<bool> {
        let status = self.blocking_get_status()?;
        if !status.is_available_for_connection() {
            return Ok(false);
        }

        let _sync_guard = self.sync.lock();
        self.api.pdu_module_connect(self.module_data.h_mod)?;
        Ok(true)
    }

    pub async fn connect(&self) -> WorkerResult<bool> {
        match self.worker.get() {
            Some(worker) => {
                let status = self.get_status().await?;
                if !status.is_available_for_connection() {
                    return Ok(false);
                }
                worker.pdu_module_connect(self.module_data.h_mod).await?;
                Ok(true)
            }
            None => {
                debug!(
                    h_mod = self.module_data.h_mod,
                    "The use of asynchronous functions is not recommended outside of PduAsyncWorker"
                );
                let me = self.take_me_expect();
                let result = spawn_blocking(move || me.blocking_connect())
                    .await
                    .expect("internal error: Vci::blocking_connect() task panicked")?;
                Ok(result)
            }
        }
    }

    pub fn blocking_disconnect(&self) -> ApiResult<bool> {
        let status = self.blocking_get_status()?;
        if !status.is_connected() {
            return Ok(false);
        }

        let _sync_guard = self.sync.lock();
        self.api
            .pdu_module_disconnect(Some(self.module_data.h_mod))?;
        Ok(true)
    }

    pub async fn disconnect(&self) -> WorkerResult<bool> {
        match self.worker.get() {
            Some(worker) => {
                let status = self.get_status().await?;
                if !status.is_connected() {
                    return Ok(false);
                }
                worker
                    .pdu_module_disconnect(Some(self.module_data.h_mod))
                    .await?;
                Ok(true)
            }
            None => {
                debug!(
                    h_mod = self.module_data.h_mod,
                    "The use of asynchronous functions is not recommended outside of PduAsyncWorker"
                );
                let me = self.take_me_expect();
                let result = spawn_blocking(move || me.blocking_disconnect())
                    .await
                    .expect("internal error: Vci::blocking_disconnect() task panicked")?;
                Ok(result)
            }
        }
    }

    pub fn blocking_list(api: &Arc<PduApi>) -> ApiResult<VciList> {
        let modules = api.pdu_get_module_ids().inspect_err(|err| {
            error!("Failed to retrieve the list of communication modules: {err}");
        })?;

        let mut list = Vec::with_capacity(modules.len());

        for module in modules.iter() {
            /*
            let Ok(_) = api.pdu_register_event_callback(
                &PduEventTarget::Module(module.h_mod),
                Some(event_callback),
            ) else {
                continue;
            };*/

            list.push(Arc::new_cyclic(|weak| PduVci {
                me: weak.clone(),
                api: api.clone(),
                worker: OnceLock::default(),
                module_data: Arc::new(module.clone()),
                sync: Arc::default(),
            }));
        }

        Ok(list)
    }

    pub async fn list<'a>(resolver: ListResolver<'a>) -> WorkerResult<VciList> {
        match resolver {
            ListResolver::Api(api) => {
                debug!(
                    "The use of asynchronous functions is not recommended outside of PduAsyncWorker"
                );
                let api = api.clone();
                let result = spawn_blocking(move || PduVci::blocking_list(&api))
                    .await
                    .expect("internal error: Vci::blocking_list() task panicked");
                Ok(result?)
            }
            ListResolver::Worker(worker) => Ok(worker
                .get_vci_list()
                .await?
                .into_iter()
                .map(|vci| {
                    vci.set_worker(worker.clone());
                    vci
                })
                .collect::<Vec<_>>()),
        }
    }

    pub fn blocking_create_logical_link(
        &self,
        create_type: &CllCreateType,
        create_flags: &CllCreateFlags,
    ) -> ApiResult<Arc<PduLogicalLink>> {
        let _sync_guard = self.sync.lock();

        let unique_tag: PduUniqueCllTag = random_non_zero_usize();
        let (tx, rx) = mpsc::channel(Self::DEFAULT_CLL_EVENT_QUEUE_SIZE);
        let tx = Arc::new(tx);

        // Register event tx for unique tag.
        PduHandleManager::register_cll(
            self.api.unique_tag,
            unique_tag,
            Some(Arc::downgrade(&tx)),
            None,
        );

        let cll_data = self.api.pdu_create_com_logical_link(
            self.get_module_handle(),
            create_type,
            create_flags,
            Some(unique_tag),
        )?;
        
        let event_target = PduEventTarget::LogicalLink(self.get_module_handle(), cll_data.h_cll);
        let register_result = self
            .api
            .pdu_register_event_callback(&event_target, Some(event_callback));

        if let Err(err) = register_result {
            let _ = self
                .api
                .pdu_destroy_com_logical_link(self.get_module_handle(), cll_data.h_cll);
            return Err(err)?;
        }

        let cll = Arc::new_cyclic(|weak| PduLogicalLink {
            me: weak.clone(),
            api: self.api.clone(),
            worker: OnceLock::default(),
            unique_tag,
            cll_data: cll_data.into(),
            event_tx: tx,
            event_rx: Arc::new(rx),
            sync: Arc::default(),
        });

        // Register cll reference for unique tag.
        PduHandleManager::register_cll(
            self.api.unique_tag,
            unique_tag,
            None,
            Some(Arc::downgrade(&cll)),
        );

        Ok(cll)
    }

    pub async fn create_logical_link(
        &self,
        create_type: &CllCreateType,
        create_flags: &CllCreateFlags,
    ) -> WorkerResult<Arc<PduLogicalLink>> {
        match self.worker.get() {
            Some(worker) => {
                let unique_tag: PduUniqueCllTag = random_non_zero_usize();
                let (tx, rx) = mpsc::channel(Self::DEFAULT_CLL_EVENT_QUEUE_SIZE);
                let tx = Arc::new(tx);

                // Register event tx for unique tag.
                PduHandleManager::register_cll(
                    self.api.unique_tag,
                    unique_tag,
                    Some(Arc::downgrade(&tx)),
                    None,
                );

                let cll_data = worker
                    .pdu_create_com_logical_link(
                        self.get_module_handle(),
                        create_type.to_owned(),
                        create_flags.to_owned(),
                        Some(unique_tag),
                    )
                    .await?;

                let event_target =
                    PduEventTarget::LogicalLink(self.get_module_handle(), cll_data.h_cll);
                let register_result = worker
                    .pdu_register_event_callback(event_target.clone(), Some(event_callback))
                    .await;

                if let Err(err) = register_result {
                    let _ = worker
                        .pdu_destroy_com_logical_link(self.get_module_handle(), cll_data.h_cll);
                    return Err(err)?;
                }

                let cll = Arc::new_cyclic(|weak| PduLogicalLink {
                    me: weak.clone(),
                    api: self.api.clone(),
                    worker: OnceLock::default(),
                    unique_tag,
                    cll_data: Arc::new(cll_data),
                    event_tx: tx,
                    event_rx: Arc::new(rx),
                    sync: Arc::default(),
                });

                cll.set_worker(worker.clone());

                // Register cll reference for unique tag.
                PduHandleManager::register_cll(
                    self.api.unique_tag,
                    unique_tag,
                    None,
                    Some(Arc::downgrade(&cll)),
                );

                Ok(cll)
            }
            None => {
                debug!(
                    "The use of asynchronous functions is not recommended outside of PduAsyncWorker"
                );
                let me = self.take_me_expect();

                let create_type = create_type.to_owned();
                let create_flags = create_flags.to_owned();

                let thread =
                    move || me.blocking_create_logical_link(&create_type, &create_flags);

                let cll = spawn_blocking(thread).await.expect(
                    "internal error: Vci::blocking_create_com_logical_link task panicked",
                )?;

                Ok(cll)
            }
        }
    }

    // TODO : fast req res
}

impl Drop for PduVci {
    fn drop(&mut self) {
        // There is no need to disconnect all `ComLogicalLink`s created by this VCI,
        // as `PduModuleDisconnect` disconnects them automatically.

        debug!(
            h_mod = self.get_module_handle(),
            "Disconnecting the Vci via destructor..."
        );

        match self.worker.get() {
            Some(worker) => {
                let query = Query::VtModuleDestructor(self.get_module_handle());
                match worker.request(query, None) {
                    Ok(_) => {}
                    Err(err) => {
                        error!(
                            h_mod = self.get_module_handle(),
                            "Error when disconnecting the module via destructor: {err}"
                        );
                    }
                }
            }
            None => {
                debug!(
                    h_mod = self.get_module_handle(),
                    "The use of asynchronous functions is not recommended outside of PduAsyncWorker"
                );
                let api = self.api.clone();
                let h_mod = self.get_module_handle();
                std::thread::spawn(move || api.vt_module_destructor(h_mod));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct VciStatus(PduStatusData);

impl Deref for VciStatus {
    type Target = PduStatusData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl VciStatus {
    pub fn is_available_for_connection(&self) -> bool {
        self.is_status_avail()
    }

    pub fn is_connected(&self) -> bool {
        self.is_status_ready() || self.is_status_not_ready()
    }

    pub fn is_status_avail(&self) -> bool {
        matches!(self.status_code, PduStatus::ModstAvail)
    }

    pub fn is_status_not_avail(&self) -> bool {
        matches!(self.status_code, PduStatus::ModstNotAvail)
    }

    pub fn is_status_ready(&self) -> bool {
        matches!(self.status_code, PduStatus::ModstReady)
    }

    pub fn is_status_not_ready(&self) -> bool {
        matches!(self.status_code, PduStatus::ModstNotReady)
    }
}

#[derive(Debug, Clone)]
pub enum ListResolver<'a> {
    Api(&'a Arc<PduApi>),
    Worker(&'a Arc<PduAsyncWorker>),
}

impl<'a> From<&'a Arc<PduApi>> for ListResolver<'a> {
    fn from(value: &'a Arc<PduApi>) -> Self {
        ListResolver::Api(value)
    }
}

impl<'a> From<&'a Arc<PduAsyncWorker>> for ListResolver<'a> {
    fn from(value: &'a Arc<PduAsyncWorker>) -> Self {
        ListResolver::Worker(value)
    }
}
