use std::mem::size_of;
use std::ptr::null_mut;
use std::time::Instant;
use std::time::SystemTime;

use log::{debug, error, info, trace, warn};
use winapi::shared::minwindef::*;
use winapi::um::winevt::*;
use winstructs::guid::Guid;
use winstructs::security::Sid;
use winstructs::timestamp::WinTimestamp;

use crate::errors::Result;
use crate::errors::WinEvtError::InsufficientBuffer;
use crate::utils;

pub enum Variant {
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    F32(f32),
    F64(f64),
    Filetime(WinTimestamp),
    Systime(WinTimestamp),
    Guid(Guid),
    String(String),
    AnsiStr(String),
    Binary(Vec<u8>),
    Sid(Sid),
    Size(usize),
    Xml(String),
    EvtHandle(EVT_HANDLE),

    Array(Vec<Variant>),
}

macro_rules! der {
    ($t:ident) => (
        paste::item! {
            #[inline]
            fn [<from_ $t>](b: *const $t) -> Option<$t>{
                if b.is_null(){
                    None
                } else {
                    Some(unsafe{*b})
                }
            }
        }
    );

    ($($ts:ident,)+) => ($(der!($ts);)*);
}

#[allow(clippy::trivially_copy_pass_by_ref)]
impl Variant {
    #[inline]
    fn from_bool(b: *const BOOL) -> Option<bool> {
        if b.is_null() {
            None
        } else {
            Some(unsafe { *b } != 0)
        }
    }

    der!(
        i8, i16, i32, i64,
        u8, u16, u32, u64,
        f32, f64,
    );
}

#[repr(transparent)]
pub struct RawVariant(EVT_VARIANT);

macro_rules! var_transform {
    ($n:tt, $t:tt, $c:tt, $f:tt, ) => {
        #[inline]
        pub fn $n(&self) -> Option<$n> {
            if (self.0.Type == $t) {
                Variant::$f(unsafe{self.0.u.$c()})
            } else {
                None
            }
        }
    };

    ($([$r: ident, $c: ident],)+) => {
        $(
            paste::item! {
                var_transform!(
                    $r,
                    [<EvtVarType $c>],
                    [<$c Val>],
                    [<from_ $r>],
                );
            }
        )*
    };
}

impl RawVariant {
    var_transform!(
        [bool, Boolean],
        [u8, Byte],
        [i8, SByte],
        [i16, Int16],
        [i32, Int32],
        [i64, Int64],
        [u16, UInt16],
        [u32, UInt32],
        [u64, UInt64],
        [f32, Single],
        [f64, Double],
    );

    //    fn bool() -> Option<bool> {
    //
    //    }

    //    fn int8() -> Option<i8>
    //    fn int16() -> Option<i16>
    //    fn int32() -> Option<i32>
    //    fn int64() -> Option<i64>
    //    fn byte() -> Option<u8>
    //    fn uint8() -> Option<u8>
    //    fn uint16() -> Option<u16>
    //    fn uint32() -> Option<u32>
    //    fn uint64() -> Option<u64>
    //    fn single() -> Option<f32>
    //    fn double() -> Option<f64>
    //    fn filetime() -> Option<WinTimestamp>
    //    fn systime() -> Option<WinTimestamp>
    //    fn guid() -> Option<Guid>
    //    fn string() -> Option<String>
    //    fn ansiStr() -> Option<String>
    //    fn binary() -> Option<Vec<u8>>
    //    fn sid() -> Option<Sid>
    //    fn size() -> Option<usize>
    //    fn xml() -> Option<String>
}

impl Drop for RawVariant {
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
pub struct VariantBuf(Vec<RawVariant>);

const VSIZE: usize = size_of::<RawVariant>();

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
