use dpdu_api_types::PduError;

mod detours;
mod vxdiag;

pub fn wrap_pdu_call<F>(func: &str, mut f: F) -> PduError
where
    F: FnMut() -> PduError,
{
    create_detours();
    register_callbacks();

    let result = f();

    if vxdiag::has_device_not_connected() {
        return match func {
            "PDUModuleDisconnect"
            | "PDUGetTimestamp"
            | "PDUIoCtl"
            | "PDUGetVersion"
            | "PDUGetLastError"
            | "PDUCreateComLogicalLink"
            | "PDUDestroyComLogicalLink"
            | "PDUDisconnect"
            | "PDULockResource"
            | "PDUUnlockResource"
            | "PDUGetComParam"
            | "PDUSetComParam"
            | "PDUStartComPrimitive"
            | "PDUCancelComPrimitive"
            | "PDUGetEventItem"
            | "PDURegisterEventCallback"
            | "PDUGetUniqueRespIdTable"
            | "PDUSetUniqueRespIdTable" => PduError::ModuleNotConnected,
            _ => PduError::FctFailed,
        };
    }

    result
}

fn create_detours() {
    detours::msg_box_a::hook_message_box_a();
}

fn register_callbacks() {
    vxdiag::register_callback();
}
