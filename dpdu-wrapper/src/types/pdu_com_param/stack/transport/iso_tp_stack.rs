use crate::types::pdu_com_param::stack::ComParamDefinitionStack;
use crate::types::pdu_com_param::table::{
    ComParamDefinition, ComParamDefinitionSet, ComParamDefinitionTable,
};
use crate::utils::ecu_name_to_unique_resp_id;
use dpdu_wrapper_support::impl_configure_from_serde_json_map_for_com_param_stack;
use map_macro::{hash_map_e, hash_set};

/// Стек возможных коммуникационных параметров для транспортного протокола ISO-TP (ISO 15765-2).
///
/// Описания возможных параметров сгенерированы ChatGpt!
#[derive(Clone, Debug)]
pub struct IsoTpStack {
    /// CP_Ar (Response timeout).
    ///
    /// Задает максимальное время ожидания отклика (ответного кадра) в миллисекундах от
    /// получателя после отправки запроса.
    pub ar: u32,

    /// CP_As (Send timeout).
    ///
    /// Определяет максимальное время ожидания перед успешной отправкой кадра (Single Frame,
    /// First Frame, Flow Control или Consecutive Frame) на шину.
    pub r#as: u32,

    /// CP_BlockSize.
    ///
    /// Определяет количество кадров, которые отправитель может последовательно передать в режиме
    /// многофреймовой передачи перед ожиданием сообщения Flow Control (FC) от получателя.
    ///
    /// Значения:
    ///
    ///   0: Подтверждения (Flow Control) не требуются до конца передачи.
    ///
    ///   1..: Количество кадров между подтверждениями.
    pub block_size: u32,

    /// CP_BlockSizeOverride.
    ///
    /// Заставляет отправителя использовать фиксированное значение Block Size (BS) для
    /// многофреймовой передачи, независимо от того, что получатель указал в Flow Control (FC).
    ///
    /// Значения:
    ///
    ///   0: отключено, используется стандартный механизм ISO-TP с динамическим BS.
    ///
    ///   1..: количество фреймов между подтверждениями от получателя.
    pub block_size_override: u32,

    /// CP_Br.
    ///
    /// Задает максимальное время ожидания ответа в миллисекундах от принимающей стороны после
    /// отправки блока данных, если используется Block Size (BS) больше нуля.
    pub br: u32,

    /// CP_Bs.
    ///
    /// Определяет максимальное количество Consecutive Frames (CF), которые отправитель может
    /// последовательно передать без получения подтверждения Flow Control (FC) от принимающей
    /// стороны.
    ///
    /// Значения:
    ///
    ///   0: Подтверждения Flow Control не требуются, и отправитель может передавать данные до
    ///      конца передачи.
    ///
    ///   1..: количество кадров, после которых отправитель должен дождаться Flow Control с
    ///       разрешением на продолжение.
    pub bs: u32,

    /// CP_CanDataSizeOffset.
    ///
    /// Определяет количество дополнительных байт, которые следует учитывать при вычислении общего
    /// размера данных в CAN-кадре.
    ///
    /// Диапазон: 0..=255.
    pub can_data_size_offset: u32,

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

    /// CP_CanFirstConsecutiveFrameValue.
    ///
    /// Определяет значение (поле), которое будет использовано в качестве первого
    /// Consecutive Frame (CF), когда начинается передача данных в несколько кадров в рамках
    /// многофреймового сообщения.
    pub can_first_consecutive_frame_value: u32,

    /// CP_CANFDTxMaxDataLength.
    ///
    /// Определяет максимальный размер блока данных (payload) в одном кадре при передаче в
    /// формате CAN FD.
    pub canfd_tx_max_data_length: u32,

    /// CP_CanFuncReqExtAddr.
    ///
    /// Указывает значение расширенного адреса, который будет использован при отправке
    /// функциональных запросов, если это требуется в соответствии с протоколом, например,
    /// при использовании UDS (Unified Diagnostic Services) или других специализированных
    /// приложений.
    ///
    /// Диапазон значений: от 0x00000000 до 0xFFFFFFFF.
    pub can_func_req_ext_addr: u32,

    /// CP_CanFuncReqFormat.
    ///
    /// Определяет формат или структуру данных для функциональных запросов, которые будут
    /// отправляться в рамках протокола. Этот параметр регулирует, как данные запроса будут
    /// оформляться, а также могут использоваться для указания того, как будет организована
    /// передача функциональных запросов, например, в протоколах типа UDS (Unified Diagnostic
    /// Services) или других системах, использующих расширенную адресацию.
    ///
    /// Возможные значения зависят от используемого протокола, например, для UDS могут быть
    /// специфические типы запросов, такие как ReadDataByIdentifier, WriteDataByIdentifier
    /// и так далее.
    pub can_func_req_format: u32,

    /// CP_CanFuncReqId.
    ///
    /// Этот параметр указывает идентификатор для функциональных запросов, которые отправляются
    /// через CAN. В контексте различных приложений, например, в UDS (Unified Diagnostic Services)
    /// или других диагностических протоколах, CP_CanFuncReqId может задавать уникальный
    /// идентификатор для конкретных запросов или операций.
    ///
    /// Диапазон значений:
    ///
    /// Значение зависит от протокола или спецификации, например, в UDS это может быть код
    /// услуги (например, 0x10 для Diagnostic Session Control).
    pub can_func_req_id: u32,

    /// CP_CanMaxNumWaitFrames.
    ///
    /// Этот параметр указывает максимальное количество Flow Control Frames (FC), которые могут
    /// быть переданы в ответ на запрос в процессе передачи данных по протоколу ISO-TP.
    /// Flow Control Frames используются для управления потоком данных, особенно при передаче
    /// больших сообщений, чтобы избежать перегрузки принимающей стороны.
    ///
    /// Диапазон значений: от 0 до 255.
    pub can_max_num_wait_frames: u32,

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

    /// CP_CanRespUSDTExtAddr
    ///
    /// Используется для задания расширенного адреса (29-битного) в ответных сообщениях, что
    /// позволяет работать с более длинными идентификаторами сообщений по сравнению с
    /// традиционными 11-битными адресами в CAN.
    pub can_resp_usdt_ext_addr: u32,

    /// CP_CanRespUSDTFormat
    ///
    /// Регулирует, как будут форматироваться данные в ответных сообщениях, передаваемых через CAN.
    /// Он может включать информацию о структуре данных, длине сообщения, кодах ответа и
    /// других аспектах формата сообщения.
    pub can_resp_usdt_format: u32,

    /// CP_CanRespUSDTId
    ///
    /// Указывает, какой идентификатор (ID) будет использоваться для отправки ответов на
    /// диагностические запросы через сеть CAN.
    pub can_resp_usdt_id: u32,

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

    /// CP_Cr.
    ///
    /// Этот параметр управляет поведением системы в случае, если необходимо отменить ранее
    /// отправленный запрос. Обычно это используется в протоколах, где может возникнуть
    /// необходимость в прерывании передачи данных или запроса (например, при диагностике с
    /// использованием UDS или других протоколов).
    ///
    /// Тип: boolean (логическое значение: true/false) или, в зависимости от реализации, может
    /// быть целым числом (0 или 1).
    pub cr: u32,

    /// CP_Cs.
    ///
    /// Этот параметр управляет поведением системы в случае, если необходимо отменить текущую
    /// последовательность сообщений или операций. Это может быть полезно в протоколах,
    /// где осуществляется передача данных в несколько последовательных кадров, и необходимо
    /// прервать эту последовательность по какой-либо причине (например, ошибка или изменение
    /// условий).
    ///
    /// Тип: boolean (логическое значение: true/false) или целое число (например, 0 или 1 в
    /// некоторых реализациях).
    pub cs: u32,

    /// CP_ECULayerShortName
    ///
    /// Виртуальный параметр!
    /// Не должен передаваться напрямую. Вместо этого, по хешу его значения
    /// формируется [unique_resp_identifier] в [PDUSetUniqueRespIdTable].
    pub ecu_layer_short_name: String,

    /// CP_MaxFirstFrameDataLength.
    ///
    /// Определяет максимальный объем данных, которые могут быть отправлены в First Frame (FF).
    pub max_first_frame_data_length: u32,

    /// CP_RepeatReqCountTrans.
    ///
    /// Этот параметр указывает количество повторных попыток передачи запроса в случае его неудачи.
    /// Если запрос не был успешно обработан или передан, система попытается повторить передачу,
    /// и CP_RepeatReqCountTrans определяет, сколько раз это будет сделано.
    ///
    /// Диапазон значений: от 0 до 255.
    pub repeat_req_count_trans: u32,

    /// CP_RequestAddrMode.
    ///
    /// Этот параметр указывает режим адресации, который будет использоваться при отправке
    /// запросов или сообщений в сети CAN. В протоколе ISO-TP могут быть использованы различные
    /// способы адресации устройств (например, с использованием стандартных или расширенных
    /// идентификаторов). CP_RequestAddrMode позволяет системе выбрать, какой способ адресации
    /// применять для запросов.
    pub request_addr_mode: u32,

    /// CP_SendRemoteFrame.
    ///
    /// Этот параметр управляет возможностью отправки Remote Frame (удаленного кадра) в сети CAN.
    ///
    /// Remote Frame — это тип кадра в сети CAN, который используется для запроса данных от
    /// другого устройства без передачи собственных данных. Он может быть использован в случае,
    /// когда одно устройство хочет запросить у другого устройства данные без фактической отправки
    /// данных, а только с запросом на передачу информации.
    pub send_remote_frame: u32,

    /// CP_StMin.
    ///
    /// Этот параметр определяет минимальное время ожидания (или паузы) между кадрами, когда
    /// передача данных происходит с использованием многокадровых сообщений в протоколе ISO-TP.
    /// Это время важно для обеспечения корректной передачи и синхронизации передачи данных между
    /// устройствами.
    pub st_min: u32,

    /// CP_StMinOverride.
    ///
    /// Этот параметр позволяет переопределить стандартное минимальное время паузы между кадрами,
    /// установленное параметром CP_StMin, в специфических случаях или для определенных операций.
    /// Это полезно, если необходимо изменить задержку между кадрами для определенной передачи
    /// данных, например, для оптимизации производительности или удовлетворения специфических
    /// требований системы.
    pub st_min_override: u32,
}

impl Default for IsoTpStack {
    fn default() -> Self {
        Self {
            ar: 1_000_000,
            r#as: 1_000_000,
            block_size: 0,
            block_size_override: 0xFFFF,
            br: 10000,
            bs: 1000000,
            can_data_size_offset: 0,
            can_filler_byte: 0x55,
            can_filler_byte_handling: 1,
            can_first_consecutive_frame_value: 1,
            can_func_req_ext_addr: 0,
            can_func_req_format: 5,
            can_func_req_id: 0x7DF,
            can_max_num_wait_frames: 255,
            can_phys_req_ext_addr: 0,
            can_phys_req_format: 5,
            can_phys_req_id: 0x7E0,
            can_resp_usdt_ext_addr: 0,
            can_resp_usdt_format: 5,
            can_resp_usdt_id: 0x7E8,
            can_resp_uudt_ext_addr: 0,
            can_resp_uudt_format: 0,
            can_resp_uudt_id: u32::MAX,
            canfd_tx_max_data_length: 0,
            cr: 1000000,
            cs: 10000,
            ecu_layer_short_name: String::default(),
            max_first_frame_data_length: 4095,
            repeat_req_count_trans: 0,
            request_addr_mode: 1,
            send_remote_frame: 0,
            st_min: 0,
            st_min_override: 0xFFFFFFFF,
        }
    }
}

impl ComParamDefinitionStack for IsoTpStack {
    impl_configure_from_serde_json_map_for_com_param_stack! {
        CP_Ar: u32,
        CP_As: u32,
        CP_BlockSize: u32,
        CP_BlockSizeOverride: u32,
        CP_Br: u32,
        CP_Bs: u32,
        CP_CanDataSizeOffset: u32,
        CP_CanFillerByte: u32,
        CP_CanFillerByteHandling: u32,
        CP_CanFirstConsecutiveFrameValue: u32,
        CP_CanFuncReqExtAddr: u32,
        CP_CanFuncReqFormat: u32,
        CP_CanFuncReqId: u32,
        CP_CanMaxNumWaitFrames: u32,
        CP_CanPhysReqExtAddr: u32,
        CP_CanPhysReqFormat: u32,
        CP_CanPhysReqId: u32,
        CP_CanRespUSDTExtAddr: u32,
        CP_CanRespUSDTFormat: u32,
        CP_CanRespUSDTId: u32,
        CP_CanRespUUDTExtAddr: u32,
        CP_CanRespUUDTFormat: u32,
        CP_CanRespUUDTId: u32,
        CP_CANFDTxMaxDataLength: u32,
        CP_Cr: u32,
        CP_Cs: u32,
        CP_MaxFirstFrameDataLength: u32,
        CP_RepeatReqCountTrans: u32,
        CP_RequestAddrMode: u32,
        CP_SendRemoteFrame: u32,
        CP_StMin: u32,
        CP_StMinOverride: u32
    }

    fn build_set(&self) -> ComParamDefinitionSet<ComParamDefinition> {
        use crate::types::pdu_com_param::table::ComParamDefinition as Def;
        use dpdu_api_types::PduPc::{Com, ErrHdl, Timing as Time};

        ComParamDefinitionSet(hash_set! {
            // Timing.
            Def::new(Time, "CP_Ar", self.ar),
            Def::new(Time, "CP_As", self.r#as),
            Def::new(Time, "CP_Br", self.br),
            Def::new(Time, "CP_Bs", self.bs),
            Def::new(Time, "CP_Cr", self.cr),
            Def::new(Time, "CP_Cs", self.cs),
            Def::new(Time, "CP_StMin", self.st_min),
            Def::new(Time, "CP_StMinOverride", self.st_min_override),

            // Com.
            Def::new(Com, "CP_BlockSize", self.block_size),
            Def::new(Com, "CP_BlockSizeOverride", self.block_size_override),
            Def::new(Com, "CP_CanDataSizeOffset", self.can_data_size_offset),
            Def::new(Com, "CP_CanFillerByte", self.can_filler_byte),
            Def::new(Com, "CP_CanFillerByteHandling", self.can_filler_byte_handling),
            Def::new(Com, "CP_CanFirstConsecutiveFrameValue", self.can_first_consecutive_frame_value),
            Def::new(Com, "CP_CanFuncReqExtAddr", self.can_func_req_ext_addr),
            Def::new(Com, "CP_CanFuncReqFormat", self.can_func_req_format),
            Def::new(Com, "CP_CanFuncReqId", self.can_func_req_id),
            Def::new(Com, "CP_CanMaxNumWaitFrames", self.can_max_num_wait_frames),
            Def::new(Com, "CP_CanPhysReqExtAddr", self.can_phys_req_ext_addr),
            Def::new(Com, "CP_CanPhysReqFormat", self.can_phys_req_format),
            Def::new(Com, "CP_CanPhysReqId", self.can_phys_req_id),
            Def::new(Com, "CP_CanRespUSDTExtAddr", self.can_resp_usdt_ext_addr),
            Def::new(Com, "CP_CanRespUSDTFormat", self.can_resp_usdt_format),
            Def::new(Com, "CP_CanRespUSDTId", self.can_resp_usdt_id),
            Def::new(Com, "CP_CanRespUUDTExtAddr", self.can_resp_uudt_ext_addr),
            Def::new(Com, "CP_CanRespUUDTFormat", self.can_resp_uudt_format),
            Def::new(Com, "CP_CanRespUUDTId", self.can_resp_uudt_id),
            Def::new(Com, "CP_CANFDTxMaxDataLength", self.canfd_tx_max_data_length),
            Def::new(Com, "CP_MaxFirstFrameDataLength", self.max_first_frame_data_length),
            Def::new(Com, "CP_RequestAddrMode", self.request_addr_mode),
            Def::new(Com, "CP_SendRemoteFrame", self.send_remote_frame),

            // Error handling.
            Def::new(ErrHdl, "CP_RepeatReqCountTrans", self.repeat_req_count_trans),
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
                Def::new(UniqueId, "CP_CanRespUSDTExtAddr", self.can_resp_usdt_ext_addr),
                Def::new(UniqueId, "CP_CanRespUSDTFormat", self.can_resp_usdt_format),
                Def::new(UniqueId, "CP_CanRespUSDTId", self.can_resp_usdt_id),
            }
        })
    }
}
