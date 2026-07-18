use crate::handle_manager::PduHandleManager;
use crate::types::pdu_event::{PduEvent, PduEventTarget};
use crate::types::{PduCllHandle, PduModuleHandle, PduUniqueApiTag, PduUniqueCllTag};
use dpdu_api_types::PduEvtData;
use std::ffi::c_void;
use tracing::{error, trace, warn};

pub(crate) unsafe extern "system-unwind" fn event_callback(
    _event_type: PduEvtData,
    h_mod: PduModuleHandle,
    h_cll: PduCllHandle,
    p_cll_tag: *mut c_void,
    p_api_tag: *mut c_void,
) {
    let api = match PduUniqueApiTag::new(p_api_tag as _) {
        Some(api_tag) => PduHandleManager::lookup_api_reference(api_tag),
        None => PduHandleManager::get_single_api()
    };

    let Some(api) = api else {
        error!(
            api_tag = p_api_tag as usize,
            h_mod,
            h_cll,
            "PDUEventCallback: there were no suitable APIs for the event"
        );
        return;
    };

    let event_target = PduEventTarget::from_callback(h_mod, h_cll);

    let mut events: Vec<PduEvent> = vec![];

    while let Ok(Some(event)) = api.pdu_get_event_item(&event_target).inspect_err(|err| {
        error!(
            api_tag = api.get_unique_tag(),
            h_mod,
            h_cll,
            "PDUEventCallback: error reading an event: {err}"
        );
    }) {
        events.push(event);
    }

    for event in events {
        trace!("PDUEventCallback: {event:?}");

        match event.target {
            PduEventTarget::System => {
                // System event.
                let Some(api_event_tx) = PduHandleManager::lookup_api_event_tx(
                    api.get_unique_tag()
                )
                else {
                    warn!(h_mod, "PDUEventCallback: unable to lookup event_tx for the PduApi");
                    return;
                };
                
                if let Err(err) = api_event_tx.try_send(event) {
                    warn!(h_mod, "PDUEventCallback: unable to deliver event to the PduApi: {err}");
                }
            }
            PduEventTarget::Module(h_mod) => {
                // Module event.
                let Some(module_event_tx) = PduHandleManager::lookup_module_event_tx(
                    api.get_unique_tag(),
                    h_mod
                )
                else {
                    warn!(h_mod, "PDUEventCallback: unable to lookup event_tx for the PduVci");
                    return;
                };

                if let Err(err) = module_event_tx.try_send(event) {
                    warn!(h_mod, "PDUEventCallback: unable to deliver event to the PduVci: {err}");
                }
            }
            PduEventTarget::LogicalLink(_h_mod, h_cll) => {
                let Some(cll_tag) = PduUniqueCllTag::new(p_cll_tag as _) else {
                    warn!(
                        "PDUEventCallback: abnormally CLL creation: cll_tag is required when PduEventTarget = ComLogicalLink"
                    );
                    continue;
                };

                if let Some(h_cop) = event.h_cop {
                    // ComPrimitive event.
                    let Some(cop_tag) = event.cop_tag else {
                        warn!(h_cop, "PDUEventCallback: API does not provide a COP tag");
                        return;
                    };

                    let Some(cop_event_tx) =
                        PduHandleManager::lookup_cop_event_tx(api.get_unique_tag(), cop_tag)
                    else {
                        warn!(
                            h_cll,
                            tag = cll_tag,
                            "PDUEventCallback: unable to lookup event_tx for the PduComPrimitive"
                        );
                        return;
                    };

                    if let Err(err) = cop_event_tx.try_send(event) {
                        warn!(
                            h_cop,
                            "PDUEventCallback: unable to deliver event to the PduComPrimitive: {err}"
                        );
                    }
                } else {
                    // ComLogicalLink event.
                    let Some(cll_event_tx) =
                        PduHandleManager::lookup_cll_event_tx(api.get_unique_tag(), cll_tag)
                    else {
                        warn!(
                            h_cll,
                            tag = cll_tag,
                            "PDUEventCallback: unable to lookup event_tx for the PduComLogicalLink"
                        );
                        return;
                    };

                    if let Err(err) = cll_event_tx.try_send(event) {
                        warn!(
                            h_cll,
                            tag = cll_tag,
                            "PDUEventCallback: unable to deliver event to the PduComLogicalLink: {err}"
                        );
                    }
                }
            }
        }
    }
}
