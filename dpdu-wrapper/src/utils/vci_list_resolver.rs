use std::sync::Arc;
use tracing::{error, info};
use crate::api::{Api, Result as ApiResult};
use crate::types::pdu_vci::PduVci;

pub type VciList = Vec<Arc<PduVci>>;

#[derive(Debug, Clone)]
pub struct PduVciListResolver;

impl PduVciListResolver {
    pub fn resolve(api: &Api) -> ApiResult<VciList> {
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
                status: api.pdu_get_status(module.h_mod, None, None)?,
            }));
        }

        info!("Successfully retrieved {} communication modules", list.len());
        Ok(list)
    }
}