use crate::errors::WinEvtError;
use crate::win_event::WinEvent;

use std::ptr;
use winapi::shared::winerror;
use winapi::um::winevt::{self, EvtRender};

pub struct Renderer {
    buf: Vec<u16>,
}

impl Renderer {
    pub fn new() -> Self {
        Self::with_capacity(1024 * 1024)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Renderer {
            buf: Vec::with_capacity(cap),
        }
    }

    pub fn render(&mut self, we: WinEvent) -> Result<String, WinEvtError> {
        let mut buf_used = 0;

        let ret = unsafe {
            EvtRender(
                ptr::null_mut(),
                we.handle,
                winevt::EvtRenderEventXml,
                self.buf.len() as u32,
                self.buf.as_mut_ptr() as *mut _,
                &mut buf_used,
                ptr::null_mut(),
            )
        };

        if ret != 0 {
            if ret == winerror::ERROR_INSUFFICIENT_BUFFER as i32 {
                self.buf.clear();
                self.buf.reserve_exact(buf_used as usize);
                self.render(we)
            } else {
                Err(WinEvtError::from_dword(ret))
            }
        } else {
            let xml = unsafe {
                widestring::U16String::from_ptr(self.buf.as_ptr(), buf_used as usize)
                    .to_string_lossy()
            };
            self.buf.clear();
            Ok(xml)
        }
    }
}
