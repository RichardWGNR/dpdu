use std::collections::HashSet;
use std::ffi::{c_void, CStr};
use std::sync::atomic::{Ordering};
use std::sync::LazyLock;
use parking_lot::RwLock;
use retour::GenericDetour;
use tracing::error;
use windows::core::{s, PCSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};

static CALLBACKS: LazyLock<RwLock<HashSet<MessageBoxACallback>>> = LazyLock::new(|| RwLock::default());

type MessageBoxAFn = unsafe extern "system" fn(hwnd: HWND, text: PCSTR, caption: PCSTR, style: u32) -> i32;

type MessageBoxACallback = fn(text: &str) -> bool;

#[allow(non_snake_case)]
unsafe extern "system"
fn hookedMessageBoxA(hwnd: HWND, text: PCSTR, caption: PCSTR, style: u32) -> i32 {
    let detour = MESSAGE_BOX_A_DETOUR
        .as_ref()
        .expect("internal error: MessageBoxA detour is not initialized");

    let call_trampoline = || -> i32 {
        unsafe { detour.call(hwnd, text, caption, style) }
    };
    
    if !text.is_null() {
        let Ok(text) = unsafe { CStr::from_ptr(text.as_ptr() as _) }.to_str() else {
            return call_trampoline();
        };
        
        let callbacks = CALLBACKS.read();
        for callback in callbacks.iter() {
            if callback(text) {
                return 1;
            }
        }
    }

    call_trampoline()
}

static MESSAGE_BOX_A_DETOUR: LazyLock<Result<GenericDetour<MessageBoxAFn>, String>> = LazyLock::new(|| {
    unsafe {
        let user32dll = LoadLibraryA(s!("user32.dll"))
            .map_err(|e| format!("LoadLibraryA(user32.dll) error: {}", e.message()))?;

        let fn_ptr = GetProcAddress(user32dll, s!("MessageBoxA"))
            .map(|v| v as *const c_void)
            .ok_or_else(|| "unable to take a pointer to the MessageBoxA() function".to_string())?;

        let fn_rust_ptr: MessageBoxAFn = std::mem::transmute(fn_ptr);

        GenericDetour::new(fn_rust_ptr, hookedMessageBoxA)
            .map_err(|e| format!("GenericDetour::new(): {}", e.to_string()))
    }
});

pub(crate) fn hook_message_box_a() -> bool {
    match MESSAGE_BOX_A_DETOUR.as_ref() {
        Ok(detour) => {
            if !detour.is_enabled() {
                if let Err(err) = unsafe { detour.enable() } {
                    error!("MessageBoxA hook error: {err}");
                } else {
                    return true;
                }
            }
        },
        Err(err) => {
            error!("MessageBoxA hook error: {err}");
        }
    }
    false
}

pub(crate) fn register_callback(callback: MessageBoxACallback) {
    let mut callbacks = CALLBACKS.write();
    callbacks.insert(callback);
}