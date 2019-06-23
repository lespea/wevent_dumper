use crate::errors::WinEvtError;
use crate::win_event::WinEvent;

use std::ptr;
use winapi::shared::winerror;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winevt::{self, EvtRender};

pub struct Renderer {
    buf: Vec<u16>,
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
                self.buf.capacity() as u32,
                self.buf.as_mut_ptr() as *mut _,
                &mut buf_used,
                ptr::null_mut(),
            )
        };

        if render_passed == 0 {
            let err = unsafe { GetLastError() };
            if err == winerror::ERROR_INSUFFICIENT_BUFFER {
                self.buf.clear();
                self.buf.reserve(buf_used as usize);
                self.render(we)
            } else {
                Err(WinEvtError::from_dword(err))
            }
        } else {
            let xml = unsafe {
                widestring::U16CString::from_ptr(self.buf.as_ptr(), buf_used as usize / 2 - 1)
                    .expect("bad unicode")
                    .to_string()
                    .expect("bad unicode")
            };
            self.buf.clear();
            Ok(xml)
        }
    }
}
