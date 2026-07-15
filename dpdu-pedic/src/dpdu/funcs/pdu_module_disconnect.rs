use dpdu_api_types::{PduError, PduStatus};
use parking_lot::Mutex;
use crate::dpdu::state::PDU_STATE;
use crate::dpdu::types::PduModuleHandle;
use crate::passthru::PassthruModule;

#[unsafe(no_mangle)]
pub extern "system" fn PDUModuleDisconnect(
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
    
    if !matches!(module.get_status(), PduStatus::ModstReady) {
        return PduError::ModuleNotConnected;
    }
    
    let Some(interface) = module.get_interface() else {
        module.set_status(PduStatus::ModstAvail);
        module.set_interface(None);
        module.set_device(None);
        return PduError::ModuleNotConnected;
    };
    let Some(device) = module.get_device() else {
        module.set_status(PduStatus::ModstAvail);
        module.set_interface(None);
        module.set_device(None);
        return PduError::ModuleNotConnected;
    };

    module.set_status(PduStatus::ModstAvail);
    drop(device);
    drop(interface);

    // TODO : mark all resources as outdated and send signal to close them

    PduError::StatusNoError
}