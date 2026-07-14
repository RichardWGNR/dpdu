use crate::types::pdu_com_param::stack::ComParamDefinitionStack;
use crate::types::pdu_com_param::table::{ComParamDefinition, ComParamDefinitionSet, ComParamDefinitionTable};
use dpdu_wrapper_support::impl_configure_from_serde_json_map_for_com_param_stack;
use map_macro::hash_set;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct KwpStack {
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

    /// CP_ChangeSpeedResCtrl
    ///
    /// Должен ли VCI ожидать ответ от ECU после команды изменения скорости (например, 0x10
    /// или 0x85) перед переключением на новую скорость.
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

    /// CP_ModifyTiming
    ///
    /// Используется для изменения стандартных таймингов в системе. Определяет, нужно ли
    /// модифицировать параметры времени, такие как время отклика или время передачи данных.
    /// При ненулевом значении активирует изменение временных интервалов в процессе передачи
    /// сообщений, например, в системе CAN или UDS.
    pub modify_timing: u32,

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

    /// CP_P2Star
    ///
    /// Определяет максимальное время для второго этапа (P2*) в протоколе UDS, которое может быть
    /// увеличено в случае, если обмен данными между устройствами занимает больше времени. Этот
    /// параметр используется для настройки времени ожидания, когда система должна ожидать
    /// завершение второго этапа передачи данных, даже если процесс занимает больше времени, чем
    /// обычно.
    pub p2_star: u32,

    /// CP_P3Min
    ///
    /// Минимальное время, которое диагностический тестер (например, диагностическая программа)
    /// должен выждать после потери связи, прежде чем повторно начать диагностику.
    pub p3_min: u32,

    /// CP_RC21CompletionTimeout
    ///
    /// Определяет время ожидания завершения обработки запроса для ошибки RC21. Этот параметр
    /// указывает максимальное время, которое система будет ожидать для завершения обработки
    /// операции, связанной с ошибкой RC21, прежде чем считать операцию неудавшейся и перейти к
    /// следующей.
    pub rc21_completion_timeout: u32,

    /// CP_RC21Handling
    ///
    /// Определяет способ обработки ошибки RC21 в системе. Этот параметр указывает, как система
    /// будет реагировать на ошибку RC21, например, какой метод или процедура будет использована
    /// для устранения или обработки ошибки. Может включать действия, такие как повторная попытка
    /// запроса, переход в безопасное состояние или выполнение других предустановленных операций.
    pub rc21_handling: u32,

    /// CP_RC21RequestTime
    ///
    /// Определяет время ожидания запроса для ошибки RC21. Этот параметр указывает максимальное
    /// время, которое система будет ожидать перед тем, как начать обработку или предпринять
    /// действия в случае возникновения ошибки RC21. Используется для контроля времени, в течение
    /// которого система будет пытаться выполнить запрос, прежде чем считать его неудавшимся.
    pub rc21_request_time: u32,

    /// CP_RC23CompletionTimeout
    ///
    /// Определяет время ожидания завершения обработки запроса для ошибки RC23. Этот параметр
    /// указывает максимальное время, которое система будет ожидать для завершения операции,
    /// связанной с ошибкой RC23, прежде чем считать операцию неудавшейся и перейти к следующей.
    pub rc23_completion_timeout: u32,

    /// CP_RC23Handling
    ///
    /// Определяет способ обработки ошибки RC23 в системе. Этот параметр указывает, как система
    /// будет реагировать на ошибку RC23, например, что делать в случае её возникновения: повторить
    /// попытку, перейти в безопасное состояние или выполнить другие действия в соответствии с
    /// заранее заданной логикой обработки ошибок.
    pub rc23_handling: u32,

    /// CP_RC23RequestTime
    ///
    /// Определяет время ожидания запроса для ошибки RC23. Этот параметр указывает максимальное
    /// время, в течение которого система будет пытаться выполнить запрос или операцию, связанный
    /// с ошибкой RC23, прежде чем считать его неудавшимся и перейти к следующей попытке или
    /// операции.
    pub rc23_request_time: u32,

    /// CP_RC78CompletionTimeout
    ///
    /// Определяет время ожидания завершения обработки запроса для ошибки RC78. Этот параметр
    /// указывает максимальное время, которое система будет ожидать для завершения операции,
    /// связанной с ошибкой RC78, прежде чем считать операцию неудавшейся и перейти к следующей.
    pub rc78_completion_timeout: u32,

    /// CP_RC78Handling
    ///
    /// Определяет способ обработки ошибки RC78 в системе. Этот параметр указывает, как система
    /// будет реагировать на ошибку RC78, например, что делать в случае её возникновения: повторить
    /// запрос, использовать альтернативные методы или выполнить другие действия в зависимости от
    /// конфигурации обработки ошибок.
    pub rc78_handling: u32,

    /// CP_RCByteOffset
    ///
    /// Определяет смещение байта для ошибки RC в системе. Этот параметр указывает, с какого
    /// байта следует начинать обработку данных или сообщений, связанных с ошибкой RC.
    /// Используется для корректной обработки ошибок, когда необходимо указать точку начала в
    /// передаваемых данных.
    pub rc_byte_offset: u32,

    /// CP_RepeatReqCountApp
    ///
    /// Определяет количество повторных попыток запроса для приложения в случае неудачи.
    /// Этот параметр указывает, сколько раз система будет повторно пытаться выполнить запрос,
    /// если предыдущие попытки не увенчались успехом, прежде чем считать операцию окончательно
    /// неудавшейся.
    pub repeat_req_count_app: u32,

    /// CP_StartMsgIndEnable
    ///
    /// Определяет, включена ли индикаторная передача стартового сообщения. Этот параметр
    /// указывает, будет ли система отправлять специальное сообщение или сигнал при старте
    /// определенной операции или процесса. Используется для уведомления системы или других
    /// устройств о начале выполнения задачи или передачи данных.
    pub start_msg_ind_enable: u32,

    /// CP_SuspendQueueOnError
    ///
    /// Определяет, нужно ли при ошибке приостанавливать очередь сообщений. Этот параметр
    /// указывает, будет ли система приостанавливать обработку дальнейших сообщений в очереди,
    /// если возникает ошибка, например, при обработке запроса или передачи данных. Используется
    /// для предотвращения отправки последующих сообщений, пока ошибка не будет устранена.
    pub suspend_queue_on_error: u32,

    /// CP_SwCan_HighVoltage
    ///
    /// Определяет наличие или отсутствие высокого напряжения для шины SWCAN. Этот параметр
    /// указывает, используется ли высокое напряжение для работы с шиной SWCAN (Single Wire CAN).
    /// Обычно такой параметр применяется для конфигурации интерфейсов, поддерживающих работу с
    /// высоким напряжением на шине передачи данных.
    pub sw_can_high_voltage: u32,

    /// CP_TesterPresentAddrMode
    ///
    /// Определяет режим адресации для запроса о присутствии тестера. Этот параметр указывает,
    /// как будет осуществляться адресация при отправке запроса на присутствие диагностического
    /// устройства (тестера) в сети. Могут быть использованы различные режимы, такие как адресация
    /// по физическому или логическому адресу устройства.
    pub tester_present_addr_mode: u32,

    /// CP_TesterPresentExpNegResp
    ///
    /// Определяет ожидаемый отрицательный ответ от тестера на запрос о его присутствии.
    /// Этот параметр указывает байтовую последовательность, которая должна быть получена,
    /// если тестер не присутствует или не отвечает на запрос о его наличии в сети.
    pub tester_present_exp_neg_resp: Vec<u8>,

    /// CP_TesterPresentExpPosResp
    ///
    /// Определяет ожидаемый положительный ответ от тестера на запрос о его присутствии. Этот
    /// параметр указывает байтовую последовательность, которая должна быть получена в случае
    /// успешного подтверждения присутствия диагностического устройства (тестера) в сети.
    pub tester_present_exp_pos_resp: Vec<u8>,

    /// CP_TesterPresentHandling
    ///
    /// Определяет способ обработки запроса на наличие тестера. Этот параметр указывает, как
    /// система будет реагировать на запрос о присутствии диагностического устройства (тестера).
    /// Может включать действия, такие как отправка подтверждения о наличии тестера, игнорирование
    /// запроса или выполнение других операций в зависимости от конфигурации системы.
    pub tester_present_handling: u32,

    /// CP_TesterPresentMessage
    ///
    /// Определяет сообщение, которое система будет отправлять при запросе о присутствии тестера.
    /// Этот параметр указывает конкретные данные или байтовую последовательность, которые будут
    /// переданы в ответ на запрос о присутствии диагностического устройства (тестера) в сети.
    pub tester_present_message: Vec<u8>,

    /// CP_TesterPresentReqRsp
    ///
    /// Определяет, будет ли система отправлять ответ на запрос о присутствии тестера. Этот
    /// параметр указывает, нужно ли системе отвечать на запросы о присутствии диагностического
    /// устройства (тестера) в сети. Значение может указывать на включение или выключение ответа,
    /// в зависимости от конфигурации работы системы с диагностическими устройствами.
    pub tester_present_req_rsp: u32,

    /// CP_TesterPresentSendType
    ///
    /// Определяет тип сообщения, которое система будет отправлять при запросе о присутствии
    /// тестера. Этот параметр указывает, какой формат или тип данных будет использоваться для
    /// ответа на запрос о присутствии диагностического устройства (тестера), например, стандартный
    /// или расширенный тип сообщения.
    pub tester_present_send_type: u32,

    /// CP_TesterPresentTime
    ///
    /// Определяет время, в течение которого система будет ожидать присутствие тестера. Этот
    /// параметр указывает максимальное время, в течение которого система будет ожидать ответа
    /// от диагностического устройства (тестера) после запроса на его присутствие.
    pub tester_present_time: u32,

    /// CP_TransmitIndEnable
    ///
    /// Определяет, включена ли индикаторная передача сообщений. Этот параметр указывает,
    /// будет ли система отправлять индикаторы или уведомления о начале передачи данных.
    /// Используется для уведомления системы или других устройств о начале или успешном завершении
    /// передачи сообщений.
    pub transmit_ind_enable: u32
}

impl Default for KwpStack {
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
            modify_timing: 0,

            p2_max: 50_000,
            p2_min: 25_000,
            p2_star: 5_000_000,
            p3_min: 55_000,

            rc21_completion_timeout: 1_300_000,
            rc21_handling: 0,
            rc21_request_time: 0,
            rc23_completion_timeout: 0,
            rc23_handling: 0,
            rc23_request_time: 0,
            rc78_completion_timeout: 25_000_000,
            rc78_handling: 0,
            rc_byte_offset: 0xFFFF_FFFF,

            repeat_req_count_app: 0,
            start_msg_ind_enable: 0,
            suspend_queue_on_error: 0,
            sw_can_high_voltage: 0,

            tester_present_addr_mode: 0,
            tester_present_exp_neg_resp: vec![0x7F, 0x3E],
            tester_present_exp_pos_resp: vec![0x7E],
            tester_present_handling: 1,
            tester_present_message: vec![0x3E],
            tester_present_req_rsp: 1,
            tester_present_send_type: 1,
            tester_present_time: 2_000_000,

            transmit_ind_enable: 0,
        }
    }
}

impl ComParamDefinitionStack for KwpStack {
    impl_configure_from_serde_json_map_for_com_param_stack! {
        CP_ChangeSpeedCtrl: u32,
        CP_ChangeSpeedMessage: Vec<u8>,
        CP_ChangeSpeedRate: u32,
        CP_ChangeSpeedResCtrl: u32,
        CP_ChangeSpeedTxDelay: u32,
        CP_CyclicRespTimeout: u32,
        CP_EnablePerformanceTest: u32,
        CP_Loopback: u32,
        CP_ModifyTiming: u32,
        CP_P2Max: u32,
        CP_P2Min: u32,
        CP_P2Star: u32,
        CP_P3Min: u32,
        CP_RC21CompletionTimeout: u32,
        CP_RC21Handling: u32,
        CP_RC21RequestTime: u32,
        CP_RC23CompletionTimeout: u32,
        CP_RC23Handling: u32,
        CP_RC23RequestTime: u32,
        CP_RC78CompletionTimeout: u32,
        CP_RC78Handling: u32,
        CP_RCByteOffset: u32,
        CP_RepeatReqCountApp: u32,
        CP_StartMsgIndEnable: u32,
        CP_SuspendQueueOnError: u32,
        CP_SwCan_HighVoltage: u32,
        CP_TesterPresentAddrMode: u32,
        CP_TesterPresentExpNegResp: Vec<u8>,
        CP_TesterPresentExpPosResp: Vec<u8>,
        CP_TesterPresentHandling: u32,
        CP_TesterPresentMessage: Vec<u8>,
        CP_TesterPresentReqRsp: u32,
        CP_TesterPresentSendType: u32,
        CP_TesterPresentTime: u32,
        CP_TransmitIndEnable: u32,
    }

    fn build_set(&self) -> ComParamDefinitionSet<ComParamDefinition> {
        use ComParamDefinition as Def;
        use dpdu_api_types::PduPc::{Timing as Time, ErrHdl, Com, TesterPresent as Tp};

        ComParamDefinitionSet(hash_set! {
            // Timings.
            Def::new(Time, "CP_CyclicRespTimeout", self.cyclic_resp_timeout),
            Def::new(Time, "CP_ModifyTiming", self.modify_timing),
            Def::new(Time, "CP_P2Max", self.p2_max),
            Def::new(Time, "CP_P2Min", self.p2_min),
            Def::new(Time, "CP_P2Star", self.p2_star),
            Def::new(Time, "CP_P3Min", self.p3_min),

            // Error handling.
            Def::new(ErrHdl, "CP_RC21CompletionTimeout", self.rc21_completion_timeout),
            Def::new(ErrHdl, "CP_RC21Handling", self.rc21_handling),
            Def::new(ErrHdl, "CP_RC21RequestTime", self.rc21_request_time),
            Def::new(ErrHdl, "CP_RC23CompletionTimeout", self.rc23_completion_timeout),
            Def::new(ErrHdl, "CP_RC23Handling", self.rc23_handling),
            Def::new(ErrHdl, "CP_RC23RequestTime", self.rc23_request_time),
            Def::new(ErrHdl, "CP_RC78CompletionTimeout", self.rc78_completion_timeout),
            Def::new(ErrHdl, "CP_RC78Handling", self.rc78_handling),
            Def::new(ErrHdl, "CP_RCByteOffset", self.rc_byte_offset),
            Def::new(ErrHdl, "CP_RepeatReqCountApp", self.repeat_req_count_app),
            Def::new(ErrHdl, "CP_SuspendQueueOnError", self.suspend_queue_on_error),

            // Com.
            Def::new(Com, "CP_ChangeSpeedCtrl", self.change_speed_ctrl),
            Def::new(Com, "CP_ChangeSpeedMessage", self.change_speed_message.clone()),
            Def::new(Com, "CP_ChangeSpeedRate", self.change_speed_rate),
            Def::new(Com, "CP_ChangeSpeedResCtrl", self.change_speed_res_ctrl),
            Def::new(Com, "CP_ChangeSpeedTxDelay", self.change_speed_tx_delay),
            Def::new(Com, "CP_EnablePerformanceTest", self.enable_performance_test),
            Def::new(Com, "CP_Loopback", self.loopback),
            Def::new(Com, "CP_StartMsgIndEnable", self.start_msg_ind_enable),
            Def::new(Com, "CP_SwCan_HighVoltage", self.sw_can_high_voltage),
            Def::new(Com, "CP_TransmitIndEnable", self.transmit_ind_enable),

            // Tester present.
            Def::new(Tp, "CP_TesterPresentAddrMode", self.tester_present_addr_mode),
            Def::new(Tp, "CP_TesterPresentExpNegResp", self.tester_present_exp_neg_resp.clone()),
            Def::new(Tp, "CP_TesterPresentExpPosResp", self.tester_present_exp_pos_resp.clone()),
            Def::new(Tp, "CP_TesterPresentHandling", self.tester_present_handling),
            Def::new(Tp, "CP_TesterPresentMessage", self.tester_present_message.clone()),
            Def::new(Tp, "CP_TesterPresentReqRsp", self.tester_present_req_rsp),
            Def::new(Tp, "CP_TesterPresentSendType", self.tester_present_send_type),
            Def::new(Tp, "CP_TesterPresentTime", self.tester_present_time),
        })
    }

    fn build_table(&self) -> ComParamDefinitionTable<ComParamDefinition> {
        ComParamDefinitionTable::new()
    }
}