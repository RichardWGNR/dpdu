#[derive(Debug, Clone)]
pub struct PduComPrimiviteParams {
    /// Время цикла в мс для циклической операции отправки или время задержки для PDU_COPT_DELAY.
    pub time: u32,

    /// Количество выполняемых циклов отправки.
    pub send_cycles: SendCycles,

    /// Количество выполняемых циклов приёма.
    pub receive_cycles: ReceiveCycles,

    /// See the [`ComParamBuffer`].
    pub temp_param_update: ComParamBuffer,

    pub tx_flag: TransmitFlags,

    pub expected_responses: Vec<ExpectedResponse>,
}

impl Default for PduComPrimiviteParams {
    fn default() -> Self {
        Self {
            time: 0,
            send_cycles: SendCycles::default(),
            receive_cycles: ReceiveCycles::default(),
            temp_param_update: ComParamBuffer::Active,
            tx_flag: TransmitFlags::default(),
            expected_responses: vec![ExpectedResponse {
                response_type: ResponseType::Positive,
                acceptance_id: 1,
                mask_data: MaskData::default(),
                unique_response_ids: vec![],
            }],
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransmitFlags {
    /// Подавление положительных ответов (часть ODX).
    ///
    /// Применимо для: ISO 15765-3/ISO14229-3.
    pub suppress_positive_response: bool,

    /// Добавлять дополнительную информацию к результирующим данным.
    /// Эта информация может быть полезна для отладки ответов от ECU.
    pub enable_extra_info: bool,

    /// Используется для уменьшения времени программирования, если приложение знает, что
    /// будет получен только один ответ.
    ///
    /// Не влияет на тайминги ответов на функциональные запросы.
    ///
    /// Действует только в сочетании с флагом [raw mode] при создании ComLogicalLink.
    ///
    /// | Значение | Описание |
    /// |----------|-------------------------------|
    /// | true | Время передачи сообщений интерфейса, как указано в стандарте ISO 14230 (по умолчанию). |
    /// | false | После получения ответа на физический запрос время ожидания сокращается до P3_MIN. |
    pub wait_p3_min_only: bool,

    /// Тип CAN идентификатора сообщения для ISO 11898, SAE J1939 и ISO 15765.
    ///
    /// Действует только в сочетании с флагом [raw mode] при создании ComLogicalLink.
    ///
    /// CAN ID содержится в первых 4-х байтах PDU Data.
    ///
    /// | Значение | Описание |
    /// |----------|----------|
    /// | true | 29-bit |
    /// | false | 11-bit (по умолчанию) |
    pub can_29_bit: bool,

    /// Метод адресации ISO 15765-2.
    ///
    /// Определяет, будет ли расширенный адрес CAN содержаться в байте, следующем за CAN ID в
    /// данных PDU.
    ///
    /// Действует только в сочетании с флагом [raw mode] при создании ComLogicalLink.
    ///
    /// | Значение | Описание |
    /// |----------|----------|
    /// | true | используется расширенная адресация |
    /// | false | нет расширенной адресации (по умолчанию) |
    pub iso_15765_addr_type: bool,

    /// Дополнение (padding) фреймов для ISO 15765-2.
    ///
    /// Действует только в сочетании с флагом [raw mode] при создании ComLogicalLink.
    ///
    /// | Значение | Описание |
    /// |----------|----------|
    /// | true | дополнять CAN сообщение до его DLC, используя ComParam CP_CanFillerByte |
    /// | false | дополнение отключено (по умолчанию) |
    pub iso_15765_frame_pad: bool,
}

impl Default for TransmitFlags {
    fn default() -> Self {
        Self {
            suppress_positive_response: false,
            enable_extra_info: false,
            wait_p3_min_only: false,
            can_29_bit: false,
            iso_15765_addr_type: false,
            iso_15765_frame_pad: false,
        }
    }
}

impl TransmitFlags {
    pub(crate) fn zero_byte(&self) -> u8 {
        let mut accumulator = 0;

        match self.suppress_positive_response {
            true => accumulator |= 0x40,
            false => accumulator &= 0xBF,
        }

        match self.enable_extra_info {
            true => accumulator |= 0x20,
            false => accumulator &= 0xDF,
        }

        accumulator
    }

    pub(crate) fn second_byte(&self) -> u8 {
        let mut accumulator = 0;

        match self.wait_p3_min_only {
            true => accumulator |= 0x02,
            false => accumulator &= 0xFD,
        }

        match self.can_29_bit {
            true => accumulator |= 0x01,
            false => accumulator &= 0xFE,
        }

        accumulator
    }

    pub(crate) fn third_byte(&self) -> u8 {
        let mut accumulator = 0;

        match self.iso_15765_addr_type {
            true => accumulator |= 0x80,
            false => accumulator &= 0x7F,
        }

        match self.iso_15765_frame_pad {
            true => accumulator |= 0x40,
            false => accumulator &= 0xBF,
        }

        accumulator
    }

    pub(crate) fn get_pdu_flag_data(&self) -> [u8; 4] {
        [self.zero_byte(), 0, self.second_byte(), self.third_byte()]
    }
}

#[derive(Debug, Clone)]
pub struct ExpectedResponse {
    pub response_type: ResponseType,

    /// ID assigned by application to be returned in PDU_RESULT_DATA,
    /// which indicates which expected response matched
    pub acceptance_id: u32,

    pub mask_data: MaskData,

    /// Массив уникальных идентификаторов ответов может использоваться, если ожидаемый ответ
    /// появляется только для определенных уникальных идентификаторов ответов.
    ///
    /// Такая ситуация может возникнуть в случае функциональной адресации,
    /// когда возможные ответы не являются общими для всех ЭБУ.
    ///
    /// Количество уникальных идентификаторов ответа может быть равно 0.
    /// В этом случае массив pUniqueRespIds не используется, и все ответы с любым уникальным
    /// идентификатором ответа учитываются при попытке сопоставить фактические данные ответа
    /// с ожидаемыми данными ответа.
    pub unique_response_ids: Vec<u32>,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, strum::AsRefStr)]
pub enum ResponseType {
    Positive = 0,
    Negative = 1,
}

/// При фильтрации по приему API D-PDU пытается сопоставить байты данных полученного ответа с
/// байтами шаблона ожидаемого ответа (всегда с учетом байтов маски).
/// Количество байтов данных в полученном ответе может отличаться от количества байтов
/// маски и шаблона в ожидаемом ответе.
///
/// Фильтрация приема использует следующие правила:
///
///   - Если количество полученных байтов данных меньше NumMaskPatternBytes, то ответ не
///     соответствует ожидаемому.
///
///   - Если количество полученных байтов данных равно NumMaskPatternBytes, все байты данных
///     сравниваются с байтами данных шаблона.
///
///   - Если количество полученных байтов данных превышает NumMaskPatternBytes, только начальные
///     байты данных полученного ответа сравниваются со всеми байтами данных шаблона ожидаемого
///     ответа. Все последующие байты данных в полученном ответе являются «безразличными».
#[derive(Debug, Clone, Default)]
pub struct MaskData {
    pub(crate) mask: Vec<u8>,
    pub(crate) pattern: Vec<u8>,
}

impl MaskData {
    pub fn new(mask: &[u8], pattern: &[u8]) -> Option<Self> {
        if mask.len() != pattern.len() {
            return None;
        }

        Some(Self {
            mask: mask.to_vec(),
            pattern: pattern.to_vec(),
        })
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.mask.len(), self.pattern.len());
        self.mask.len()
    }

    pub fn get_mask(&self) -> &[u8] {
        assert_eq!(self.mask.len(), self.pattern.len());
        &self.mask
    }

    pub fn get_pattern(&self) -> &[u8] {
        assert_eq!(self.mask.len(), self.pattern.len());
        &self.pattern
    }
}

#[derive(Debug, Clone)]
pub enum SendCycles {
    Normal(u32),
    Infinite,
}

impl Default for SendCycles {
    fn default() -> Self {
        SendCycles::Normal(1)
    }
}

impl SendCycles {
    pub fn to_i32(&self) -> i32 {
        match self {
            SendCycles::Normal(v) => {
                i32::try_from(*v).expect("SendCycles value is too large for i32: {v}")
            }
            SendCycles::Infinite => -1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReceiveCycles {
    Normal(u32),

    Infinite,

    /// When a ComPrimitive is configured with NumSendCycles set to 1 and NumReceiveCycles set to -2
    /// (IS-MULTIPLE), the MVCI protocol module expects responses from one or more ECUs. Until a specific
    /// timeout expires, the MVCI protocol module receives responses and tries to match the Unique Response ID for
    /// each response (see the following subclauses for details). The Unique Response Id is saved and then returned
    /// in a result item when the payload data is matched to a ComPrimitive expected response. The application
    /// retrieves the result items from the ComLogicalLink's Event Queue (details of event notification are not shown
    /// in the diagram). When processing the result data, the application is able to assign the data to a certain ECU
    /// via the Unique Response ID.
    Multiple,
}

impl Default for ReceiveCycles {
    fn default() -> Self {
        ReceiveCycles::Normal(1)
    }
}

impl ReceiveCycles {
    pub fn to_i32(&self) -> i32 {
        match self {
            ReceiveCycles::Normal(v) => {
                i32::try_from(*v).expect("ReceiveCycles value is too large for i32: {v}")
            }
            ReceiveCycles::Infinite => -1,
            ReceiveCycles::Multiple => -2,
        }
    }
}

/// Temporary ComParam settings for the ComPrimitive.
#[repr(u32)]
#[derive(Debug, Copy, Clone, strum::AsRefStr)]
pub enum ComParamBuffer {
    /// Do not use temporary ComParams for this ComPrimitive. The
    /// ComPrimitive shall attach the “Active” ComParam buffer to the
    /// ComPrimitive. This buffer shall be in effect for the ComPrimitive until it is
    /// finished. The ComParams for the ComPrimitive will not change even if
    /// the “Active” buffer is modified by a subsequent ComPrimitive type of
    /// PDU_COPT_UPDATEPARAM
    Active = 0,

    /// Use temporary ComParams for this ComPrimitive; The
    /// ComPrimitive shall attach the ComParam “Working” buffer to the
    /// ComPrimitive. This buffer shall be in effect for the ComPrimitive until it is
    /// finished. The ComParams for the ComPrimitive will not change even if
    /// the “Active” or “Working” buffers are modified by any subsequent calls to
    /// PDUSetComParam.
    Working = 1,
}
