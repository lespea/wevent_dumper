use std::ptr;

use widestring::U16CString;
use winapi::um::winevt::{
    EvtClose, EvtGetPublisherMetadataProperty, EvtOpenPublisherMetadata, EVT_HANDLE,
};
use winapi::um::winnt::{
    LANG_ENGLISH, LCID, MAKELANGID, MAKELCID, SORT_DEFAULT, SUBLANG_ENGLISH_US,
};

use crate::errors::WinError;
use crate::errors::WinEvtError;
use crate::pub_metadata_fields::PubMetaField;
use crate::utils;
use crate::vwrapper::WevWrapper;

pub struct PubMetadataFetcher {
    pub name: String,
    handle: EVT_HANDLE,
}

impl PubMetadataFetcher {
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

        Ok(PubMetadataFetcher { name, handle })
    }

    pub fn for_publisher(name: String) -> Result<Self, WinEvtError> {
        PubMetadataFetcher::for_publisher_and_locale(
            name,
            MAKELCID(MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US), SORT_DEFAULT),
        )
    }

    pub fn get_prop(
        &mut self,
        field: &PubMetaField,
        varw: &mut WevWrapper,
    ) -> Result<(), WinEvtError> {
        let mut buf_used = 0;

        let (var, vsize) = varw.get_pointer();

        if let Err(e) = utils::check_okay_check(unsafe {
            EvtGetPublisherMetadataProperty(
                self.handle,
                field.id,
                0,
                vsize as u32,
                var,
                &mut buf_used,
            )
        }) {
            return match e {
                WinError::InsufficientBuffer => {
                    varw.resize(buf_used as usize).unwrap();
                    return self.get_prop(field, varw);
                }
                err => Err(err.into_err()),
            };
        }

        Ok(())
    }
}

impl Drop for PubMetadataFetcher {
    fn drop(&mut self) {
        crate::utils::check_okay(unsafe { EvtClose(self.handle) })
            .expect("Couldn't close the pub metadata handle")
    }
}
