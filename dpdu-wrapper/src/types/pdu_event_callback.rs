use crate::types::{PduCllHandle, PduModuleHandle};

#[derive(Debug, Clone, strum::AsRefStr, strum::Display)]
pub enum PduEventCallbackTarget {
    System,
    Module(PduModuleHandle),
    ComLogicalLink(PduModuleHandle, PduCllHandle),
}
