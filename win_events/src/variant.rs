use std::mem::size_of;
use std::ptr::null_mut;

use log::{debug, error, info, trace, warn};
use winapi::shared::minwindef::DWORD;
use winapi::um::winevt::*;

use crate::errors::Result;
use crate::errors::WinEvtError::InsufficientBuffer;
use crate::utils;

#[repr(transparent)]
pub struct VariantWrapper(EVT_VARIANT);

impl Drop for VariantWrapper {
    fn drop(&mut self) {
        if self.0.Type == EvtVarTypeEvtHandle {
            let evt = unsafe { *self.0.u.EvtHandleVal() };
            if !evt.is_null() {
                debug!("Closing underlying evt handle");
                if let Err(err) = crate::utils::check_okay(unsafe { EvtClose(evt) }) {
                    error!("Couldn't close evt handle in a variant: {}", err);
                }
            } else {
                debug!("Underlying evt handle was null");
            }
        }
    }
}

#[derive(Default)]
pub struct VariantBuf(Vec<VariantWrapper>);

const VSIZE: usize = size_of::<VariantWrapper>();

impl VariantBuf {
    pub fn new() -> Self {
        VariantBuf(Vec::new())
    }

    pub fn sized(size: usize) -> Self {
        VariantBuf(Vec::with_capacity(size / VSIZE + VSIZE))
    }

    pub fn reset(&mut self) {
        self.0.clear();
    }

    pub fn resize(&mut self, size: DWORD) {
        warn!("Resizing buf to {}", size);
        self.reset();
        self.0.reserve(size as usize / VSIZE + VSIZE);
    }

    pub fn render(&mut self, evt_handle: EVT_HANDLE, ctx: EVT_HANDLE) -> Result<DWORD> {
        let mut used = 0;
        let mut props = 0;

        match utils::check_okay(unsafe {
            EvtRender(
                ctx,
                evt_handle,
                EvtRenderEventValues,
                (self.0.capacity() * VSIZE) as u32,
                self.0.as_mut_ptr() as _,
                &mut used,
                &mut props,
            )
        }) {
            Ok(_) => {
                unsafe { self.0.set_len(props as usize) };
                Ok(props)
            }

            Err(InsufficientBuffer) => {
                self.resize(used);
                self.render(evt_handle, ctx)
            }

            Err(e) => Err(e),
        }
    }
}
