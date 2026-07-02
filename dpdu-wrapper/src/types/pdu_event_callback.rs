use crate::types::{PduCllHandle, PduModuleHandle};

#[derive(Debug, Clone)]
#[derive(strum::AsRefStr)]
pub enum PduEventCallbackTarget {
    System,
    Module(PduModuleHandle),
    ComLogicalLink(PduModuleHandle, PduCllHandle)
}