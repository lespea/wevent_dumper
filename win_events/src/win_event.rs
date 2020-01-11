use std::mem::size_of;
use winapi::_core::ptr::null_mut;
use winapi::um::winevt::*;

use crate::utils;
use crate::variant::VariantBuf;
use widestring::{U16CString, U16String};

/// Holds on to a evt handle to make sure we cleanup after we're done with it
pub struct WinEvent(EVT_HANDLE);

impl Drop for WinEvent {
    fn drop(&mut self) {
        crate::utils::check_okay(unsafe { EvtClose(self.0) })
            .expect("Couldn't close a windows event handle")
    }
}

impl WinEvent {
    /// Hold on to a evt handle
    pub fn new(handle: EVT_HANDLE) -> Self {
        WinEvent(handle)
    }

    pub fn test(&mut self, buf: &mut VariantBuf, counts: &mut [u64; 100]) {
        let ctx =
            utils::not_null(unsafe { EvtCreateRenderContext(0, null_mut(), EvtRenderContextUser) })
                .expect("no ctx");

        buf.render(self.0, ctx).expect("Couldn't get evt info");

        //        for v in buf.0.iter() {
        //            //            println!("{}: {}/{}", i, v.Type, v.Count);
        //
        //            let rt = v.Type & EVT_VARIANT_TYPE_MASK;
        //            let is_arr = v.Type & EVT_VARIANT_TYPE_ARRAY > 0;
        //
        //            if rt >= 50 {
        //                println!("Weird type: {} / {} / {}", v.Type, rt, is_arr)
        //            } else {
        //                if is_arr {
        //                    counts[50 + rt as usize] += 1;
        //                } else {
        //                    counts[rt as usize] += 1;
        //                }
        //            }
        //
        //            //            if v.Type == 1 {
        //            //                let s = unsafe {
        //            //                    U16String::from_ptr(*v.u.StringVal(), v.Count as usize)
        //            //                };
        //            //
        //            //                println!("{}", s.to_string_lossy());
        //            //            }
        //        }
    }
}
