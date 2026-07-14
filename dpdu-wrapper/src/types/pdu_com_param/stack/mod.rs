use std::collections::HashMap;
use crate::types::pdu_com_param::table::{ComParamDefinition, ComParamDefinitionSet, ComParamDefinitionTable};

pub mod application;
pub mod composite;
pub mod physical;
pub mod transport;

pub trait ComParamDefinitionStack {
    /// Для конфигурации через JSON.
    fn configure_from_serde_json_map(&mut self, map: &HashMap<String, serde_json::Value>);

    /// Для использования в PDUSetComParam.
    fn build_set(&self) -> ComParamDefinitionSet<ComParamDefinition>;

    /// Для использования в PDUSetUniqueRespIdTable.
    fn build_table(&self) -> ComParamDefinitionTable<ComParamDefinition>;
}