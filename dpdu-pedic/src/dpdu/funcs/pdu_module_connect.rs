use dpdu_api_types::{PduError, PduStatus};
use j2534_mrw::{Interface};
use parking_lot::Mutex;
use tracing::error;
use crate::dpdu::state::PDU_STATE;
use crate::dpdu::types::PduModuleHandle;
use crate::passthru::PassthruModule;

#[unsafe(no_mangle)]
pub extern "system" fn PDUModuleConnect(
    h_mod: PduModuleHandle
) -> PduError {
    static SYNC: Mutex<()> = Mutex::new(());
    let _sync = SYNC.lock();

    if !PDU_STATE.is_constructed() {
        return PduError::PduApiNotConstructed;
    }

    let Some(module) = PassthruModule::get(h_mod as _) else {
        return PduError::InvalidHandle;
    };
    let _mod_sync = module.sync.lock();
    
    if !matches!(module.get_status(), PduStatus::ModstAvail) {
        return PduError::FctFailed;
    }

    let interafce = match Interface::new(&module.library_path) {
        Ok(v) => Box::leak(Box::new(v)),
        Err(err) => {
            error!(h_mod, name = module.name, "Unable to connect passthru module: {err}");
            return PduError::FctFailed;
        }
    };
    
    let device = match interafce.open_any() {
        Ok(v) => v,
        Err(err) => {
            error!(h_mod, name = module.name, "Unable to connect passthru module: {err}");
            return PduError::FctFailed;
        }
    };
    
    module.set_status(PduStatus::ModstReady);
    module.set_interface(Some(interafce));
    module.set_device(Some(device));

    PduError::StatusNoError
}