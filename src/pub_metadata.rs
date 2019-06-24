use std::ptr;

use widestring::U16CString;
use winapi::um::winevt::{
    EvtClose, EvtGetPublisherMetadataProperty, EvtOpenPublisherMetadata, EVT_HANDLE, EVT_VARIANT,
};
use winapi::um::winnt::{
    LANG_ENGLISH, LCID, MAKELANGID, MAKELCID, SORT_DEFAULT, SUBLANG_ENGLISH_US,
};

use crate::errors::WinError;
use crate::errors::WinEvtError;
use crate::pub_metadata_fields::PubMetaField;
use crate::utils;

pub struct PubMetadata {
    name: String,
    handle: EVT_HANDLE,
    variant: EVT_VARIANT,
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

        Ok(PubMetadata {
            name,
            handle,
            variant: unsafe { std::mem::zeroed() },
        })
    }

    pub fn for_publisher(name: String) -> Result<Self, WinEvtError> {
        PubMetadata::for_publisher_and_locale(
            name,
            MAKELCID(MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US), SORT_DEFAULT),
        )
    }

    pub fn get_prop(&mut self, field: PubMetaField) -> Result<EVT_VARIANT, WinEvtError> {
        let mut buf_used = 0;

        if let Err(e) = utils::check_okay_check(unsafe {
            EvtGetPublisherMetadataProperty(
                self.handle,
                field.id,
                0,
                std::u32::MAX,
                &mut self.variant,
                &mut buf_used,
            )
        }) {
            return match e {
                WinError::InsufficientBuffer => panic!(format!(
                    "Insufficient size creating a variant? :: {}/{}/{}",
                    buf_used,
                    std::mem::size_of::<EVT_VARIANT>(),
                    unsafe { std::mem::size_of_val(&self.variant) }
                )),
                err => Err(err.into_err()),
            };
        }

        Ok(self.variant.clone())
    }
}

impl Drop for PubMetadata {
    fn drop(&mut self) {
        crate::utils::check_okay(unsafe { EvtClose(self.handle) })
            .expect("Couldn't close the pub metadata handle")
    }
}
