pub mod module_description;
pub mod root_file;

use std::ffi::{CStr, c_char, c_void};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Cursor, Read, Seek};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::ptr::NonNull;
use crate::types::PduUniqueRespIdentifier;

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

/// A lifetime-bound opaque pointer (`*const c_void`).
///
/// Does not own the pointed-to value.
#[derive(Debug)]
pub(crate) struct VoidPtr<'a> {
    pub ptr: *const c_void,
    _marker: PhantomData<&'a ()>
}

impl<'a> VoidPtr<'a> {
    pub fn new(ptr: *const c_void) -> Self {
        Self {
            ptr,
            _marker: PhantomData
        }
    }
    
    /// Returns the pointer as `*const c_void`.
    pub fn as_ptr(&self) -> *const c_void {
        self.ptr
    }

    /// Returns the pointer as `*mut c_void`.
    ///
    /// This only casts the pointer type and does not guarantee mutability
    /// of the underlying data.
    pub fn as_mut(&self) -> *mut c_void {
        self.ptr as _
    }
}

/// Marks the wrapped value as `Send` and `Sync`.
///
/// # Safety
///
/// The caller must ensure that sharing or transferring `T` across threads
/// is sound. Wrapping a non-thread-safe type in `SendSync` can cause
/// undefined behavior.
#[derive(Debug, Clone)]
pub(crate) struct SendSync<T>(pub T);

unsafe impl<T> Send for SendSync<T> {}
unsafe impl<T> Sync for SendSync<T> {}

impl<T> Deref for SendSync<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for SendSync<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// When calling [`PDUSetUniqueRespIdTable`], the caller must provide a unique 32-bit identifier
/// that is used to identify subsequent requests to and responses from the ECU.
///
/// This function converts block names (for example, `EZS213`) into a 32-bit identifier.
/// The conversion is performed by hashing the string using the MurmurHash3 algorithm,
/// which provides a low probability of collisions compared to many other hashing algorithms.
pub fn ecu_name_to_unique_resp_id<S>(name: S) -> PduUniqueRespIdentifier
where
    S: AsRef<str>,
{
    murmur3::murmur3_32(&mut Cursor::new(name.as_ref()), 0).expect("murmur failed")
}