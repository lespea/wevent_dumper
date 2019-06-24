use std::ptr;

use winapi::shared::ntdef::NULL;
use winapi::shared::winerror;
use winapi::um::winevt::{EvtClose, EvtNextChannelPath, EvtOpenChannelEnum, EVT_HANDLE};

use crate::errors::WinEvtError;
use widestring::U16String;
use winapi::um::errhandlingapi::GetLastError;

pub struct ChannelIter {
    handle: EVT_HANDLE,
    buf: Vec<u16>,
}

impl Iterator for ChannelIter {
    type Item = Result<String, WinEvtError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut filled = 0;

        let got_okay = unsafe {
            EvtNextChannelPath(
                self.handle,
                self.buf.capacity() as u32,
                self.buf.as_mut_ptr(),
                &mut filled,
            )
        };

        if got_okay == 0 {
            let err = unsafe { GetLastError() };
            if err == winerror::ERROR_INSUFFICIENT_BUFFER {
                self.buf.clear();
                self.buf.reserve(filled as usize);
                self.next()
            } else if err == winerror::ERROR_NO_MORE_ITEMS {
                None
            } else {
                Some(Err(WinEvtError::from_dword(err)))
            }
        } else {
            if unsafe { self.buf.as_ptr().add(filled as usize - 1).read() } == 0 {
                filled -= 1;
            }
            let s = unsafe { U16String::from_ptr(self.buf.as_ptr(), filled as usize) }
                .to_string_lossy();
            self.buf.clear();
            Some(Ok(s))
        }
    }
}

impl ChannelIter {
    pub fn new() -> Result<ChannelIter, WinEvtError> {
        let handle = unsafe { EvtOpenChannelEnum(ptr::null_mut(), 0) };
        if handle == NULL {
            Err(WinEvtError::from_last_error())
        } else {
            Ok(ChannelIter {
                handle,
                buf: Vec::with_capacity(1024 * 2),
            })
        }
    }
}

impl Drop for ChannelIter {
    fn drop(&mut self) {
        if unsafe { EvtClose(self.handle) } == 0 {
            panic!(format!(
                "Couldn't close the channel enum handle: {}",
                WinEvtError::from_last_error()
            ))
        }
    }
}
