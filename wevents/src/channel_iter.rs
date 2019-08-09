use std::ptr;

use widestring::U16String;
use winapi::um::winevt::{EvtClose, EvtNextChannelPath, EvtOpenChannelEnum, EVT_HANDLE};

use crate::errors::WinError;
use crate::errors::WinEvtError;
use crate::utils;

pub struct ChannelIter {
    handle: EVT_HANDLE,
    buf: Vec<u16>,
}

impl Iterator for ChannelIter {
    type Item = Result<String, WinEvtError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut filled = 0;

        if let Err(e) = utils::check_okay_check(unsafe {
            EvtNextChannelPath(
                self.handle,
                self.buf.capacity() as u32,
                self.buf.as_mut_ptr(),
                &mut filled,
            )
        }) {
            return match e {
                WinError::InsufficientBuffer => {
                    self.buf.clear();
                    self.buf.reserve(filled as usize);
                    self.next()
                }

                WinError::NoMoreItems => None,
                WinError::Err(err) => Some(Err(err)),
            };
        }

        if unsafe { self.buf.as_ptr().add(filled as usize - 1).read() } == 0 {
            filled -= 1;
        }
        let s =
            unsafe { U16String::from_ptr(self.buf.as_ptr(), filled as usize) }.to_string_lossy();
        self.buf.clear();
        Some(Ok(s))
    }
}

impl ChannelIter {
    pub fn new() -> Result<ChannelIter, WinEvtError> {
        Ok(ChannelIter {
            handle: utils::not_null(unsafe { EvtOpenChannelEnum(ptr::null_mut(), 0) })?,
            buf: Vec::with_capacity(1024 * 2),
        })
    }
}

impl Drop for ChannelIter {
    fn drop(&mut self) {
        crate::utils::check_okay(unsafe { EvtClose(self.handle) })
            .expect("Couldn't close the channel enum handle")
    }
}
