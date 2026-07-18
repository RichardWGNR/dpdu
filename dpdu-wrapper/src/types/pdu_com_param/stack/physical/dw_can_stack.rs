use crate::types::pdu_com_param::stack::ComParamDefinitionStack;
use crate::types::pdu_com_param::table::{
    ComParamDefinition, ComParamDefinitionSet, ComParamDefinitionTable,
};
use dpdu_wrapper_support::impl_configure_from_serde_json_map_for_com_param_stack;
use map_macro::hash_set;

/// Стек возможных коммуникационных параметров для физического протокола DW CAN (ISO 11898-2).
///
/// Описания возможных параметров сгенерированы ChatGpt!
#[derive(Clone, Debug)]
pub struct DwCanStack {
    /// CP_Baudrate.
    ///
    /// Параметр CP_Baudrate указывает скорость обмена данными в сети CAN (например, 500 Kbps,
    /// 1 Mbps и т.д.).
    /// Это важно для правильной синхронизации всех устройств в сети CAN, поскольку все устройства
    /// должны работать с одинаковой скоростью для корректного обмена сообщениями.
    pub baudrate: u32,

    /// CP_BitSamplePoint.
    ///
    /// Определяет момент, когда осуществляется выборка (чтение) бита в
    /// битовом интервале. В контексте сети CAN (или CAN FD) битовый интервал — это время, в
    /// течение которого передается один бит данных, и выборка бита — это момент, когда контроллер
    /// сети считывает значение бита.
    pub bit_sample_point: u32,

    /// CP_CanBaudrateRecord.
    ///
    /// Используется для записи и хранения конфигурации скорости передачи данных в канале CAN или
    /// CAN FD. Это может включать как скорость базовой передачи данных (например, 125 Кбит/с,
    /// 500 Кбит/с, 1 Мбит/с для стандартного CAN), так и скорости для CAN FD, которые могут быть
    /// значительно выше.
    pub can_baudrate_record: Vec<u32>,

    /// CP_CANFDBaudrate.
    ///
    /// Этот параметр устанавливает скорость передачи данных для канала CAN FD в сети.
    /// В отличие от стандартного протокола CAN, CAN FD поддерживает переменную скорость передачи
    /// данных, что позволяет более эффективно передавать большие объемы данных. CP_CANFDBaudrate
    /// указывает, с какой скоростью данные будут передаваться по каналу CAN FD.
    pub canfd_baudrate: u32,

    /// CP_CANFDBitSamplePoint.
    ///
    /// Этот параметр определяет точку, на которой осуществляется выборка бита в одном битовом
    /// интервале при передаче данных в сети CAN FD. В контексте протокола CAN,
    /// битовая выборка — это момент времени в каждом интервале, когда система "считывает" значение
    /// бита из сигнала на шине.
    pub canfd_bit_sample_point: u32,

    /// CP_CANFDSyncJumpWidth.
    ///
    /// Устанавливает максимальную величину изменения синхронизации (так называемое "прыжок
    /// синхронизации") между двумя последовательными битами в процессе передачи данных по сети
    /// CAN FD.
    ///
    /// Это необходимо для корректной работы на высоких скоростях передачи данных, где требуется
    /// точная синхронизация битов для предотвращения ошибок.
    pub canfd_sync_jump_width: u32,

    /// CP_ListenOnly.
    ///
    /// Управляет возможностью устройства работать в режиме, когда оно только слушает сообщения на
    /// шине и не участвует в передаче. Устройство, находящееся в Listen-Only mode, может
    /// мониторить сеть, принимать данные, но не будет вмешиваться в обмен сообщениями, не
    /// отправляя собственных сообщений на шину.
    pub listen_only: u32,

    /// CP_SamplesPerBit.
    ///
    /// Определяет, сколько раз в течение одного битового интервала будет выполнена выборка данных.
    /// Это важный параметр, который влияет на точность синхронизации и качество передачи данных,
    /// особенно при работе на высоких скоростях передачи.
    pub samples_per_bit: u32,

    /// CP_SyncJumpWidth.
    ///
    /// Контролирует максимальный размер корректировки времени синхронизации между битами при
    /// передаче данных. Это настройка важна для правильной синхронизации устройств в сети,
    /// особенно при работе с высокоскоростными системами, такими как CAN FD.
    pub sync_jump_width: u32,

    /// CP_TerminationType.
    ///
    /// Указывает тип терминации канала, то есть, как будет организовано завершение линии передачи
    /// данных в сети CAN или CAN FD. Терминация важна для предотвращения искажений сигнала из-за
    /// отражений, особенно на длинных кабелях или при высокоскоростных передачах данных.
    ///
    /// Значения:
    ///
    ///   0: Без терминации.
    ///
    ///   1: AC termination.
    ///
    ///   2: 60 Ом.
    ///
    ///   3: 120 Ом.
    ///
    ///   4: SWCAN
    pub termination_type: u32,
}

impl Default for DwCanStack {
    fn default() -> Self {
        Self {
            baudrate: 500000,
            canfd_baudrate: 0,
            canfd_bit_sample_point: 80,
            canfd_sync_jump_width: 15,
            bit_sample_point: 80,
            can_baudrate_record: vec![],
            listen_only: 0,
            samples_per_bit: 0,
            sync_jump_width: 15,
            termination_type: 0,
        }
    }
}

impl DwCanStack {
    pub fn with_baudrate(mut self, rate: impl Into<u32>) -> Self {
        self.baudrate = rate.into();
        self
    }

    pub fn with_canfd_baudate(mut self, rate: impl Into<u32>) -> Self {
        self.canfd_baudrate = rate.into();
        self
    }

    pub fn with_canfd_bit_sample_point(mut self, point: impl Into<u32>) -> Self {
        self.canfd_bit_sample_point = point.into();
        self
    }

    pub fn with_canfd_sync_jump_width(mut self, width: impl Into<u32>) -> Self {
        self.canfd_sync_jump_width = width.into();
        self
    }

    pub fn with_bit_sample_point(mut self, point: impl Into<u32>) -> Self {
        self.bit_sample_point = point.into();
        self
    }

    pub fn with_can_baudrate_record(mut self, record: Vec<u32>) -> Self {
        self.can_baudrate_record = record;
        self
    }

    pub fn with_listen_only(mut self, status: bool) -> Self {
        self.listen_only = if status { 1 } else { 0 };
        self
    }

    pub fn with_samples_per_bit(mut self, samples: impl Into<u32>) -> Self {
        self.samples_per_bit = samples.into();
        self
    }

    pub fn with_sync_jump_width(mut self, width: impl Into<u32>) -> Self {
        self.sync_jump_width = width.into();
        self
    }

    pub fn with_termination_type(mut self, typ: impl Into<u32>) -> Self {
        self.termination_type = typ.into();
        self
    }
}

impl ComParamDefinitionStack for DwCanStack {
    impl_configure_from_serde_json_map_for_com_param_stack! {
        CP_Baudrate: u32,
        CP_CANFDBaudrate: u32,
        CP_CANFDBitSamplePoint: u32,
        CP_CANFDSyncJumpWidth: u32,
        CP_BitSamplePoint: u32,
        CP_CanBaudrateRecord: Vec<u32>,
        CP_ListenOnly: u32,
        CP_SamplesPerBit: u32,
        CP_SyncJumpWidth: u32,
        CP_TerminationType: u32
    }

    fn build_set(&self) -> ComParamDefinitionSet<ComParamDefinition> {
        use crate::types::pdu_com_param::table::ComParamDefinition as Def;
        use dpdu_api_types::PduPc::BusType;

        ComParamDefinitionSet(hash_set! {
            // Bus type.
            Def::new(BusType, "CP_Baudrate", self.baudrate),
            Def::new(BusType, "CP_BitSamplePoint", self.bit_sample_point),
            Def::new(BusType, "CP_CANFDBaudrate", self.canfd_baudrate),
            Def::new(BusType, "CP_CANFDBitSamplePoint", self.canfd_bit_sample_point),
            Def::new(BusType, "CP_CANFDSyncJumpWidth", self.canfd_sync_jump_width),
            //Def::new(BusType, "CP_CanBaudrateRecord", (self.can_baudrate_record.clone(), 255)),
            Def::new(BusType, "CP_CanBaudrateRecord", self.can_baudrate_record.clone()),
            Def::new(BusType, "CP_ListenOnly", self.listen_only),
            Def::new(BusType, "CP_SamplesPerBit", self.samples_per_bit),
            Def::new(BusType, "CP_SyncJumpWidth", self.sync_jump_width),
            Def::new(BusType, "CP_TerminationType", self.termination_type)
        })
    }

    fn build_table(&self) -> ComParamDefinitionTable<ComParamDefinition> {
        ComParamDefinitionTable::new()
    }
}
