use std::cell::Cell;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use cfg_if::cfg_if;
use dpdu_api_types::PduStatus;
use j2534_mrw::{Device, Interface};
use parking_lot::RwLock;
use tracing::{error, warn};
use crate::SendSync;

static MODULES: OnceLock<Vec<PassthruModule>> = OnceLock::new();

pub struct PassthruModule {
    pub name: String,

    pub vendor: Option<String>,

    pub library_path: PathBuf,

    pub product_version: Option<String>,

    pub winreg_folder: String,

    pub status: RwLock<PduStatus>,

    pub interface: RwLock<Option<Arc<SendSync<&'static Interface>>>>,

    pub device: RwLock<Option<Arc<SendSync<Device<'static>>>>>,
}

impl PassthruModule {
    pub fn set_interface(&self, interface: Option<&'static Interface>) {
        *self.interface.write() = interface.map(|v| Arc::new(SendSync(v)));
    }

    pub fn get_interface(&self) -> Option<Arc<SendSync<&'static Interface>>> {
        self.interface.read().clone()
    }

    pub fn set_device(&self, device: Option<Device<'static>>) {
        *self.device.write() = device.map(|v| Arc::new(SendSync(v)));
    }

    pub fn get_device(&self) -> Option<Arc<SendSync<Device<'static>>>> {
        self.device.read().clone()
    }

    pub fn set_status(&self, status: PduStatus) {
        *self.status.write() = status;
    }

    pub fn get_status(&self) -> PduStatus {
        self.status.read().clone()
    }

    pub fn get(id: usize) -> Option<&'static PassthruModule> {
        PassthruModule::load().get(id)
    }

    pub fn load() -> &'static [PassthruModule] {
        use winreg::enums::HKEY_LOCAL_MACHINE;
        use winreg::{enums, RegKey};

        MODULES.get_or_init(|| {
            #[allow(non_snake_case)]
            let HKEY_LM_REG_KEY = RegKey::predef(HKEY_LOCAL_MACHINE);
            let mut modules = vec![];

            cfg_if! {
                if #[cfg(target_arch = "x86_64")] {
                    let flags = enums::KEY_READ;
                } else if #[cfg(target_arch = "x86")] {
                    let flags = enums::KEY_READ | enums::KEY_WOW64_32KEY;
                } else {
                    compile_error!("Unsupported target architecture");
                }
            }

            const WINREG_PATH: &'static str = "SOFTWARE\\PassThruSupport.04.04";

            let key = match HKEY_LM_REG_KEY.open_subkey_with_flags(WINREG_PATH, flags ) {
                Ok(v) => v,
                Err(err) => {
                    error!("Failed to open registry key {WINREG_PATH:?}: {err}");
                    return modules;
                }
            };

            for folder in key.enum_keys() {
                let Ok(folder) = folder else {
                    continue;
                };

                let key = match key.open_subkey_with_flags(&folder, enums::KEY_READ) {
                    Ok(v) => v,
                    Err(err) => {
                        let path = format!("{WINREG_PATH}\\{folder}");
                        error!("Failed to open registry key {path:?}: {err}");
                        continue;
                    }
                };

                let Ok(name) = key.get_value::<String, _>("Name") else {
                    continue;
                };

                let Some(library_path) = key
                    .get_value::<String, _>("FunctionLibrary")
                    .map(|v| PathBuf::from(v))
                    .ok()
                    .and_then(|v| (v.is_file() && v.exists()).then(|| v))
                else {
                    continue;
                };

                let version= key
                    .get_value::<String, _>("ProductVersion")
                    .ok();

                modules.push(PassthruModule {
                    name,
                    vendor: key.get_value::<String, _>("Vendor").ok(),
                    library_path,
                    product_version: version,
                    winreg_folder: folder,
                    status: RwLock::new(PduStatus::ModstAvail),
                    interface: Default::default(),
                    device: Default::default(),
                });
            }

            modules
        })
    }
}