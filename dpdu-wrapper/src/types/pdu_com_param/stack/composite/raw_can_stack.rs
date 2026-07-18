use std::collections::HashMap;
use serde_json::Value;
use crate::types::pdu_com_param::stack::application::{RawCanApplicationStack};
use crate::types::pdu_com_param::stack::ComParamDefinitionStack;
use crate::types::pdu_com_param::stack::physical::DwCanStack;
use crate::types::pdu_com_param::stack::transport::{RawCanTransportStack};
use crate::types::pdu_com_param::table::{ComParamDefinition, ComParamDefinitionSet, ComParamDefinitionTable};

/// ISO 11898 RAW.
#[derive(Debug, Clone, Default)]
pub struct RawCanStack {
    pub app_stack: RawCanApplicationStack,
    pub transport_stack: RawCanTransportStack,
    pub physical_stack: DwCanStack,
}

impl ComParamDefinitionStack for RawCanStack {
    fn configure_from_serde_json_map(&mut self, map: &HashMap<String, Value>) {
        self.app_stack.configure_from_serde_json_map(map);
        self.transport_stack.configure_from_serde_json_map(map);
        self.physical_stack.configure_from_serde_json_map(map);
    }

    fn build_set(&self) -> ComParamDefinitionSet<ComParamDefinition> {
        self.app_stack
            .build_set()
            .merge(self.transport_stack.build_set())
            .merge(self.physical_stack.build_set())
    }

    fn build_table(&self) -> ComParamDefinitionTable<ComParamDefinition> {
        self.app_stack
            .build_table()
            .merge(self.transport_stack.build_table())
            .merge(self.physical_stack.build_table())
    }
}