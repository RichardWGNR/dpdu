use dpdu_wrapper_support::impl_configure_from_serde_json_map_for_com_param_stack;
use map_macro::hash_set;
use crate::types::pdu_com_param::stack::ComParamDefinitionStack;
use crate::types::pdu_com_param::table::{ComParamDefinition, ComParamDefinitionSet, ComParamDefinitionTable};

#[derive(Debug, Clone)]
pub struct RawCanApplicationStack {
    /// CP_ChangeSpeedCtrl
    ///
    /// Определяет, включен ли контроль изменения скорости передачи данных. Этот параметр
    /// указывает, будет ли система управлять изменением скорости передачи данных, например, в
    /// зависимости от текущих условий сети или других факторов.
    pub change_speed_ctrl: u32,

    /// CP_ChangeSpeedMessage
    ///
    /// Определяет сообщение, которое будет отправлено при изменении скорости передачи данных.
    /// Этот параметр указывает байтовую последовательность или данные, которые должны быть
    /// переданы для изменения скорости передачи в системе, например, в случае изменения режима
    /// работы интерфейса или канала связи.
    pub change_speed_message: Vec<u8>,

    /// CP_ChangeSpeedRate
    ///
    /// Определяет скорость изменения передачи данных. Этот параметр указывает, на какую
    /// скорость будет изменена передача данных, например, в процессе изменения условий сети или
    /// в ответ на запрос о изменении скорости работы канала связи.
    pub change_speed_rate: u32,

    pub change_speed_res_ctrl: u32,

    /// CP_ChangeSpeedTxDelay
    ///
    /// Определяет задержку передачи данных при изменении скорости. Этот параметр указывает
    /// время задержки, которое должно быть соблюдено перед началом передачи данных после
    /// изменения скорости. Используется для корректного синхронизирования передачи при переходе
    /// на новую скорость.
    pub change_speed_tx_delay: u32,

    /// CP_CyclicRespTimeout
    ///
    /// Определяет таймаут для ожидания циклического ответа в системе CAN или протоколе UDS
    /// (Unified Diagnostic Services). Этот таймаут указывает максимальное время, в течение
    /// которого система ожидает получения ответа на запрос или команду, прежде чем будет
    /// предпринята следующая попытка или выполнены действия в случае его отсутствия.
    pub cyclic_resp_timeout: u32,

    /// CP_EnablePerformanceTest
    ///
    /// Специальный параметр, используемый для включения или отключения измерения
    /// производительности канала связи между диагностическим приложением и ECU (или между
    /// приложением и VCI).
    pub enable_performance_test: u32,

    /// CP_Loopback
    ///
    /// Определяет, активирован ли режим петли (loopback) для передачи данных. Этот параметр
    /// указывает, будет ли система работать в режиме петли, при котором отправленные данные
    /// немедленно возвращаются обратно, что используется для тестирования и диагностики канала
    /// передачи данных или устройства.
    pub loopback: u32,

    /// CP_P2Max
    ///
    /// Определяет максимальное время второго этапа (P2) в протоколе UDS. Это время, которое
    /// система будет ожидать для завершения второго этапа обмена данными после отправки первого
    /// этапа. Может использоваться для настройки времени ожидания в случае, если система работает
    /// в условиях ограниченной скорости передачи или высокой нагрузки.
    pub p2_max: u32,

    /// CP_P2Min
    ///
    /// Минимальное время, через которое тестер (диагностическая программа) может ожидать ответ
    /// от ECU после отправки запроса.
    pub p2_min: u32,

    /// CP_P3Func
    ///
    /// Определяет время третьего этапа (P3) в функциональном режиме UDS. Этот параметр указывает
    /// максимальное время, которое система ожидает для завершения обработки запроса в случае,
    /// если устройство не может завершить операцию в более короткий срок. Обычно используется
    /// для операций, которые требуют длительной обработки, таких как программирование или сложные
    /// диагностики.
    pub p3_func: u32,

    /// CP_P3Phys
    ///
    /// Определяет время третьего этапа (P3) в физическом режиме UDS. Этот параметр указывает
    /// максимальное время, которое система ожидает для завершения операции в физическом режиме
    /// обмена данными, если устройство не может выполнить операцию быстрее. Он используется для
    /// операций, которые могут потребовать больше времени из-за физической задержки или сложности
    /// обработки данных.
    pub p3_phys: u32,

    /// CP_RepeatReqCountApp
    ///
    /// Определяет количество повторных попыток запроса для приложения в случае неудачи.
    /// Этот параметр указывает, сколько раз система будет повторно пытаться выполнить запрос,
    /// если предыдущие попытки не увенчались успехом, прежде чем считать операцию окончательно
    /// неудавшейся.
    pub repeat_req_count_app: u32,

    /// CP_SwCan_HighVoltage
    ///
    /// Определяет наличие или отсутствие высокого напряжения для шины SWCAN. Этот параметр
    /// указывает, используется ли высокое напряжение для работы с шиной SWCAN (Single Wire CAN).
    /// Обычно такой параметр применяется для конфигурации интерфейсов, поддерживающих работу с
    /// высоким напряжением на шине передачи данных.
    pub sw_can_high_voltage: u32,

    /// CP_TransmitIndEnable
    ///
    /// Определяет, включена ли индикаторная передача сообщений. Этот параметр указывает,
    /// будет ли система отправлять индикаторы или уведомления о начале передачи данных.
    /// Используется для уведомления системы или других устройств о начале или успешном завершении
    /// передачи сообщений.
    pub transmit_ind_enable: u32,
}

impl Default for RawCanApplicationStack {
    fn default() -> Self {
        Self {
            change_speed_ctrl: 0,
            change_speed_message: vec![],
            change_speed_rate: 0,
            change_speed_res_ctrl: 0,
            change_speed_tx_delay: 0,
            cyclic_resp_timeout: 0,
            enable_performance_test: 0,
            loopback: 0,
            p2_max: 50000,
            p2_min: 0,
            p3_func: 0,
            p3_phys: 0,
            repeat_req_count_app: 0,
            sw_can_high_voltage: 0,
            transmit_ind_enable: 0,
        }
    }
}

impl RawCanApplicationStack {
    pub fn with_change_speed_ctrl(mut self, value: bool) -> Self {
        self.change_speed_ctrl = value.then(|| 1).unwrap_or(0);
        self
    }
    pub fn with_change_speed_message(mut self, value: impl AsRef<[u8]>) -> Self {
        self.change_speed_message = value.as_ref().to_vec();
        self
    }

    pub fn with_change_speed_rate(mut self, value: impl Into<u32>) -> Self {
        self.change_speed_rate = value.into();
        self
    }

    pub fn with_change_speed_res_ctrl(mut self, value: impl Into<u32>) -> Self {
        self.change_speed_res_ctrl = value.into();
        self
    }

    pub fn with_change_speed_tx_delay(mut self, value: impl Into<u32>) -> Self {
        self.change_speed_tx_delay = value.into();
        self
    }

    pub fn with_enable_performance_test(mut self, value: bool) -> Self {
        self.enable_performance_test = value.then(|| 1).unwrap_or(0);
        self
    }

    pub fn with_loopback(mut self, value: bool) -> Self {
        self.loopback = value.then(|| 1).unwrap_or(0);
        self
    }

    pub fn with_p2_max(mut self, value: impl Into<u32>) -> Self {
        self.p2_max = value.into();
        self
    }

    pub fn with_p2_min(mut self, value: impl Into<u32>) -> Self {
        self.p2_min = value.into();
        self
    }

    pub fn with_p3_func(mut self, value: impl Into<u32>) -> Self {
        self.p3_func = value.into();
        self
    }

    pub fn with_p3_phys(mut self, value: impl Into<u32>) -> Self {
        self.p3_phys = value.into();
        self
    }

    pub fn with_repeat_req_count_app(mut self, value: impl Into<u32>) -> Self {
        self.repeat_req_count_app = value.into();
        self
    }

    pub fn with_sw_can_high_voltage(mut self, value: bool) -> Self {
        self.sw_can_high_voltage = value.then(|| 1).unwrap_or(0);
        self
    }

    pub fn with_transmit_ind_enable(mut self, value: bool) -> Self {
        self.transmit_ind_enable = value.then(|| 1).unwrap_or(0);
        self
    }
}

impl ComParamDefinitionStack for RawCanApplicationStack {
    impl_configure_from_serde_json_map_for_com_param_stack! {
        CP_ChangeSpeedCtrl: u32,
        CP_ChangeSpeedMessage: Vec<u8>,
        CP_ChangeSpeedRate: u32,
        CP_ChangeSpeedResCtrl: u32,
        CP_ChangeSpeedTxDelay: u32,
        CP_CyclicRespTimeout: u32,
        CP_EnablePerformanceTest: u32,
        CP_Loopback: u32,
        CP_P2Max: u32,
        CP_P2Min: u32,
        CP_P3Func: u32,
        CP_P3Phys: u32,
        CP_RepeatReqCountApp: u32,
        CP_SwCan_HighVoltage: u32,
        CP_TransmitIndEnable: u32
    }

    fn build_set(&self) -> ComParamDefinitionSet<ComParamDefinition> {
        use ComParamDefinition as Def;
        use dpdu_api_types::PduPc::{Com, ErrHdl, Timing as Time};

        ComParamDefinitionSet(hash_set! {
            // Timings.
            Def::new(Time, "CP_ChangeSpeedTxDelay", self.change_speed_tx_delay),
            Def::new(Time, "CP_CyclicRespTimeout", self.cyclic_resp_timeout),
            Def::new(Time, "CP_P2Max", self.p2_max),
            Def::new(Time, "CP_P2Min", self.p2_min),
            Def::new(Time, "CP_P3Func", self.p3_func),
            Def::new(Time, "CP_P3Phys", self.p3_phys),

            // Error handling.
            Def::new(ErrHdl, "CP_RepeatReqCountApp", self.repeat_req_count_app),

            // Com.
            Def::new(Com, "CP_ChangeSpeedCtrl", self.change_speed_ctrl),
            Def::new(Com, "CP_ChangeSpeedMessage", self.change_speed_message.clone()),
            Def::new(Com, "CP_ChangeSpeedRate", self.change_speed_rate),
            Def::new(Com, "CP_ChangeSpeedResCtrl", self.change_speed_res_ctrl),
            Def::new(Com, "CP_EnablePerformanceTest", self.enable_performance_test),
            Def::new(Com, "CP_Loopback", self.loopback),
            Def::new(Com, "CP_SwCan_HighVoltage", self.sw_can_high_voltage),
            Def::new(Com, "CP_TransmitIndEnable", self.transmit_ind_enable),
        })
    }

    fn build_table(&self) -> ComParamDefinitionTable<ComParamDefinition> {
        ComParamDefinitionTable::new()
    }
}