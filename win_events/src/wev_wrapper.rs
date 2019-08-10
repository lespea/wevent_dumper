use std::alloc;
use std::ops::Deref;
use std::ptr;

use slog::Logger;
use winapi::um::winevt;
use winapi::um::winevt::PEVT_VARIANT;
use winapi::um::winevt::{EvtClose, EVT_VARIANT};

use crate::errors::WinEvtError;

/// Pointer to an underlying evt_variant struct.
///
/// The underlying variant can itself be a pointer so you must call close_evt before
/// you use this again to make sure that gets cleaned up if it was.  It will be cleaned
/// when this is dropped.
pub struct WevWrapper {
    pointer: PEVT_VARIANT,
    size: usize,
    align: usize,
    layout: alloc::Layout,
    log: slog::Logger,
}

impl WevWrapper {
    /// Create a sized wrapper with a starting buffer of 4k
    pub fn new(log: Logger) -> Result<Self, alloc::LayoutErr> {
        WevWrapper::sized(1024 * 4, log)
    }

    /// Creates a wrapper with a starting buffer of the provided size
    pub fn sized(size: usize, log: Logger) -> Result<Self, alloc::LayoutErr> {
        let align = alloc::Layout::new::<EVT_VARIANT>().align();
        let layout = alloc::Layout::from_size_align(size, align)?;

        // Don't allocate anything if nothing was asked for
        if size == 0 {
            debug!(log, "Allocating a wev wrapper of size 0");

            return Ok(WevWrapper {
                pointer: ptr::null_mut(),
                size: 0,
                align,
                layout,
                log,
            });
        }

        let bytes = unsafe { alloc::alloc_zeroed(layout) };

        if bytes.is_null() {
            crit!(log, "Couldn't allocate a windows event variant object");
            panic!("Couldn't allocate a windows event variant object")
        }

        Ok(WevWrapper {
            pointer: bytes as PEVT_VARIANT,
            size,
            align,
            layout,
            log,
        })
    }

    /// Checks if the variant is a pointer itself and if it is, closes it
    ///
    /// You *must* call this before you reuse the underlying pointer!
    pub unsafe fn close_evt(&self) -> Result<(), WinEvtError> {
        if !self.pointer.is_null() {
            debug!(self.log, "Closing a wev wrapper");
            let evt = *self.pointer;
            if evt.Type == winevt::EvtVarTypeEvtHandle {
                let evt = *evt.u.EvtHandleVal();
                if !evt.is_null() {
                    debug!(self.log, "Closing underlying evt handle");
                    return crate::utils::check_okay(EvtClose(evt));
                } else {
                    debug!(self.log, "Underlying evt handle was null");
                }
            }
        } else {
            debug!(self.log, "Closing a null wrapper; doing nothing");
        }

        Ok(())
    }

    /// Assumes the variant is a pointer and closes it
    pub fn close_evt_unchecked(&self) -> Result<(), WinEvtError> {
        debug!(self.log, "Closing the evt handle unchecked");
        let evt = unsafe { *(*self.pointer).u.EvtHandleVal() };
        if !evt.is_null() {
            crate::utils::check_okay(unsafe { EvtClose(evt) })
        } else {
            Ok(())
        }
    }

    /// Gets a pointer to the underlying variant and returns how big it is
    pub unsafe fn get_pointer<'a, 'b: 'a>(&'b mut self) -> (&'a mut EVT_VARIANT, usize) {
        debug!(self.log, "Getting the wev pointer");
        (self.pointer.as_mut().unwrap(), self.size)
    }

    /// Grows the variant to the requested size.
    ///
    /// This clears the underlying memory first (including the pointer if it's that type of variant)
    pub unsafe fn resize(&mut self, new_size: usize) -> Result<(), alloc::LayoutErr> {
        self.dealloc();

        // If we want a zero sized variant then just set everything to nothing
        if new_size == 0 {
            self.pointer = ptr::null_mut();
            self.size = 0;
        } else {
            let layout = alloc::Layout::from_size_align(new_size, self.align)?;
            self.layout = layout;

            self.pointer = alloc::alloc_zeroed(layout) as PEVT_VARIANT;

            if self.pointer.is_null() {
                crit!(self.log, "Couldn't allocate a windows event variant object");
                panic!("Couldn't allocate a windows event variant object");
            }
        }

        Ok(())
    }

    /// Frees the underlying memory if there was anything alloc'd
    unsafe fn dealloc(&mut self) {
        if self.size > 0 {
            if let Err(e) = self.close_evt() {
                error!(self.log, "Error dropping a wev wrapper: {}", e);
            };
            alloc::dealloc(self.pointer as *mut u8, self.layout);
        }
    }
}

impl Deref for WevWrapper {
    type Target = EVT_VARIANT;

    fn deref(&self) -> &Self::Target {
        assert_ne!(self.size, 0);
        unsafe { self.pointer.as_ref() }.unwrap()
    }
}

impl Drop for WevWrapper {
    fn drop(&mut self) {
        if let Err(e) = unsafe { self.close_evt() } {
            error!(self.log, "Error dropping a wev wrapper: {}", e);
        }
        unsafe { self.dealloc() };
    }
}
