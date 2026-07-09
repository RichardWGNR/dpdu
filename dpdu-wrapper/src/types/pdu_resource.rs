use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use dpdu_api_types::PduObjt;
use crate::api::{PduApi, Result as ApiResult};
use crate::types::{PduModuleHandle, PduObjectId};

pub type PduResourceStatus = HashMap<PduResource, ResourceStatus>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PduResource {
    pub h_mod: PduModuleHandle,
    pub resource_id: PduObjectId,
}

impl PduResource {
    pub fn new(h_mod: PduModuleHandle, resource_id: PduObjectId) -> Self {
        PduResource { h_mod, resource_id }
    }

    pub fn get_module_handle(&self) -> PduModuleHandle {
        self.h_mod
    }

    pub fn get_resource_id(&self) -> PduObjectId {
        self.resource_id
    }
}

#[derive(Debug, Clone)]
pub struct ResourceStatus {
    pub raw_status: u32,

    /// - false = resource not in use
    /// - true = resource in use
    pub busy: bool,

    /// - false = resource available
    /// - true = resource not available
    pub available: bool,

    /// - false = Transmit Queue is not locked
    /// - true = Transmit Queue is locked by a CLL. No other CLL except the
    ///       one which holds the lock is allowed to transmit on the physical
    ///       resource.
    pub transmit_queue_lock: bool,

    /// - false = Physical ComParams are not locked
    /// - true = Physical ComParams are locked by a CLL. No other CLL
    ///          except the one which holds the lock is allowed to change the
    ///          physical ComParams for the resource.
    pub physical_com_param_lock: bool,
}

#[derive(Debug, Clone)]
pub enum BusSource {
    /// Bus type by ID.
    ///
    /// Not recommended.
    ///
    /// The bus type id will be used ‘as is’.
    Id(PduObjectId),

    /// Bus type by name.
    ///
    /// Recommended.
    ///
    /// It will be taken from the module description file or by calling the PDUGetObjectId function.
    Name(String),
}

impl BusSource {
    pub(crate) fn resolve_bus_id(&self, api: &PduApi) -> ApiResult<PduObjectId> {
        Ok(match self {
            BusSource::Id(id) => id.clone(),
            BusSource::Name(name) => api.pdu_get_object_id(PduObjt::BusType, name)?,
        })
    }
}

impl From<PduObjectId> for BusSource {
    fn from(value: PduObjectId) -> Self {
        BusSource::Id(value)
    }
}

impl From<String> for BusSource {
    fn from(value: String) -> Self {
        BusSource::Name(value)
    }
}

impl From<&str> for BusSource {
    fn from(value: &str) -> Self {
        BusSource::Name(value.to_owned())
    }
}

impl Display for BusSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BusSource::Id(v) => write!(f, "#{v}"),
            BusSource::Name(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProtocolSource {
    /// Protocol type by ID.
    ///
    /// Not recommended.
    ///
    /// The protocol id will be used ‘as is’.
    Id(PduObjectId),

    /// Protocol by name.
    ///
    /// Recommended.
    ///
    /// It will be taken from the module description file or by calling the PDUGetObjectId function.
    Name(String),
}

impl ProtocolSource {
    pub(crate) fn resolve_protocol_id(&self, api: &PduApi) -> ApiResult<PduObjectId> {
        Ok(match self {
            ProtocolSource::Id(id) => id.clone(),
            ProtocolSource::Name(name) => {
                api.pdu_get_object_id(PduObjt::Protocol, name)?
            }
        })
    }
}

impl From<PduObjectId> for ProtocolSource {
    fn from(value: PduObjectId) -> Self {
        ProtocolSource::Id(value)
    }
}

impl From<String> for ProtocolSource {
    fn from(value: String) -> Self {
        ProtocolSource::Name(value)
    }
}

impl From<&str> for ProtocolSource {
    fn from(value: &str) -> Self {
        ProtocolSource::Name(value.to_owned())
    }
}

impl Display for ProtocolSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolSource::Id(v) => write!(f, "#{v}"),
            ProtocolSource::Name(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PinSource {
    /// Pin type by ID.
    ///
    /// Not recommended.
    ///
    /// The pin type id will be used ‘as is’.
    Id(u32),

    /// Pin type by name.
    ///
    /// Recommended.
    ///
    /// It will be taken from the module description file or by calling the PDUGetObjectId function.
    Name(String),
}

impl From<PduObjectId> for PinSource {
    fn from(value: PduObjectId) -> Self {
        PinSource::Id(value)
    }
}

impl From<String> for PinSource {
    fn from(value: String) -> Self {
        PinSource::Name(value)
    }
}

impl From<&str> for PinSource {
    fn from(value: &str) -> Self {
        PinSource::Name(value.to_owned())
    }
}

impl Display for PinSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PinSource::Id(v) => write!(f, "#{v}"),
            PinSource::Name(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TargetPin {
    pub num_on_vci: u32,

    pub pin_type: PinSource,
}
