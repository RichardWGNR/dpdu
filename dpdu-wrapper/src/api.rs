use std::sync::{Arc, Weak};
use dpdu_api_types::PduError;
use rand::random;
use tracing::{debug, error};
use crate::types::{PduLibraryPath, PduOptions, PduUniqueId};
use crate::utils::module_description::PduModuleDescription;
use crate::utils::root_file::Mvci;

#[derive(Debug)]
pub struct Api {
    me: Weak<Api>,

    pdu_options: PduOptions,

    pdu_unique_id: PduUniqueId,

    library: libloading::Library,

    library_file: Option<PduLibraryPath>,

    module_description: Option<PduModuleDescription>,

    mvci: Option<Mvci>,
}

impl Api {
    pub fn new(
        options: PduOptions,
        library: libloading::Library,
        library_file: Option<PduLibraryPath>,
        module_description: Option<PduModuleDescription>,
        mvci: Option<Mvci>
    ) -> Arc<Self> {
        Arc::new_cyclic(|me| Self {
            me: me.clone(),
            pdu_options: options,
            pdu_unique_id: random(),
            library,
            library_file,
            module_description,
            mvci,
        })
    }

    fn log_api_call(&self, func: &str) {
        debug!(func, "D-PDU API Call");
    }

    fn log_failed_api_call(&self, func: &str, result: PduError) {
        error!(
            func,
            result_str = result.as_ref(),
            result_int = format!("{:#X}", result as usize),
            "D-PDU API Call failed"
        );
    }
}
