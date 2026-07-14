use std::collections::HashMap;
use crate::types::pdu_com_param::stack::application::UdsStack;
use crate::types::pdu_com_param::stack::ComParamDefinitionStack;
use crate::types::pdu_com_param::stack::physical::DwCanStack;
use crate::types::pdu_com_param::stack::transport::IsoTpStack;
use crate::types::pdu_com_param::table::{ComParamDefinition, ComParamDefinitionSet, ComParamDefinitionTable};

/// Layers:
///   - Application: ISO 14229-3 (UDS)
///   - Transport (Bus): ISO 15765-2 (ISO-TP)
///   - Physical (Can): ISO 11898-2 (Dual Wire CAN)
#[derive(Debug, Clone)]
pub struct UdsOnIsoTpOnDwCanStack {
    pub app_stack: UdsStack,
    pub bus_stack: IsoTpStack,
    pub can_stack: DwCanStack,
}

impl Default for UdsOnIsoTpOnDwCanStack {
    fn default() -> Self {
        Self {
            app_stack: UdsStack {
                p2_max: 500_000,
                p3_func: 150000,
                p3_phys: 150000,
                rc21_completion_timeout: 10_000_000,
                rc21_handling: 1,
                rc21_request_time: 800_000,
                rc23_completion_timeout: 30_000_000,
                rc23_handling: 1,
                rc23_request_time: 500_000,
                rc78_completion_timeout: 30_000_000,
                rc78_handling: 1,
                rc_byte_offset: 2,
                tester_present_handling: 1,
                tester_present_req_rsp: 1,   // Response expected
                tester_present_send_type: 1, // On idle
                tester_present_addr_mode: 0, // physical
                tester_present_message: vec![0x3E, 0x00],
                tester_present_exp_pos_resp: vec![0x7E, 0x00],
                tester_present_exp_neg_resp: vec![0x7F, 0x3E],
                ..Default::default()
            },
            bus_stack: IsoTpStack {
                br: 0,
                cs: 0,
                can_phys_req_id: 0x7E0,
                can_resp_usdt_id: 0x7E8,
                ..Default::default()
            },
            can_stack: DwCanStack {
                termination_type: 3,
                ..Default::default()
            },
        }
    }
}

impl ComParamDefinitionStack for UdsOnIsoTpOnDwCanStack {
    fn configure_from_serde_json_map(&mut self, map: &HashMap<String, serde_json::Value>) {
        self.app_stack.configure_from_serde_json_map(map);
        self.bus_stack.configure_from_serde_json_map(map);
        self.can_stack.configure_from_serde_json_map(map);
    }

    fn build_set(&self) -> ComParamDefinitionSet<ComParamDefinition> {
        self.app_stack
            .build_set()
            .merge(self.bus_stack.build_set())
            .merge(self.can_stack.build_set())
    }

    fn build_table(&self) -> ComParamDefinitionTable<ComParamDefinition> {
        self.app_stack
            .build_table()
            .merge(self.bus_stack.build_table())
            .merge(self.can_stack.build_table())
    }
}
