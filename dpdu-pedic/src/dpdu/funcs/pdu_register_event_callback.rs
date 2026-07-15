use dpdu_api_types::{EventCallbackFn, PduError};
use crate::dpdu::event_bus::EventBus;
use crate::dpdu::state::PDU_STATE;
use crate::dpdu::types::pdu_event::PduEventTarget;

#[unsafe(no_mangle)]
pub extern "system" fn PDURegisterEventCallback(
    h_mod: u32,
    h_cll: u32,
    callback_fn: EventCallbackFn
) -> PduError {
    if !PDU_STATE.is_constructed() {
        return PduError::FctFailed;
    }

    let target = match PduEventTarget::from_api(h_mod, h_cll) {
        Ok(v) => v,
        Err(err) => {
            return err;
        }
    };

    let func: Option<EventCallbackFn> = unsafe {
        std::mem::transmute(callback_fn)
    };

    EventBus::register(&target, func);

    PduError::StatusNoError
}