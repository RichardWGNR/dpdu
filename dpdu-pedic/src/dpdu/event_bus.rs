use std::collections::HashMap;
use std::ptr;
use std::sync::LazyLock;
use dpdu_api_types::{EventCallbackFn, PduEvtData, PDU_ID_UNDEF};
use parking_lot::RwLock;
use crate::dpdu::logical_links::LogicalLink;
use crate::dpdu::state::PDU_STATE;
use crate::dpdu::types::{PduCllHandle, PduModuleHandle};
use crate::dpdu::types::pdu_event::PduEventTarget;

static EVENT_BUS: LazyLock<EventBus> = LazyLock::new(|| EventBus::default());

#[derive(Debug, Default)]
pub struct EventBus {
    system_handler: RwLock<Option<EventCallbackFn>>,
    module_handlers: RwLock<HashMap<PduModuleHandle, EventCallbackFn>>,
    cll_handlers: RwLock<HashMap<PduCllHandle, EventCallbackFn>>
}

impl EventBus {
    pub fn register(target: &PduEventTarget, callback: Option<EventCallbackFn>) {
        match target {
            PduEventTarget::System => {
                let mut system_handler = EVENT_BUS.system_handler.write();

                match callback {
                    Some(v) => {
                        system_handler.replace(v);
                    },
                    None => {
                        system_handler.take();
                    }
                }
            },
            PduEventTarget::Module(h_mod) => {
                let mut module_handlers = EVENT_BUS.module_handlers.write();
                match callback {
                    Some(v) => {
                        module_handlers.insert(h_mod.to_owned(), v);
                    },
                    None => {
                        module_handlers.remove(h_mod);
                    }
                }
            },
            PduEventTarget::LogicalLink(_, h_cll) => {
                let mut cll_handlers = EVENT_BUS.cll_handlers.write();
                match callback {
                    Some(v) => {
                        cll_handlers.insert(h_cll.to_owned(), v);
                    },
                    None => {
                        cll_handlers.remove(h_cll);
                    }
                }
            }
        }
    }

    pub fn fire(
        target: &PduEventTarget,
        event_type: Option<PduEvtData>,
    ) {
        let event_type = event_type.unwrap_or(PduEvtData::Available);
        let api_tag = PDU_STATE.get_api_tag();

        match target {
            PduEventTarget::System => {
                let system_handler = EVENT_BUS.system_handler.read();
                if let Some(handler) = *system_handler {
                    unsafe {
                        handler(
                            event_type,
                            PDU_ID_UNDEF,
                            PDU_ID_UNDEF,
                            ptr::null_mut(),
                            api_tag as *mut usize as _
                        );
                    }
                }
            },
            PduEventTarget::Module(h_mod) => {
                let module_handlers = EVENT_BUS.module_handlers.read();
                if let Some(handler) = module_handlers.get(h_mod) {
                    unsafe {
                        handler(
                            event_type,
                            h_mod.to_owned(),
                            PDU_ID_UNDEF,
                            ptr::null_mut(),
                            api_tag as *mut usize as _
                        );
                    }
                }
            },
            PduEventTarget::LogicalLink(h_mod, h_cll) => {
                let cll_handlers = EVENT_BUS.cll_handlers.read();
                let link = LogicalLink::get(h_cll.to_owned());
                let cll_tag = link
                    .map(|v| v.get_tag())
                    .unwrap_or(PDU_ID_UNDEF as usize);

                if let Some(handler) = cll_handlers.get(h_cll) {
                    unsafe {
                        handler(
                            event_type,
                            h_mod.to_owned(),
                            h_cll.to_owned(),
                            cll_tag as *mut usize as _,
                            api_tag as *mut usize as _
                        );
                    }
                }
            }
        }
    }
}