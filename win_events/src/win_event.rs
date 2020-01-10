use std::mem::size_of;
use winapi::_core::ptr::null_mut;
use winapi::um::winevt::*;

use crate::utils;
use widestring::{U16CString, U16String};

/// Holds on to a evt handle to make sure we cleanup after we're done with it
pub struct WinEvent {
    handle: EVT_HANDLE,
    vals: Vec<EVT_VARIANT>,
}

impl Drop for WinEvent {
    fn drop(&mut self) {
        crate::utils::check_okay(unsafe { EvtClose(self.handle) })
            .expect("Couldn't close a windows event handle")
    }
}

impl WinEvent {
    /// Hold on to a evt handle
    pub fn new(handle: EVT_HANDLE) -> Self {
        WinEvent {
            handle,
            vals: Vec::with_capacity(1 << 13),
        }
    }

    pub fn test(&mut self, counts: &mut [u64; 64]) {
        let ctx =
            utils::not_null(unsafe { EvtCreateRenderContext(0, null_mut(), EvtRenderContextUser) })
                .expect("no ctx");

        let mut used = 0;
        let mut props = 0;

        if let Err(e) = utils::check_okay(unsafe {
            EvtRender(
                ctx,
                self.handle,
                EvtRenderEventValues,
                (self.vals.capacity() * size_of::<EVT_VARIANT>()) as u32,
                self.vals.as_mut_ptr() as _,
                &mut used,
                &mut props,
            )
        }) {
            eprintln!("{}", e);
        }

        unsafe { self.vals.set_len(props as usize) }

        for v in self.vals.iter() {
            //            println!("{}: {}/{}", i, v.Type, v.Count);

            let rt = v.Type & EVT_VARIANT_TYPE_MASK;
            let is_arr = v.Type & EVT_VARIANT_TYPE_ARRAY > 0;

            if rt > 63 {
                println!("Weird type: {} / {} / {}", v.Type, rt, is_arr)
            } else {
                counts[rt as usize] += 1;
            }
            //            unsafe{counts.get_unchecked_mut(rt as usize)} += 1;

            if is_arr {
                counts[63] += 1;
                //                unsafe{counts.get_unchecked_mut(63)} += 1;
            }

            //            if v.Type == 1 {
            //                let s = unsafe {
            //                    U16String::from_ptr(*v.u.StringVal(), v.Count as usize)
            //                };
            //
            //                println!("{}", s.to_string_lossy());
            //            }
        }
    }
}
