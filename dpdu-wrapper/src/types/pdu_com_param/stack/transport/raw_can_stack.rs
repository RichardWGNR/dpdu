use dpdu_wrapper_support::impl_configure_from_serde_json_map_for_com_param_stack;
use map_macro::{hash_map_e, hash_set};
use crate::types::pdu_com_param::stack::ComParamDefinitionStack;
use crate::types::pdu_com_param::table::{ComParamDefinition, ComParamDefinitionSet, ComParamDefinitionTable};
use crate::utils::ecu_name_to_unique_resp_id;

#[derive(Debug, Clone)]
pub struct RawCanTransportStack {
    /// CP_CanFillerByte.
    ///
    /// Определяет, каким значением будут заполняться неиспользуемые байты в CAN-кадре, если его
    /// длина меньше максимально допустимой (8 байт в Classical CAN или до 64 байт в CAN FD).
    ///
    /// Диапазон: от 0x00 до 0xFF.
    pub can_filler_byte: u32,

    /// CP_CanFillerByteHandling.
    ///
    /// Задает правила обработки байтов-заполнителей в сообщениях ISO-TP.
    /// Это касается как отправки, так и приёма данных в случае, если длина полезной нагрузки
    /// меньше, чем максимально допустимый размер кадра (8 байт для Classical CAN и до 64 байт
    /// для CAN FD).
    ///
    /// Диапазон: от 0 до 3.
    ///
    /// Значения:
    ///
    ///   0: No Handling (Ignore Filler Bytes) — байты-заполнители игнорируются при приёме.
    ///
    ///   1: Remove Filler Bytes — байты-заполнители автоматически удаляются из полученных данных.
    ///
    ///   2: Check Filler Bytes — байты-заполнители проверяются на соответствие значению,
    ///      указанному в CP_CanFillerByte. Если значение не совпадает, может возникнуть ошибка.
    ///
    ///   3: Replace Filler Bytes — байты-заполнители заменяются на указанное значение при обработке.
    pub can_filler_byte_handling: u32,

    /// CP_CanPhysReqExtAddr
    ///
    /// Используется для задания расширенного адреса в сообщениях, которые передаются на физическом
    /// уровне CAN, что позволяет использовать более длинные адреса (в отличие от стандартных
    /// 11-битных).
    pub can_phys_req_ext_addr: u32,

    /// CP_CanPhysReqFormat
    ///
    /// Определяет формат запроса, который будет использоваться на физическом уровне CAN при
    /// передаче сообщений.
    pub can_phys_req_format: u32,

    /// CP_CanPhysReqId
    ///
    /// Этот параметр указывает идентификатор (ID) сообщения для запроса, который будет отправлен
    /// через сеть CAN.
    pub can_phys_req_id: u32,

    /// CP_CanRespUUDTExtAddr
    ///
    /// Этот параметр используется для задания 29-битного расширенного адреса в ответных
    /// сообщениях, что позволяет системе работать с более длинными адресами по сравнению
    /// со стандартным 11-битным форматом.
    pub can_resp_uudt_ext_addr: u32,

    /// CP_CanRespUUDTFormat
    ///
    /// Регулирует, как будут структурированы данные в ответных сообщениях на физическом уровне
    /// CAN.
    /// Он включает в себя информацию о том, как должны быть представлены данные, как они должны
    /// быть упакованы и как должны быть организованы в ответных сообщениях.
    pub can_resp_uudt_format: u32,

    /// CP_CanRespUUDTId
    ///
    /// Уникальный идентификатор (ID), который будет использоваться в ответных сообщениях,
    /// отправляемых на физическом уровне CAN, и позволяет различать ответы от разных источников
    /// или разных типов сообщений.
    pub can_resp_uudt_id: u32,

    /// CP_RepeatReqCountTrans.
    ///
    /// Этот параметр указывает количество повторных попыток передачи запроса в случае его неудачи.
    /// Если запрос не был успешно обработан или передан, система попытается повторить передачу,
    /// и CP_RepeatReqCountTrans определяет, сколько раз это будет сделано.
    ///
    /// Диапазон значений: от 0 до 255.
    pub repeat_req_count_trans: u32,

    /// CP_SendRemoteFrame.
    ///
    /// Этот параметр управляет возможностью отправки Remote Frame (удаленного кадра) в сети CAN.
    ///
    /// Remote Frame — это тип кадра в сети CAN, который используется для запроса данных от
    /// другого устройства без передачи собственных данных. Он может быть использован в случае,
    /// когда одно устройство хочет запросить у другого устройства данные без фактической отправки
    /// данных, а только с запросом на передачу информации.
    pub send_remote_frame: u32,

    /// CP_ECULayerShortName
    ///
    /// Виртуальный параметр!
    /// Не должен передаваться напрямую. Вместо этого, по хешу его значения
    /// формируется [unique_resp_identifier] в [PDUSetUniqueRespIdTable].
    pub ecu_layer_short_name: String,
}

impl Default for RawCanTransportStack {
    fn default() -> Self {
        Self {
            can_filler_byte: 0,
            can_filler_byte_handling: 0,
            can_phys_req_ext_addr: 0,
            can_phys_req_format: 5,
            can_phys_req_id: 2016,
            can_resp_uudt_ext_addr: 0,
            can_resp_uudt_format: 0,
            can_resp_uudt_id: 2024,
            repeat_req_count_trans: 0,
            send_remote_frame: 0,
            ecu_layer_short_name: String::default()
        }
    }
}

impl RawCanTransportStack {
    pub fn with_can_filler_byte(mut self, byte: impl Into<u8>) -> Self {
        self.can_filler_byte = byte.into() as _;
        self
    }

    pub fn with_can_filler_byte_handling(mut self, status: bool) -> Self {
        self.can_filler_byte_handling = status.then(|| 1).unwrap_or(0);
        self
    }

    pub fn with_can_phys_req_ext_addr(mut self, value: impl Into<u32>) -> Self {
        self.can_phys_req_ext_addr = value.into();
        self
    }

    pub fn with_can_phys_req_format(mut self, format: impl Into<u32>) -> Self {
        self.can_phys_req_format = format.into();
        self
    }

    pub fn with_can_phys_req_id(mut self, id: impl Into<u32>) -> Self {
        self.can_phys_req_id = id.into();
        self
    }

    pub fn with_can_resp_uudt_ext_addr(mut self, value: impl Into<u32>) -> Self {
        self.can_resp_uudt_ext_addr = value.into();
        self
    }

    pub fn with_can_resp_uudt_format(mut self, format: impl Into<u32>) -> Self {
        self.can_resp_uudt_format = format.into();
        self
    }

    pub fn with_can_resp_uudt_id(mut self, id: impl Into<u32>) -> Self {
        self.can_resp_uudt_id = id.into();
        self
    }

    pub fn with_repeat_req_count_trans(mut self, value: impl Into<u32>) -> Self {
        self.repeat_req_count_trans = value.into();
        self
    }

    pub fn with_send_remote_frame(mut self, value: impl Into<u32>) -> Self {
        self.send_remote_frame = value.into();
        self
    }
    
    pub fn with_ecu_layer_short_name(mut self, value: impl Into<String>) -> Self {
        self.ecu_layer_short_name = value.into();
        self
    }
}

impl ComParamDefinitionStack for RawCanTransportStack {
    impl_configure_from_serde_json_map_for_com_param_stack! {
        CP_CanFillerByte: u32,
        CP_CanFillerByteHandling: u32,
        CP_CanPhysReqExtAddr: u32,
        CP_CanPhysReqFormat: u32,
        CP_CanPhysReqId: u32,
        CP_CanRespUUDTExtAddr: u32,
        CP_CanRespUUDTFormat: u32,
        CP_CanRespUUDTId: u32,
        CP_RepeatReqCountTrans: u32,
        CP_SendRemoteFrame: u32,
        CP_EcuLayerShortName: String
    }

    fn build_set(&self) -> ComParamDefinitionSet<ComParamDefinition> {
        use crate::types::pdu_com_param::table::ComParamDefinition as Def;
        use dpdu_api_types::PduPc::{Com, ErrHdl};

        ComParamDefinitionSet(hash_set! {
            // Com.
            Def::new(Com, "CP_CanFillerByte", self.can_filler_byte),
            Def::new(Com, "CP_CanFillerByteHandling", self.can_filler_byte_handling),
            Def::new(Com, "CP_SendRemoteFrame", self.send_remote_frame),

            // Error handling.
            Def::new(ErrHdl, "CP_RepeatReqCountTrans", self.repeat_req_count_trans)
        })
    }

    fn build_table(&self) -> ComParamDefinitionTable<ComParamDefinition> {
        use crate::types::pdu_com_param::table::ComParamDefinition as Def;
        use dpdu_api_types::PduPc::UniqueId;

        let id = if self.ecu_layer_short_name.is_empty() {
            0
        } else {
            ecu_name_to_unique_resp_id(&self.ecu_layer_short_name)
        };

        ComParamDefinitionTable(hash_map_e! {
            id => hash_set! {
                Def::new(UniqueId, "CP_CanPhysReqExtAddr", self.can_phys_req_ext_addr),
                Def::new(UniqueId, "CP_CanPhysReqFormat", self.can_phys_req_format),
                Def::new(UniqueId, "CP_CanPhysReqId", self.can_phys_req_id),
                Def::new(UniqueId, "CP_CanRespUUDTExtAddr", self.can_resp_uudt_ext_addr),
                Def::new(UniqueId, "CP_CanRespUUDTFormat", self.can_resp_uudt_format),
                Def::new(UniqueId, "CP_CanRespUUDTId", self.can_resp_uudt_id),
            }
        })
    }
}