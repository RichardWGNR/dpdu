mod rpc;

use crate::api::ApiError;
use crate::api::PduApi;
use crate::types::pdu_vci::PduVci;
use crossbeam_channel::select;
pub use rpc::Query;
pub use rpc::Response;
use std::sync::{Arc, Weak};
use std::thread::spawn;
use tokio::sync::oneshot;
use tracing::{info, warn};
use crate::types::PduModuleHandle;

pub type WorkerResult<T> = std::result::Result<T, WorkerError>;

#[derive(Debug, thiserror::Error)]
pub enum WorkerError {
    #[error("api error: {0}")]
    ApiError(#[from] ApiError),

    #[error("channel error: {0}")]
    ChannelError(String),

    #[error("worker stopped")]
    WorkerStopped,
}

#[derive(Debug, Clone)]
pub struct PduAsyncWorker {
    pub(crate) me: Weak<PduAsyncWorker>,

    pub(crate) api: Arc<PduApi>,

    pub(crate) shutdown_tx: crossbeam_channel::Sender<()>,

    pub(crate) query_tx: crossbeam_channel::Sender<(Query, Option<oneshot::Sender<Response>>)>,
}

impl PduAsyncWorker {
    pub fn new(api: Arc<PduApi>) -> Arc<Self> {
        let (shdn_tx, shdn_rx) = crossbeam_channel::bounded(1);
        let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();

        cmd_tx.send((Query::PduConstruct, None)).unwrap();

        let worker = Arc::new_cyclic(|weak| PduAsyncWorker {
            me: weak.clone(),
            api: api.clone(),
            shutdown_tx: shdn_tx,
            query_tx: cmd_tx,
        });

        // supervisor
        spawn(move || {
            loop {
                let api = api.clone();
                let shdn_rx = shdn_rx.clone();
                let cmd_rx = cmd_rx.clone();

                let thread_result = spawn(move || {
                    PduAsyncWorker::command_thread(api.clone(), shdn_rx.clone(), cmd_rx.clone())
                })
                .join();

                if thread_result.is_ok() {
                    break; // normal thread termination
                } else {
                    warn!("D-PDU command worker panicked; restarting worker thread");
                }
            }
        });

        worker
    }

    pub fn get_api(&self) -> &PduApi {
        &self.api
    }

    pub(crate) fn request(
        &self,
        query: Query,
        tx: Option<oneshot::Sender<Response>>,
    ) -> WorkerResult<()> {
        self.query_tx
            .try_send((query, tx))
            .map_err(|err| WorkerError::ChannelError(err.to_string()))?;

        Ok(())
    }

    fn command_thread(
        api: Arc<PduApi>,
        shdn: crossbeam_channel::Receiver<()>,
        cmd: crossbeam_channel::Receiver<(Query, Option<oneshot::Sender<Response>>)>,
    ) {
        use rpc::Query as Q;
        use rpc::Response as R;

        loop {
            select! {
                recv(shdn) -> _ => {
                    info!("Shutdown received");
                    return;
                },

                recv(cmd) -> msg => {
                    let (query, resp_tx) = match msg {
                        Ok((query, resp_tx)) => (query, resp_tx),
                        Err(err) => {
                            unreachable!("Unexpected command receive errror: {}", err);
                        }
                    };
                    
                    let response = match query {
                        // Virtual functions.
                        Q::VtVciList => R::VtVciList(PduVci::blocking_list(&api)),
                        Q::VtModuleDestructor(h_mod) => R::VtModuleDestructor(api.vt_module_destructor(h_mod)),
                        Q::VtCllDestructor(h_mod, h_cll) => R::VtCllDestructor(api.vt_cll_destructor(h_mod, h_cll)),
                        Q::VtCopDestructor(h_mod, h_cll, h_cop) => R::VtCopDestructor(api.vt_cop_destructor(h_mod, h_cll, h_cop)),

                        // Real D-PDU API queries.
                        Q::PduCancelComPrimitive(h_mod, h_cll, h_cop) => R::PduCancelComPrimitive(api.pdu_cancel_com_primitive(h_mod, h_cll, h_cop)),
                        Q::PduConnect(h_mod, h_cll) => R::PduConnect(api.pdu_connect(h_mod, h_cll)),
                        Q::PduConstruct => R::PduConstruct(api.pdu_construct()),
                        Q::PduCreateComLogicalLink(h_mod, create_type, create_flags, tag) => R::PduCreateComLogicalLink(api.pdu_create_com_logical_link(h_mod, &create_type, &create_flags, tag)),
                        Q::PduDestruct => R::PduDestruct(api.pdu_destruct()),
                        Q::PduDestroyComLogicalLink(h_mod, h_cll) => R::PduDestroyComLogicalLink(api.pdu_destroy_com_logical_link(h_mod, h_cll)),
                        Q::PduDestroyItem(ptr) => R::PduDestroyItem(api.pdu_destroy_item(ptr.0 as _)),
                        Q::PduDisconnect(h_mod, h_cll) => R::PduDisconnect(api.pdu_disconnect(h_mod, h_cll)),
                        Q::PduGetComParam(h_mod, h_cll, object_id) => R::PduGetComParam(api.pdu_get_com_param(h_mod, h_cll, object_id)),
                        Q::PduGetConflictingResources(resource_id, modules) => R::PduGetConflictingResources(api.pdu_get_conflicting_resources(resource_id, modules)),
                        Q::PduGetEventItem(target) => R::PduGetEventItem(api.pdu_get_event_item(&target)),
                        Q::PduGetLastError(target) => R::PduGetLastError(api.pdu_get_last_error(&target)),
                        Q::PduGetModuleIds => R::PduGetModuleIds(api.pdu_get_module_ids()),
                        Q::PduGetObjectId(object, short_name) => R::PduGetObjectId(api.pdu_get_object_id(object, &short_name)),
                        Q::PduGetResourceIds(h_mod, bus, protocol, pins) => R::PduGetResourceIds(api.pdu_get_resource_ids(h_mod, &bus, &protocol, &pins)),
                        Q::PduGetResourceStatus(resources) => R::PduGetResourceStatus(api.pdu_get_resource_status(resources)),
                        Q::PduGetStatus(target) => R::PduGetStatus(api.pdu_get_status(&target)),
                        Q::PduGetTimestamp(h_mod) => R::PduGetTimestamp(api.pdu_get_timestamp(h_mod)),
                        Q::PduGetUniqueRespIdTable(h_mod, h_cll) => R::PduGetUniqueRespIdTable(api.pdu_get_unique_resp_id_table(h_mod, h_cll)),
                        Q::PduGetVersion(h_mod) => R::PduGetVersion(api.pdu_get_version(h_mod)),
                        Q::PduIoCtl(target, command, data) => R::PduIoCtl(api.pdu_io_ctl(&target, &command, data.as_ref())),
                        Q::PduLockResource(h_mod, h_cll, mask) => R::PduLockResource(api.pdu_lock_resource(h_mod, h_cll, mask)),
                        Q::PduModuleConnect(h_mod) => R::PduModuleConnect(api.pdu_module_connect(h_mod)),
                        Q::PduModuleDisconnect(h_mod) => R::PduModuleDisconnect(api.pdu_module_disconnect(h_mod)),
                        Q::PduRegisterEventCallback(target, callback) => R::PduRegisterEventCallback(api.pdu_register_event_callback(&target, callback)),
                        Q::PduSetComParam(h_mod, h_cll, cp) => R::PduSetComParam(api.pdu_set_com_param(h_mod, h_cll, &cp)),
                        Q::PduSetUniqueRespIdTable(h_mod, h_cll, table) => R::PduSetUniqueRespIdTable(api.pdu_set_unique_resp_id_table(h_mod, h_cll, &table)),
                        Q::PduStartComPrimitive(h_mod, h_cll, cop_type, data, params, tag) => R::PduStartComPrimitive(api.pdu_start_com_primitive(h_mod, h_cll, cop_type, &data, params.as_ref(), tag)),
                        Q::PduUnlockResource(h_mod, h_cll, mask) => R::PduUnlockResource(api.pdu_unlock_resource(h_mod, h_cll, mask))
                    };

                    if let Some(tx) = resp_tx {
                        let _ = tx.send(response);
                    }
                }
            }
        }
    }

    pub(crate) async fn receive_query_response_callback(
        &self,
        query: Query,
    ) -> WorkerResult<Response> {
        let (tx, rx) = oneshot::channel();

        self.request(query, Some(tx))?;

        rx.await
            .map_err(|e| WorkerError::ChannelError(e.to_string()))
    }

    /*
    pub fn get_vci_list(&self) -> VciList {
        let (tx, rx) = oneshot::channel();

        self.request(Query::VciList, Some(tx));
    }*/
}

impl Drop for PduAsyncWorker {
    fn drop(&mut self) {
        use crossbeam_channel::TrySendError;

        match self.shutdown_tx.try_send(()) {
            Err(TrySendError::Disconnected(_)) => {
                panic!("Unexpected closure of the shutdown channel in PduAsyncWorker");
            }
            _ => {}
        }
    }
}
