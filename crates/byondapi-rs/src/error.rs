use std::ffi::{CStr, CString};

use crate::static_global::BYOND;

#[derive(Debug)]
pub enum Error {
    /// This error is thrown when you try to convert a ByondValue into a type which it does not represent.
    InvalidConversion,
    /// Thrown when trying to get a String from a ByondValue.
    NonUtf8String,
    /// Internal BYOND API error
    ByondError(ByondError),
    /// When the BYOND API doesn't tell us what the error is
    UnknownByondError,
}

impl Error {
    pub fn get_last_byond_error() -> Self {
        if let Some(err) = ByondError::get_last() {
            Self::ByondError(err)
        } else {
            Self::UnknownByondError
        }
    }
}

#[derive(Debug)]
pub struct ByondError(CString);

impl ByondError {
    pub fn get_last() -> Option<Self> {
        // Safety: It's always safe to call Byond_LastError
        let ptr = unsafe { BYOND.Byond_LastError() };
        if !ptr.is_null() {
            // Safety: We just have to trust that Byond gave us a valid cstring...
            let cstr = unsafe { CStr::from_ptr(ptr) };
            Some(ByondError(cstr.to_owned()))
        } else {
            None
        }
    }
}