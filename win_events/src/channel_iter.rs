use std::ptr;

use widestring::U16CString;
use winapi::um::winevt::{EvtClose, EvtNextChannelPath, EvtOpenChannelEnum, EVT_HANDLE};

use crate::errors::WinEvtError;
use crate::utils;

/// Iterator over the list of channels registered on this machine
pub struct ChannelIter {
    handle: EVT_HANDLE,
    buf: Vec<u16>,
}

impl Iterator for ChannelIter {
    /// Iterate over the channel names (but each part might have an error)
    type Item = Result<String, WinEvtError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut used = 0;

        // Loop until we get an error we can't deal with or get a valid response
        while let Err(e) = utils::check_okay(unsafe {
            EvtNextChannelPath(
                self.handle,
                self.buf.capacity() as u32, // size in chars
                self.buf.as_mut_ptr(),
                &mut used,
            )
        }) {
            match e {
                WinEvtError::InsufficientBuffer => {
                    if (used as usize) < self.buf.capacity() {
                        self.buf.reserve(used as usize);
                    } else {
                        self.buf.reserve(used as usize - self.buf.capacity());
                    }
                }

                WinEvtError::NoMoreItems => return None,
                e => return Some(Err(e)),
            };
        }

        unsafe {
            match U16CString::from_ptr_with_nul(self.buf.as_ptr(), used as usize) {
                Ok(sp) => Some(Ok(sp.to_string_lossy())),
                Err(_) => Some(Err(WinEvtError::InvalidStrPtr)),
            }
        }
    }
}

impl ChannelIter {
    pub fn new() -> Result<ChannelIter, WinEvtError> {
        Ok(ChannelIter {
            handle: utils::not_null(unsafe { EvtOpenChannelEnum(ptr::null_mut(), 0) })?,
            buf: Vec::with_capacity(1 << 9),
        })
    }
}

impl Drop for ChannelIter {
    fn drop(&mut self) {
        crate::utils::check_okay(unsafe { EvtClose(self.handle) })
            .expect("Couldn't close the channel enum handle")
    }
}
