use crate::types::{PduCllHandle, PduModuleHandle};

#[derive(Debug, Clone, strum::AsRefStr)]
pub enum PduEventCallbackTarget {
    System,
    Module(PduModuleHandle),
    ComLogicalLink(PduModuleHandle, PduCllHandle),
}
