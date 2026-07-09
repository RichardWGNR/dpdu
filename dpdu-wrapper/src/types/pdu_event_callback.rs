use crate::types::{PduCllHandle, PduModuleHandle};

#[derive(Debug, Clone)]
#[derive(strum::AsRefStr, strum::Display)]
pub enum PduEventCallbackTarget {
    System,
    Module(PduModuleHandle),
    ComLogicalLink(PduModuleHandle, PduCllHandle),
}
