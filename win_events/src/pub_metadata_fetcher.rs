use std::ptr;

use widestring::U16CString;
use winapi::shared::guiddef::GUID;
use winapi::um::winevt::{
    self, EvtClose, EvtGetPublisherMetadataProperty, EvtOpenPublisherMetadata, EVT_HANDLE,
};
use winapi::um::winnt::{
    LANG_ENGLISH, LCID, MAKELANGID, MAKELCID, SORT_DEFAULT, SUBLANG_ENGLISH_US,
};

use crate::errors::WinError;
use crate::errors::WinEvtError;
use crate::pub_metadata::PubMetadata;
use crate::pub_metadata_fields as meta_fields;
use crate::pub_metadata_fields::PubMetaField;
use crate::utils;
use crate::vwrapper::WevWrapper;
use std::ffi::CStr;

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

    fn get_str(&mut self, varw: &mut WevWrapper, field: &PubMetaField) -> Option<String> {
        if self.get_prop(field, varw).is_ok() {
            let (e, _) = varw.get_pointer();
            match e.Type {
                winevt::EvtVarTypeString => {
                    Some(unsafe { U16CString::from_ptr_str(*e.u.StringVal()).to_string_lossy() })
                }

                winevt::EvtVarTypeAnsiString => {
                    println!("ASCI STR");
                    Some(
                        unsafe { CStr::from_ptr(*e.u.AnsiStringVal()) }
                            .to_string_lossy()
                            .to_string(),
                    )
                }

                winevt::EvtVarTypeEvtHandle => {
                    let _ = varw.close_evt_unchecked();
                    None
                }

                _ => None,
            }
        } else {
            None
        }
    }

    fn get_u32(&mut self, varw: &mut WevWrapper, field: &PubMetaField) -> Option<u32> {
        if self.get_prop(field, varw).is_ok() {
            let (e, _) = varw.get_pointer();
            match e.Type {
                winevt::EvtVarTypeUInt32 => Some(unsafe { *e.u.UInt32Val() }),

                winevt::EvtVarTypeEvtHandle => {
                    let _ = varw.close_evt_unchecked();
                    None
                }

                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_metadata(&mut self, varw: &mut WevWrapper) -> Option<PubMetadata> {
        let guid = if self.get_prop(&meta_fields::PUBLISHER_GUID, varw).is_ok() {
            let (e, _) = varw.get_pointer();
            if e.Type == winevt::EvtVarTypeGuid {
                Some(format_guid(unsafe { **e.u.GuidVal() }))
            } else if e.Type == winevt::EvtVarTypeEvtHandle {
                let _ = varw.close_evt_unchecked();
                None
            } else {
                None
            }
        } else {
            None
        };

        let parameter_file_path = self.get_str(varw, &meta_fields::PARAMETER_FILE_PATH);
        let message_file_path = self.get_str(varw, &meta_fields::MESSAGE_FILE_PATH);

        let help_link = self.get_str(varw, &meta_fields::HELP_LINK);

        let message_id = self.get_u32(varw, &meta_fields::PUBLISHER_MESSAGE_ID);

        Some(PubMetadata {
            guid,
            parameter_file_path,
            message_file_path,

            help_link,

            message_id,

            channels: vec![],
            levels: vec![],
            tasks: vec![],
            opcodes: vec![],
            keywords: vec![],
        })
    }
}

fn format_guid(g: GUID) -> String {
    format!(
        "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
        g.Data1,
        g.Data2,
        g.Data3,
        g.Data4[0],
        g.Data4[1],
        g.Data4[2],
        g.Data4[3],
        g.Data4[4],
        g.Data4[5],
        g.Data4[6],
        g.Data4[7],
    )
}

impl Drop for PubMetadataFetcher {
    fn drop(&mut self) {
        crate::utils::check_okay(unsafe { EvtClose(self.handle) })
            .expect("Couldn't close the pub metadata handle")
    }
}
