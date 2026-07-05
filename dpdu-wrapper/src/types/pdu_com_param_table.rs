use crate::types::PduUniqueRespIdentifier;
use crate::types::pdu_com_param::PduComParam;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Default)]
pub struct PduComParamTable(pub HashMap<PduUniqueRespIdentifier, HashSet<PduComParam>>);

impl Deref for PduComParamTable {
    type Target = HashMap<PduUniqueRespIdentifier, HashSet<PduComParam>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PduComParamTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PduComParamTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        id: PduUniqueRespIdentifier,
        com_param: PduComParam,
    ) -> Option<PduComParam> {
        self.entry(id).or_default().replace(com_param)
    }
}
