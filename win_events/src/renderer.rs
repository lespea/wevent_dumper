use std::ptr;

use winapi::um::winevt::{self, EvtRender};

use crate::errors::WinEvtError;
use crate::utils;
use crate::win_event::WinEvent;

/// Turns a windows event into an xml string, reusing the buffer for each render
pub struct Renderer {
    buf: Vec<u16>,
}

impl Default for Renderer {
    fn default() -> Self {
        Renderer::new()
    }
}

impl Renderer {
    /// New renderer with 32k of buffer
    pub fn new() -> Self {
        Self::with_capacity(1024 * 32)
    }

    /// New renderer with the specified buffer
    pub fn with_capacity(cap: usize) -> Self {
        Renderer {
            buf: Vec::with_capacity(cap),
        }
    }

    /// Render the windows event as xml
    pub fn render(&mut self, we: WinEvent) -> Result<String, WinEvtError> {
        let mut buf_used = 0;

        match utils::check_bool(unsafe {
            EvtRender(
                ptr::null_mut(),
                we.handle,
                winevt::EvtRenderEventXml,
                self.buf.capacity() as u32 * 2,
                self.buf.as_mut_ptr() as *mut _,
                &mut buf_used,
                ptr::null_mut(),
            )
        }) {
            Err(WinEvtError::InsufficientBuffer) => {
                self.buf.clear();
                self.buf.reserve((buf_used / 2) as usize + 1);
                return self.render(we);
            }

            Err(e) => return Err(e),

            _ => (),
        };

        // We need # of u16 but it returns "bytes" so u8 which means we need half of this
        let mut buf_used = (buf_used / 2) as usize;

        // See if there is a null byte at end. Should be but double check just in case
        if self.buf[buf_used] == 0 {
            buf_used -= 1;
        }

        let xml = unsafe {
            widestring::U16String::from_ptr(self.buf.as_ptr(), buf_used).to_string_lossy()
        };

        self.buf.clear();
        Ok(xml)
    }
}
