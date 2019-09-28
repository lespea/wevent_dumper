use snafu::Snafu;
use widestring::U16String;
use winapi::shared::winerror;
use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winevt::EvtGetExtendedStatus;

pub type Result<T> = std::result::Result<T, WinEvtError>;

#[derive(Debug, Snafu)]
pub enum WinEvtError {
    #[snafu(display("No more items"))]
    NoMoreItems,

    #[snafu(display("Insufficient buffer passed"))]
    InsufficientBuffer,

    #[snafu(display("Found an invalid string pointer"))]
    InvalidStrPtr,

    #[snafu(display("Evt error {}: {}", errno, msg))]
    StdEvtError { errno: u32, msg: &'static str },

    #[snafu(display("Evt error {}: {}", errno, msg))]
    ExtendedEvtError { errno: u32, msg: String },

    #[snafu(display("Other os error"))]
    OsError { source: std::io::Error },
}

impl WinEvtError {
    pub fn from_last_error() -> Self {
        Self::from_dword(unsafe { GetLastError() })
    }

    pub fn from_dword(errno: u32) -> Self {
        match errno {
            ERROR_NO_MORE_ITEMS => WinEvtError::NoMoreItems,
            ERROR_INSUFFICIENT_BUFFER => WinEvtError::InsufficientBuffer,
            _ => {
                let msg: Option<&'static str> = match errno {
                    ERROR_EVT_CANNOT_OPEN_CHANNEL_OF_QUERY => Some("cannot open channel of query"),
                    ERROR_EVT_CHANNEL_CANNOT_ACTIVATE => Some("channel cannot activate"),
                    ERROR_EVT_CHANNEL_NOT_FOUND => Some("channel not found"),
                    ERROR_EVT_CONFIGURATION_ERROR => Some("configuration error"),
                    ERROR_EVT_EVENT_DEFINITION_NOT_FOUND => Some("event definition not found"),
                    ERROR_EVT_EVENT_TEMPLATE_NOT_FOUND => Some("event template not found"),
                    ERROR_EVT_FILTER_ALREADYSCOPED => Some("filter alreadyscoped"),
                    ERROR_EVT_FILTER_INVARG => Some("filter invarg"),
                    ERROR_EVT_FILTER_INVTEST => Some("filter invtest"),
                    ERROR_EVT_FILTER_INVTYPE => Some("filter invtype"),
                    ERROR_EVT_FILTER_NOTELTSET => Some("filter noteltset"),
                    ERROR_EVT_FILTER_OUT_OF_RANGE => Some("filter out of range"),
                    ERROR_EVT_FILTER_PARSEERR => Some("filter parseerr"),
                    ERROR_EVT_FILTER_TOO_COMPLEX => Some("filter too complex"),
                    ERROR_EVT_FILTER_UNEXPECTEDTOKEN => Some("filter unexpectedtoken"),
                    ERROR_EVT_FILTER_UNSUPPORTEDOP => Some("filter unsupportedop"),
                    ERROR_EVT_INVALID_CHANNEL_PATH => Some("invalid channel path"),
                    ERROR_EVT_INVALID_CHANNEL_PROPERTY_VALUE => {
                        Some("invalid channel property value")
                    }
                    ERROR_EVT_INVALID_EVENT_DATA => Some("invalid event data"),
                    ERROR_EVT_INVALID_OPERATION_OVER_ENABLED_DIRECT_CHANNEL => {
                        Some("invalid operation over enabled direct channel")
                    }
                    ERROR_EVT_INVALID_PUBLISHER_NAME => Some("invalid publisher name"),
                    ERROR_EVT_INVALID_PUBLISHER_PROPERTY_VALUE => {
                        Some("invalid publisher property value")
                    }
                    ERROR_EVT_INVALID_QUERY => Some("invalid query"),
                    ERROR_EVT_MALFORMED_XML_TEXT => Some("malformed xml text"),
                    ERROR_EVT_MAX_INSERTS_REACHED => Some("max inserts reached"),
                    ERROR_EVT_MESSAGE_ID_NOT_FOUND => Some("message id not found"),
                    ERROR_EVT_MESSAGE_LOCALE_NOT_FOUND => Some("message locale not found"),
                    ERROR_EVT_MESSAGE_NOT_FOUND => Some("message not found"),
                    ERROR_EVT_NON_VALIDATING_MSXML => Some("non validating msxml"),
                    ERROR_EVT_PUBLISHER_DISABLED => Some("publisher disabled"),
                    ERROR_EVT_PUBLISHER_METADATA_NOT_FOUND => Some("publisher metadata not found"),
                    ERROR_EVT_QUERY_RESULT_INVALID_POSITION => {
                        Some("query result invalid position")
                    }
                    ERROR_EVT_QUERY_RESULT_STALE => Some("query result stale"),
                    ERROR_EVT_SUBSCRIPTION_TO_DIRECT_CHANNEL => {
                        Some("subscription to direct channel")
                    }
                    ERROR_EVT_UNRESOLVED_PARAMETER_INSERT => Some("unresolved parameter insert"),
                    ERROR_EVT_UNRESOLVED_VALUE_INSERT => Some("unresolved value insert"),
                    ERROR_EVT_VERSION_TOO_NEW => Some("version too new"),
                    ERROR_EVT_VERSION_TOO_OLD => Some("version too old"),

                    _ => None,
                };

                msg.map(|m| WinEvtError::StdEvtError { errno, msg: m })
                    .or_else(|| {
                        try_detailed_error()
                            .map(|m| WinEvtError::ExtendedEvtError { errno, msg: m })
                    })
                    .unwrap_or_else(|| WinEvtError::OsError {
                        source: std::io::Error::from_raw_os_error(errno as i32),
                    })
            }
        }
    }
}

fn try_detailed_error() -> Option<String> {
    let mut buf = Vec::with_capacity(1024);
    let mut used = 0;

    loop {
        let ret =
            unsafe { EvtGetExtendedStatus(buf.capacity() as u32, buf.as_mut_ptr(), &mut used) };

        if ret != 0 {
            if ret != winerror::ERROR_INSUFFICIENT_BUFFER {
                return None;
            } else if buf.capacity() > used as usize {
                buf.reserve(used as usize)
            } else {
                buf.reserve((used as usize - buf.capacity()) + 10)
            }
        } else {
            break;
        }
    }

    if used == 0 {
        None
    } else {
        Some(unsafe { U16String::from_ptr(buf.as_ptr(), used as usize) }.to_string_lossy())
    }
}
