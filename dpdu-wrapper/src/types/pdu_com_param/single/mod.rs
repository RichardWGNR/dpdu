use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize, Serializer};

pub mod unique;
pub mod com;
pub mod err_hdl;
pub mod timing;
pub mod tester_present;
pub mod bus_type;

pub(crate) fn deserialize_bool_from_u32<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>
{
    let value = u32::deserialize(deserializer)?;
    Ok(value > 0)
}

pub(crate) fn serialize_u32_from_bool<S>(
    value: &bool,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u32(if *value { 1 } else { 0 })
}