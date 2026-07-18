use std::cell::Cell;
use std::sync::Once;
use crate::vendor_specific::detours;

thread_local! {
    static DEVICE_NOT_CONNECTED: Cell<bool> = Cell::new(false);
}

pub(crate) fn has_device_not_connected() -> bool {
    DEVICE_NOT_CONNECTED.with(|cell| {
        let result = cell.get();
        cell.set(false);
        result
    })
}

pub(crate) fn message_box_a_callback(text: &str) -> bool {
    match text {
        "Device Not Connected!" => {
            DEVICE_NOT_CONNECTED.with(|cell| cell.set(true));
            true
        },
        _ => false
    }
}

pub(crate) fn register_callback() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        detours::msg_box_a::register_callback(message_box_a_callback);
    });
}