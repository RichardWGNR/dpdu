use crate::types::{PduCllHandle, PduModuleHandle, PduObjectId};
use crate::utils::PhantomRef;
use dpdu_api_types::{
    IoEventQueuePropertyData, IoFilterData, IoProgVoltageData, PduDataItem, PduIt,
};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub enum PduIoCtlCommand {
    Id(PduObjectId),
    Name(String),
}

impl Display for PduIoCtlCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PduIoCtlCommand::Id(v) => write!(f, "#{v}"),
            PduIoCtlCommand::Name(v) => write!(f, "{v}"),
        }
    }
}

macro_rules! impl_pdu_io_ctl_command_from_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl From<$t> for PduIoCtlCommand {
                fn from(value: $t) -> Self {
                    PduIoCtlCommand::Id(value.into())
                }
            }
        )*
    };
}

impl_pdu_io_ctl_command_from_int!(u8, u16, u32);

impl From<String> for PduIoCtlCommand {
    fn from(value: String) -> Self {
        PduIoCtlCommand::Name(value)
    }
}

impl From<&str> for PduIoCtlCommand {
    fn from(value: &str) -> Self {
        PduIoCtlCommand::Name(value.to_owned())
    }
}

#[derive(Debug, Clone, strum::AsRefStr)]
pub enum PduIoCtlData {
    U32(u32),
    ProgVoltage(IoProgVoltageData),
    ByteArray(IoCtlByteArray),
    Filter(IoFilterData),
    EventQueueProperty(IoEventQueuePropertyData),
}

impl PduIoCtlData {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

macro_rules! impl_pdu_it_ctl_data_from_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl From<$t> for PduIoCtlData {
                fn from(value: $t) -> Self {
                    PduIoCtlData::U32(value.into())
                }
            }
        )*
    };
}

impl_pdu_it_ctl_data_from_int!(u8, u16, u32);

impl From<IoProgVoltageData> for PduIoCtlData {
    fn from(value: IoProgVoltageData) -> Self {
        PduIoCtlData::ProgVoltage(value)
    }
}

impl From<IoCtlByteArray> for PduIoCtlData {
    fn from(value: IoCtlByteArray) -> Self {
        PduIoCtlData::ByteArray(value)
    }
}

impl From<IoFilterData> for PduIoCtlData {
    fn from(value: IoFilterData) -> Self {
        PduIoCtlData::Filter(value)
    }
}

impl From<IoEventQueuePropertyData> for PduIoCtlData {
    fn from(value: IoEventQueuePropertyData) -> Self {
        PduIoCtlData::EventQueueProperty(value)
    }
}

impl PduIoCtlData {
    pub(crate) fn to_pdu_data_item(&self) -> PhantomRef<'_, PduDataItem> {
        let (item_type, p_data) = match self {
            PduIoCtlData::U32(v) => (PduIt::IoUnum32, v as *const _ as _),
            PduIoCtlData::ProgVoltage(v) => (PduIt::IoProgVoltage, v as *const _ as _),
            PduIoCtlData::ByteArray(v) => (PduIt::IoByteArray, v.as_ptr() as _),
            PduIoCtlData::Filter(v) => (PduIt::IoFilter, v as *const _ as _),
            PduIoCtlData::EventQueueProperty(v) => {
                (PduIt::IoEventQueueProperty, v as *const _ as _)
            }
        };

        PhantomRef::new(PduDataItem { item_type, p_data })
    }
}

#[derive(Debug, Clone)]
pub struct IoCtlByteArray(pub(crate) Vec<u8>);

impl From<&[u8]> for IoCtlByteArray {
    fn from(value: &[u8]) -> Self {
        IoCtlByteArray(value.to_owned())
    }
}

impl Deref for IoCtlByteArray {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IoCtlByteArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub enum PduIoCtlTarget {
    System,
    Module(PduModuleHandle),
    LogicalLink(PduModuleHandle, PduCllHandle),
}

impl PduIoCtlTarget {
    pub fn is_system(&self) -> bool {
        matches!(self, PduIoCtlTarget::System)
    }

    pub fn is_module(&self) -> bool {
        matches!(self, PduIoCtlTarget::Module(..))
    }

    pub fn is_logical_link(&self) -> bool {
        matches!(self, PduIoCtlTarget::LogicalLink(..))
    }

    pub fn get_module_handle(&self) -> Option<PduModuleHandle> {
        match self {
            PduIoCtlTarget::Module(h_mod) => Some(h_mod.clone()),
            PduIoCtlTarget::LogicalLink(h_mod, ..) => Some(h_mod.clone()),
            _ => None,
        }
    }

    pub fn get_cll_handle(&self) -> Option<PduCllHandle> {
        match self {
            PduIoCtlTarget::LogicalLink(_, h_cll) => Some(h_cll.clone()),
            _ => None,
        }
    }
}
