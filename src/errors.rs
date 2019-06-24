use std::fmt::{Display, Error, Formatter};

use widestring::U16String;
use winapi::shared::winerror;
use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winevt::EvtGetExtendedStatus;
use windows_error::WindowsError;

pub enum WinError {
    NoMoreItems,
    InsufficientBuffer,
    Err(WinEvtError),
}

impl WinError {
    #[inline]
    pub fn into_err(self) -> WinEvtError {
        match self {
            WinError::NoMoreItems => WinEvtError::from_dword(ERROR_NO_MORE_ITEMS),
            WinError::InsufficientBuffer => WinEvtError::from_dword(ERROR_INSUFFICIENT_BUFFER),
            WinError::Err(e) => e,
        }
    }
}

#[derive(Debug)]
pub struct WinEvtError {
    pub errno: u32,
    pub msg: String,
}

impl Display for WinEvtError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(self.msg.as_str())
    }
}

fn try_detailed_error() -> Option<String> {
    let mut buf = Vec::with_capacity(1024 * 32);
    let mut used = 0;

    let ret = unsafe { EvtGetExtendedStatus(buf.capacity() as u32, buf.as_mut_ptr(), &mut used) };

    if ret != 0 {
        if ret != winerror::ERROR_INSUFFICIENT_BUFFER {
            return None;
        } else {
            buf.clear();
            buf.reserve(used as usize);

            let ret =
                unsafe { EvtGetExtendedStatus(buf.capacity() as u32, buf.as_mut_ptr(), &mut used) };

            if ret != 0 {
                return None;
            }
        }
    }

    if used == 0 {
        None
    } else {
        Some(unsafe { U16String::from_ptr(buf.as_ptr(), used as usize) }.to_string_lossy())
    }
}

impl WinEvtError {
    pub fn from_last_error() -> Self {
        Self::from_dword(unsafe { GetLastError() })
    }

    pub fn from_dword(errno: u32) -> Self {
        let msg: String = match errno {
            ERROR_EVT_CANNOT_OPEN_CHANNEL_OF_QUERY => "cannot open channel of query".to_string(),
            ERROR_EVT_CHANNEL_CANNOT_ACTIVATE => "channel cannot activate".to_string(),
            ERROR_EVT_CHANNEL_NOT_FOUND => "channel not found".to_string(),
            ERROR_EVT_CONFIGURATION_ERROR => "configuration error".to_string(),
            ERROR_EVT_EVENT_DEFINITION_NOT_FOUND => "event definition not found".to_string(),
            ERROR_EVT_EVENT_TEMPLATE_NOT_FOUND => "event template not found".to_string(),
            ERROR_EVT_FILTER_ALREADYSCOPED => "filter alreadyscoped".to_string(),
            ERROR_EVT_FILTER_INVARG => "filter invarg".to_string(),
            ERROR_EVT_FILTER_INVTEST => "filter invtest".to_string(),
            ERROR_EVT_FILTER_INVTYPE => "filter invtype".to_string(),
            ERROR_EVT_FILTER_NOTELTSET => "filter noteltset".to_string(),
            ERROR_EVT_FILTER_OUT_OF_RANGE => "filter out of range".to_string(),
            ERROR_EVT_FILTER_PARSEERR => "filter parseerr".to_string(),
            ERROR_EVT_FILTER_TOO_COMPLEX => "filter too complex".to_string(),
            ERROR_EVT_FILTER_UNEXPECTEDTOKEN => "filter unexpectedtoken".to_string(),
            ERROR_EVT_FILTER_UNSUPPORTEDOP => "filter unsupportedop".to_string(),
            ERROR_EVT_INVALID_CHANNEL_PATH => "invalid channel path".to_string(),
            ERROR_EVT_INVALID_CHANNEL_PROPERTY_VALUE => {
                "invalid channel property value".to_string()
            }
            ERROR_EVT_INVALID_EVENT_DATA => "invalid event data".to_string(),
            ERROR_EVT_INVALID_OPERATION_OVER_ENABLED_DIRECT_CHANNEL => {
                "invalid operation over enabled direct channel".to_string()
            }
            ERROR_EVT_INVALID_PUBLISHER_NAME => "invalid publisher name".to_string(),
            ERROR_EVT_INVALID_PUBLISHER_PROPERTY_VALUE => {
                "invalid publisher property value".to_string()
            }
            ERROR_EVT_INVALID_QUERY => "invalid query".to_string(),
            ERROR_EVT_MALFORMED_XML_TEXT => "malformed xml text".to_string(),
            ERROR_EVT_MAX_INSERTS_REACHED => "max inserts reached".to_string(),
            ERROR_EVT_MESSAGE_ID_NOT_FOUND => "message id not found".to_string(),
            ERROR_EVT_MESSAGE_LOCALE_NOT_FOUND => "message locale not found".to_string(),
            ERROR_EVT_MESSAGE_NOT_FOUND => "message not found".to_string(),
            ERROR_EVT_NON_VALIDATING_MSXML => "non validating msxml".to_string(),
            ERROR_EVT_PUBLISHER_DISABLED => "publisher disabled".to_string(),
            ERROR_EVT_PUBLISHER_METADATA_NOT_FOUND => "publisher metadata not found".to_string(),
            ERROR_EVT_QUERY_RESULT_INVALID_POSITION => "query result invalid position".to_string(),
            ERROR_EVT_QUERY_RESULT_STALE => "query result stale".to_string(),
            ERROR_EVT_SUBSCRIPTION_TO_DIRECT_CHANNEL => {
                "subscription to direct channel".to_string()
            }
            ERROR_EVT_UNRESOLVED_PARAMETER_INSERT => "unresolved parameter insert".to_string(),
            ERROR_EVT_UNRESOLVED_VALUE_INSERT => "unresolved value insert".to_string(),
            ERROR_EVT_VERSION_TOO_NEW => "version too new".to_string(),
            ERROR_EVT_VERSION_TOO_OLD => "version too old".to_string(),

            other => try_detailed_error().unwrap_or_else(|| WindowsError::new(other).to_string()),
        };

        WinEvtError { errno, msg }
    }
}
