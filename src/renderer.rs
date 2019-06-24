use crate::errors::WinEvtError;
use crate::win_event::WinEvent;

use std::ptr;
use winapi::shared::winerror;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winevt::{self, EvtRender};

pub struct Renderer {
    buf: Vec<u16>,
}

impl Default for Renderer {
    fn default() -> Self {
        Renderer::new()
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self::with_capacity(1024 * 32)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Renderer {
            buf: Vec::with_capacity(cap),
        }
    }

    pub fn render(&mut self, we: WinEvent) -> Result<String, WinEvtError> {
        let mut buf_used = 0;

        let render_passed = unsafe {
            EvtRender(
                ptr::null_mut(),
                we.handle,
                winevt::EvtRenderEventXml,
                self.buf.capacity() as u32 * 2,
                self.buf.as_mut_ptr() as *mut _,
                &mut buf_used,
                ptr::null_mut(),
            )
        };

        // We need # of u16 but it returns "bytes" so u8 which means we need half of this
        let mut buf_used = (buf_used / 2) as usize;

        if render_passed == 0 {
            let err = unsafe { GetLastError() };
            if err == winerror::ERROR_INSUFFICIENT_BUFFER {
                self.buf.clear();
                self.buf.reserve(buf_used + 1);
                self.render(we)
            } else {
                Err(WinEvtError::from_dword(err))
            }
        } else {
            // See if there is a null byte at end. Should be but double check just in case
            if unsafe { self.buf.as_ptr().add(buf_used as usize - 1).read() } == 0 {
                buf_used -= 1;
            }

            let xml = unsafe {
                widestring::U16String::from_ptr(self.buf.as_ptr(), buf_used).to_string_lossy()
            };

            self.buf.clear();
            Ok(xml)
        }
    }
}
