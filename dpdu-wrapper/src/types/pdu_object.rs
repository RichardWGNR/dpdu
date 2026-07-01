use std::fmt::{Display, Formatter};
use crate::types::PduObjectId;

#[derive(Debug, Clone)]
pub enum PduObjectIdSource {
    Id(PduObjectId),
    ShortName(String)
}

impl From<&str> for PduObjectIdSource {
    fn from(value: &str) -> Self {
        PduObjectIdSource::ShortName(value.to_owned())
    }
}

impl From<String> for PduObjectIdSource {
    fn from(value: String) -> Self {
        PduObjectIdSource::ShortName(value)
    }
}

impl Display for PduObjectIdSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PduObjectIdSource::Id(v) => write!(f, "#{v}"),
            PduObjectIdSource::ShortName(v) => write!(f, "{v}")
        }
    }
}

macro_rules! impl_from_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl From<$t> for PduObjectIdSource {
                fn from(value: $t) -> Self {
                    PduObjectIdSource::Id(value.into())
                }
            }
        )*
    };
}

impl_from_int!(u8, u16, u32);