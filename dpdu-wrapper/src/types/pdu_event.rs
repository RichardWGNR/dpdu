use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use dpdu_api_types::{PduErrorEvt, PduInfo, PduStatus};
use crate::types::{PduCllHandle, PduCopHandle, PduModuleHandle};

#[derive(Debug, Clone)]
pub struct PduEvent {
    pub target: PduEventTarget,

    pub timestamp: u32,

    /// Желательно создавать через типаж [`Into<PduEventData>`].
    pub data: PduEventData
}

#[derive(Debug, Copy, Clone)]
pub enum PduEventTarget {
    System,
    Module(PduModuleHandle),
    ComLogicalLink(PduModuleHandle, PduCllHandle),
}

impl PduEventTarget {
    pub fn is_system(&self) -> bool {
        matches!(self, PduEventTarget::System)
    }

    pub fn is_module(&self) -> bool {
        matches!(self, PduEventTarget::Module(..))
    }

    pub fn is_com_logical_link(&self) -> bool {
        matches!(self, PduEventTarget::ComLogicalLink(..))
    }

    pub fn get_module_handle(&self) -> Option<PduModuleHandle> {
        match self {
            PduEventTarget::Module(h_mod) => Some(h_mod.clone()),
            PduEventTarget::ComLogicalLink(h_mod, ..) => Some(h_mod.clone()),
            _ => None,
        }
    }

    pub fn get_cll_handle(&self) -> Option<PduCllHandle> {
        match self {
            PduEventTarget::ComLogicalLink(_, h_cll) => Some(h_cll.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PduEventData {
    Status(PduStatusEvent),

    Result(PduResultEvent),

    Error(PduErrorEvent),

    Info(PduInfoEvent)
}

impl From<PduStatusEvent> for PduEventData {
    fn from(value: PduStatusEvent) -> Self {
        PduEventData::Status(value)
    }
}

impl From<PduResultEvent> for PduEventData {
    fn from(value: PduResultEvent) -> Self {
        PduEventData::Result(value)
    }
}

impl From<PduErrorEvent> for PduEventData {
    fn from(value: PduErrorEvent) -> Self {
        PduEventData::Error(value)
    }
}

impl From<PduInfoEvent> for PduEventData {
    fn from(value: PduInfoEvent) -> Self {
        PduEventData::Info(value)
    }
}

impl PduEventData {
    pub fn as_status(&self) -> Option<&PduStatusEvent> {
        match self {
            PduEventData::Status(v) => Some(v),
            _ => None
        }
    }

    pub fn as_result(&self) -> Option<&PduResultEvent> {
        match self {
            PduEventData::Result(v) => Some(v),
            _ => None
        }
    }

    pub fn as_error(&self) -> Option<&PduErrorEvent> {
        match self {
            PduEventData::Error(v) => Some(v),
            _ => None
        }
    }

    pub fn as_info(&self) -> Option<&PduInfoEvent> {
        match self {
            PduEventData::Info(v) => Some(v),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct PduStatusEvent(pub PduStatus);

impl Deref for PduStatusEvent {
    type Target = PduStatus;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PduStatusEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct PduResultEvent {
    pub rx_flags: PduResultEventRxFlags,

    pub unique_resp_identifier: u32,

    pub acceptance_id: u32,

    pub timestamp_flags: PduResultEventTimestampFlags,

    pub tx_msg_done_timestamp: u32,

    pub start_msg_timestamp: u32,

    /// Если [ComLogicalLink] была создана с флагом [RawMode], то данные включают:
    ///   - байты заголовка
    ///   - контрольную сумму
    ///   - байты данных сообщения
    ///   - дополнительные данные, если таковые имеются.
    pub data: Vec<u8>,

    pub extra_info_header: Option<Vec<u8>>,

    pub extra_info_footer: Option<Vec<u8>>
}

#[derive(Debug, Clone)]
pub struct PduResultEventRxFlags(Vec<u8>);

impl From<Vec<u8>> for PduResultEventRxFlags {
    fn from(value: Vec<u8>) -> Self {
        PduResultEventRxFlags(value)
    }
}

impl Deref for PduResultEventRxFlags {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PduResultEventRxFlags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PduResultEventRxFlags {
    /// Обнаружен ли CAN фрейм RTR.
    /// Первый байт содержит DLC.
    pub fn is_remote_frame(&self) -> bool {
        if let Some(byte) = self.get(0) {
            return (*byte & 0x80) != 0;
        }

        false
    }

    /// Указывает, что последовательная шина перешла на новую скорость.
    pub fn is_speed_change_event(&self) -> bool {
        if let Some(byte) = self.get(1) {
            return (*byte & 0x04) != 0;
        }
        false
    }

    /// Коммуникационные параметры класса Timing были изменены для текущего логического
    /// соединения.
    ///
    /// Этот флаг будет установлен только в том случае, если в текущем логическом соединении
    /// установлен коммуникационный параметр CP_ModifyTiming.
    pub fn is_ecu_timing_changed(&self) -> bool {
        if let Some(byte) = self.get(1) {
            return (*byte & 0x02) != 0;
        }
        false
    }

    /// По шине SW CAN было принято сообщения High Voltage.
    pub fn is_sw_can_high_voltage_msg(&self) -> bool {
        if let Some(byte) = self.get(1) {
            return (*byte & 0x01) != 0;
        }
        false
    }

    /// Формат CAN фрейма принятого по шине CAN.
    ///
    /// Действует только если текущее логическое соединение было создано с параметром [raw mode].
    pub fn is_can_29_bit_id(&self) -> bool {
        if let Some(byte) = self.get(3) {
            return (*byte & 0x80) != 0;
        }
        false
    }

    /// Было ли полученное сообщение обработано как сегментированное или нет.
    ///
    /// Если да, то информация сегмента будет вырезана из PDU Data.
    ///
    /// Действует только если текущее логическое соединение было создано с параметром [raw mode].
    pub fn is_can_segmentation(&self) -> bool {
        if let Some(byte) = self.get(3) {
            return (*byte & 0x40) != 0;
        }
        false
    }

    /// Была ли фактическая длина сообщения меньше 8 байт.
    ///
    /// Действует только для протокола ISO 15765, и только если текущее логическое соединение
    /// было создано с параметром [raw mode].
    pub fn is_iso_15765_padding_error(&self) -> bool {
        if let Some(byte) = self.get(3) {
            return (*byte & 0x10) != 0;
        }
        false
    }

    /// Индикация передачи.
    pub fn get_tx_status(&self) -> bool {
        if let Some(byte) = self.get(3) {
            return (*byte & 0x08) != 0;
        }
        false
    }

    /// SAE J2610 и SAE J1850 VPW. Получен ли индикатор разрыва.
    pub fn get_rx_break_status(&self) -> bool {
        if let Some(byte) = self.get(3) {
            return (*byte & 0x04) != 0;
        }
        false
    }

    /// Указывает на прием первого байта сообщения ISO 9141 или ISO 14230, или первого
    /// кадра многокадрового сообщения ISO 15765.
    pub fn is_start_of_message(&self) -> bool {
        if let Some(byte) = self.get(3) {
            return (*byte & 0x02) != 0;
        }
        false
    }

    pub fn get_tx_msg_type(&self) -> bool {
        if let Some(byte) = self.get(3) {
            return (*byte & 0x01) != 0;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct PduResultEventTimestampFlags(Vec<u8>);

impl From<Vec<u8>> for PduResultEventTimestampFlags {
    fn from(value: Vec<u8>) -> Self {
        PduResultEventTimestampFlags(value)
    }
}

impl Deref for PduResultEventTimestampFlags {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PduResultEventTimestampFlags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PduResultEventTimestampFlags {
    /// Указывает, что значение Transmit Done Timestamp в структуре
    /// PDU_RESULT_DATA является действительным.
    pub fn is_tx_msg_done_timestamp_indicator(&self) -> bool {
        if let Some(byte) = self.get(0) {
            return (*byte & 0x80) != 0;
        }
        false
    }

    /// Указывает, что значение Start Message Timestamp в структуре
    /// PDU_RESULT_DATA является действительным.
    pub fn is_start_msg_timestamp_indicator(&self) -> bool {
        if let Some(byte) = self.get(0) {
            return (*byte & 0x40) != 0;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct PduErrorEvent {
    pub code: PduErrorEvt,
    pub extra_code: u32,
}

impl Display for PduErrorEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PduErrorEvent: code={}, extra_code={}",
            self.code.as_ref(),
            self.extra_code
        )
    }
}

#[derive(Debug, Clone)]
pub struct PduInfoEvent {
    pub code: PduInfo,
    pub extra_code: u32
}

impl Display for PduInfoEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PduInfoEvent: code={}, extra_code={}",
            self.code.as_ref(),
            self.extra_code
        )
    }
}