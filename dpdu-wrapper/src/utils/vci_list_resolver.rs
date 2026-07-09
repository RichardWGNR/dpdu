use crate::api::{PduApi, Result as ApiResult};
use crate::types::pdu_status::PduStatusTarget;
use crate::types::pdu_vci::PduVci;
use crate::worker::{PduAsyncWorker, Query, Response, WorkerResult};
use std::sync::Arc;
use tracing::{error, info};

pub type VciList = Vec<Arc<PduVci>>;

#[derive(Debug, Clone)]
pub struct VciListResolver;

impl VciListResolver {
    pub fn blocking_resolve(api: &PduApi) -> ApiResult<VciList> {
        info!("Attempt to retrieve the list of communication modules (VCI)...");

        let modules = api.pdu_get_module_ids().inspect_err(|err| {
            error!("Failed to retrieve the list of communication modules: {err}");
        })?;

        let mut list = Vec::with_capacity(modules.len());

        for module in modules.into_iter() {
            list.push(Arc::new(PduVci {
                api: api.me.clone(),
                h_mod: module.h_mod,
                module_name: module.vendor_module_name,
                additional_info: module.vendor_additional_info,
                status: api.pdu_get_status(PduStatusTarget::Module(module.h_mod))?,
            }));
        }

        info!(
            "Successfully retrieved {} communication modules",
            list.len()
        );

        Ok(list)
    }

    pub async fn resolve(worker: &PduAsyncWorker) -> WorkerResult<VciList> {
        match worker
            .receive_query_response_callback(Query::ResolveVciList)
            .await
        {
            Ok(Response::ResolveVciList(v)) => Ok(v?),
            _ => unreachable!(),
        }
    }
}
