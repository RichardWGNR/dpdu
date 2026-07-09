use crate::types::pdu_resource::{BusSource, ProtocolSource, TargetPin};
use crate::types::{PduCllHandle, PduModuleHandle, PduObjectId};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct PduComLogicalLink {
    pub(crate) h_mod: PduModuleHandle,

    pub(crate) h_cll: PduCllHandle,

    pub(crate) create_type: CllCreateType,

    pub(crate) create_flags: CllCreateFlags,
}

impl PduComLogicalLink {
    pub fn get_module_handle(&self) -> PduModuleHandle {
        self.h_mod.clone()
    }

    pub fn get_cll_handle(&self) -> PduCllHandle {
        self.h_cll.clone()
    }

    pub fn get_create_type(&self) -> &CllCreateType {
        &self.create_type
    }

    pub fn get_create_flags(&self) -> &CllCreateFlags {
        &self.create_flags
    }
}

#[derive(Debug, Clone)]
pub enum CllCreateType {
    /// ComLogicalLink will be created by resource ID.
    ///
    /// Not recommended.
    ResourceId(PduObjectId),

    /// ComLogicalLink will be created by
    ///  - bus type ID
    ///  - protocol ID
    ///  - information about the pins on VCI (can't be empty).
    ///
    /// Recommended.
    ResourceData {
        bus: BusSource,
        protocol: ProtocolSource,
        pins: Vec<TargetPin>,
    },
}

/// Вспомогательная структура, используемая в функции [`PduWrapper::create_com_logical_link()`] при
/// создании "логической связи".
/// См. ISO 22900-2, глава D.2.3, таблица D.6.
#[derive(Debug, Clone)]
pub struct CllCreateFlags {
    /// Byte 0, bit 7.
    ///
    /// Обеспечивает возможность передачи всех принятых сообщений без изменений по каналу связи
    /// (переданных и принятых).
    ///
    /// Эта функция зависит от протокола!
    ///
    /// [FALSE]: указывает, что API D-PDU будет удалять байты заголовка и контрольные суммы перед
    /// возвратом (TxFlag ENABLE_EXTRA_INFO может быть использован для получения дополнительной
    /// информации о заголовке/футере сообщения).
    ///
    /// [TRUE]: указывает, что байты заголовка и контрольные суммы будут оставлены в возвращаемом
    /// элементе результата
    pub raw_mode: bool,

    /// Byte 0, bit 6.
    ///
    /// API D-PDU будет создавать контрольную сумму для передачи сообщений.
    ///
    /// Этот флаг игнорируется, если для [raw_mode] установлено значение [false].
    pub checksum_mode: bool,

    /// Byte 3, bit 0.
    ///
    /// Действительно только для Softing D-PDU-API (не является частью ISO22900-2)!
    ///
    /// Создает "мониторную" связь при вызове PDUCreateComLogicalLink, а не логическую связь.
    ///
    /// Используется только в протоколах:
    ///   - ISO_11898_RAW
    ///   - ISO_14230_3_on_ISO_14230_2
    ///
    /// И только если для [raw_mode] установлено значение false.
    pub monitor_mode: bool,
}

impl Display for CllCreateFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "raw_mode={:?}, checksum_mode={:?}, monitor_mode={:?}",
            self.raw_mode, self.checksum_mode, self.monitor_mode
        )
    }
}

impl CllCreateFlags {
    pub fn new_with_recommended() -> Self {
        // Объяснение почему лучше иметь по умолчанию включенный режим "checksum":
        //
        // Природа D-PDU API заключается в том, чтобы управлять протоколами транспортного средства.
        // Поэтому D-PDU API по своей природе делает контрольную сумму.
        //
        // Но если вы, например, хотите получить больше информации о глубоких слоях,
        // вы можете включить режим RawMode.
        //
        // Многие удивляются, что, например, протоколы K-line больше не работают.
        //
        // Это происходит потому, что режим "Checksum" работает только тогда, когда
        // включён режим "Raw".
        //
        // Так что, если вы хотите включить режим "Raw" только для некоторых протоколов, вам так
        // же нужно включить режим "Checksum", чтобы всё работало, как и раньше.
        //
        // Поэтому лучше, чтобы режим "Checksum" был включён по умолчанию.

        Self {
            raw_mode: false,
            checksum_mode: true,
            monitor_mode: false,
        }
    }

    pub(crate) fn zero_byte(&self) -> u8 {
        let mut b = 0;

        if self.raw_mode {
            b |= 0x8;
        }

        if self.checksum_mode {
            b |= 0x4;
        }

        b
    }

    pub(crate) fn third_byte(&self) -> u8 {
        let mut b = 0;

        if self.monitor_mode {
            b |= 0x01;
        }

        b
    }

    /// Рассчитывает байтовый массив с учётом используемых режимов.
    pub(crate) fn get_pdu_flag_data(&self) -> [u8; 4] {
        [self.zero_byte(), 0, self.third_byte(), 0]
    }
}
