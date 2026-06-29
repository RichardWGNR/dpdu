mod module_description;
pub mod root_file;

use std::ffi::{CStr, c_char};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek};
use std::path::Path;
use std::ptr::NonNull;

/// Converts a nullable C string to `Option<String>`.
///
/// Returns `None` for null pointers and empty strings.
pub(crate) fn c_str(ptr: *const c_char) -> Option<String> {
    NonNull::new(ptr as _)
        .map(|wrapped_ptr| unsafe {
            CStr::from_ptr(wrapped_ptr.as_ptr())
                .to_string_lossy()
                .into_owned()
        })
        .filter(|s| !s.is_empty())
}

/// FileReader that skips the BOM header.
pub(crate) fn get_bomless_file_reader(path: &Path) -> Result<BufReader<File>, std::io::Error> {
    let file = OpenOptions::new().read(true).open(path)?;
    let mut reader = BufReader::new(file);

    let mut bom_header = [0u8; 3];
    reader.read_exact(&mut bom_header)?;

    if !bom_header.starts_with(&[239, 187, 191]) {
        reader.rewind()?;
    }

    Ok(reader)
}
