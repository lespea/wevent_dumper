use std::ptr;

use widestring::U16CString;
use winapi::um::winevt::{
    EvtClose, EvtGetPublisherMetadataProperty, EvtOpenPublisherMetadata, EVT_HANDLE,
};
use winapi::um::winnt::{
    LANG_ENGLISH, LCID, MAKELANGID, MAKELCID, SORT_DEFAULT, SUBLANG_ENGLISH_US,
};

use crate::errors::WinEvtError;
use crate::utils;

pub struct PubMetadata {
    name: String,
    handle: EVT_HANDLE,
}

pub struct PubMetaProb {
    id: u32,
    name: &'static str,
}

impl PubMetadata {
    pub fn for_publisher_and_locale(name: String, lang: LCID) -> Result<Self, WinEvtError> {
        let handle = utils::not_null(unsafe {
            EvtOpenPublisherMetadata(
                ptr::null_mut(),
                U16CString::from_str(name.as_str())
                    .expect("Invalid provider")
                    .as_ptr(),
                ptr::null_mut(),
                lang,
                0,
            )
        })?;

        Ok(PubMetadata { name, handle })
    }

    pub fn for_publisher(name: String) -> Result<Self, WinEvtError> {
        PubMetadata::for_publisher_and_locale(
            name,
            MAKELCID(MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US), SORT_DEFAULT),
        )
    }
}

impl Drop for PubMetadata {
    fn drop(&mut self) {
        crate::utils::check_okay(unsafe { EvtClose(self.handle) })
            .expect("Couldn't close the pub metadata handle")
    }
}
