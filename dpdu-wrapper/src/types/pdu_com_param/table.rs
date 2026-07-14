use crate::types::PduUniqueRespIdentifier;
use crate::types::pdu_com_param::{CpVariant, PduComParam};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use async_trait::async_trait;
use dpdu_api_types::PduPc;
use crate::api::{ApiResult, PduApi};
use crate::AsyncRuntimeTarget;
use crate::utils::ecu_name_to_unique_resp_id;
use crate::worker::WorkerResult;

pub type PduComParamSet = ComParamDefinitionSet<PduComParam>;

pub type PduComParamTable = ComParamDefinitionTable<PduComParam>;

#[derive(Clone, Default)]
pub struct ComParamDefinitionSet<T>(pub HashSet<T>)
where
    T: IntoPduComParam
;

impl<T> Deref for ComParamDefinitionSet<T>
where
    T: IntoPduComParam
{
    type Target = HashSet<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ComParamDefinitionSet<T>
where
    T: IntoPduComParam
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> ComParamDefinitionSet<T>
where
    T: IntoPduComParam
{
    pub(crate) fn with_capacity(cap: usize) -> Self {
        ComParamDefinitionSet(HashSet::with_capacity(cap))
    }

    pub fn new() -> Self {
        ComParamDefinitionSet(HashSet::new())
    }
    
    pub fn merge(mut self, other: Self) -> Self {
        for def in other.0 {
            self.insert(def);
        }
        self
    }
}

#[derive(Clone, Default)]
pub struct ComParamDefinitionTable<T>(pub HashMap<PduUniqueRespIdentifier, HashSet<T>>)
where
    T: IntoPduComParam;

impl<T> Deref for ComParamDefinitionTable<T>
where
    T: IntoPduComParam
{
    type Target = HashMap<PduUniqueRespIdentifier, HashSet<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ComParamDefinitionTable<T>
where
    T: IntoPduComParam
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> ComParamDefinitionTable<T>
where
    T: IntoPduComParam
{
    pub(crate) fn with_capacity(cap: usize) -> Self {
        ComParamDefinitionTable(HashMap::with_capacity(cap))
    }

    pub fn new() -> Self {
        ComParamDefinitionTable(HashMap::new())
    }

    pub fn add(
        &mut self,
        id: PduUniqueRespIdentifier,
        com_param: T,
    ) -> Option<T> {
        self.entry(id).or_default().replace(com_param)
    }

    pub fn add_with_ecu_name(
        &mut self,
        name: &str,
        com_param: T
    ) -> Option<T> {
        self.entry(ecu_name_to_unique_resp_id(name)).or_default().replace(com_param)
    }
    
    pub fn merge(mut self, another: Self) -> Self {
        for (id, set) in another.0 {
            for def in set {
                self.add(id, def);
            }
        }
        self
    }
}

mod sealed {
    pub trait Sealed {}
}

#[async_trait]
pub trait IntoPduComParam: sealed::Sealed + Eq + Hash {
    fn blocking_build(&self, api: &PduApi) -> ApiResult<PduComParam>;

    async fn build<'a>(&self, runtime: impl Into<AsyncRuntimeTarget<'a>> + Send) -> WorkerResult<PduComParam>;
}

impl sealed::Sealed for PduComParam {}

#[async_trait]
impl IntoPduComParam for PduComParam {
    fn blocking_build(&self, _api: &PduApi) -> ApiResult<PduComParam> {
        Ok(self.clone())
    }

    async fn build<'a>(&self, _runtime: impl Into<AsyncRuntimeTarget<'a>> + Send) -> WorkerResult<PduComParam> {
        Ok(self.clone())
    }
}

#[derive(Clone)]
pub struct ComParamDefinition {
    pub class: PduPc,
    pub short_name: String,
    pub variant: CpVariant
}

impl Eq for ComParamDefinition {}

impl PartialEq for ComParamDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.short_name.eq(&other.short_name)
    }
}

impl Hash for ComParamDefinition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.short_name.hash(state)
    }
}


impl ComParamDefinition {
    pub fn new(class: PduPc, short_name: impl Into<String>, variant: impl Into<CpVariant>) -> Self {
        let short_name = short_name.into();
        let variant = variant.into();
        Self {
            class,
            short_name,
            variant
        }
    }
}

impl sealed::Sealed for ComParamDefinition {}

#[async_trait]
impl IntoPduComParam for ComParamDefinition {
    fn blocking_build(&self, api: &PduApi) -> ApiResult<PduComParam> {
        PduComParam::blocking_from_short_name(
            api,
            &self.short_name,
            self.class,
            self.variant.clone()
        )
    }

    async fn build<'a>(&self, runtime: impl Into<AsyncRuntimeTarget<'a>> + Send) -> WorkerResult<PduComParam> {
        PduComParam::from_short_name(
            runtime,
            &self.short_name,
            self.class,
            self.variant.clone()
        ).await
    }
}