use std::ffi::c_void;
use std::slice;
use dpdu_api_types::{FlagData, PduError, PduStatus, RscData, PDU_ID_UNDEF};
use crate::dpdu::logical_links::LogicalLink;
use crate::dpdu::objects::{BUSES, BUS_ISO_11898_2_DWCAN, PIN_HI, PIN_LOW, PROTOCOL_ISO_11898_RAW};
use crate::dpdu::state::PDU_STATE;
use crate::dpdu::types::PduUniqueCllTag;
use crate::passthru::PassthruModule;
use crate::utils::is_valid_ptr;

#[unsafe(no_mangle)]
pub extern "system" fn PDUCreateComLogicalLink(
    h_mod: u32,
    p_rsc_data: *const RscData,
    resource_id: u32,
    p_cll_tag: *const c_void,
    ph_cll: *mut u32,
    p_cll_create_flag: *mut FlagData
) -> PduError {
    if !PDU_STATE.is_constructed() {
        return PduError::PduApiNotConstructed;
    } else if !is_valid_ptr(p_rsc_data)
        || !is_valid_ptr(ph_cll)
        || !is_valid_ptr(p_cll_create_flag) {
        return PduError::InvalidParameters;
    }

    if resource_id != PDU_ID_UNDEF {
        // идентификаторы ресурсов (пока) не поддерживаем.
        return PduError::InvalidParameters;
    }

    let resource_data = unsafe { &*p_rsc_data };
    if let Err(err) = check_resource_data(resource_data) {
        return err;
    }

    let Some(module) = PassthruModule::get(h_mod as _) else {
        return PduError::InvalidHandle;
    };

    if !matches!(module.get_status(), PduStatus::ModstReady) {
        return PduError::ModuleNotConnected;
    }
    
    let h_cll = LogicalLink::register(LogicalLink {
        protocol_id: resource_data.protocol_id,
        bus_id: resource_data.bus_type_id,
        tag: (!p_cll_tag.is_null())
            .then(|| PduUniqueCllTag::new(p_cll_tag as usize))
            .flatten(),
        ..Default::default()
    });

    unsafe {
        *ph_cll = h_cll;
    }

    PduError::StatusNoError
}

fn check_resource_data(resource_data: &RscData) -> Result<(), PduError> {
    let bus_type_id = resource_data.bus_type_id;
    let protocol_id = resource_data.protocol_id;

    if bus_type_id != BUS_ISO_11898_2_DWCAN || protocol_id != PROTOCOL_ISO_11898_RAW {
        // Поддерживаем только ISO_11898_RAW на ISO_11898_2_DWCAN.
        return Err(PduError::InvalidParameters);
    }

    check_pins(resource_data)?;

    Ok(())
}

fn check_pins(resource_data: &RscData) -> Result<(), PduError> {
    if !is_valid_ptr(resource_data.p_dlc_pin_data) {
        return Err(PduError::InvalidParameters);
    }

    let pins = unsafe {
        slice::from_raw_parts(
            resource_data.p_dlc_pin_data,
            resource_data.num_pin_data as _
        )
    };

    let mut hi = false;
    let mut low = false;

    for pin in pins {
        if pin.dlc_pin_type_id == PIN_HI && pin.dlc_pin_number == 6 {
            hi = true;
        }
        if pin.dlc_pin_type_id == PIN_LOW && pin.dlc_pin_number == 14 {
            low = true;
        }
    }

    if !hi || !low {
        return Err(PduError::InvalidParameters);
    }

    Ok(())
}