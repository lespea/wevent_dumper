use std::mem::size_of;
use std::ptr::null_mut;
use std::time::Instant;
use std::time::SystemTime;

use log::{debug, error, info, trace, warn};
use widestring::{U16CStr, U16CString};
use winapi::shared::minwindef::*;
use winapi::um::winevt::*;
use winstructs::guid::Guid;
use winstructs::security::Sid;
use winstructs::timestamp::WinTimestamp;

use crate::errors::Result;
use crate::errors::WinEvtError::InsufficientBuffer;
use crate::utils;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use once_cell::sync::Lazy;
use std::ffi::{CStr, CString};
use winapi::shared::ntdef::{LPCSTR, LPCWSTR};

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
    Usize(usize),
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

static WIN_START_DATE: Lazy<NaiveDateTime> =
    Lazy::new(|| NaiveDate::from_ymd(1601, 1, 1).and_hms_nano(0, 0, 0, 0));

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

    #[inline]
    fn from_ts(ts: *const u64) -> Option<DateTime<Utc>> {
        Some(DateTime::from_utc(
            *WIN_START_DATE + Duration::microseconds((unsafe { *ts } / 10) as i64),
            Utc,
        ))
    }

    #[inline]
    fn from_ansi_str(s: *const LPCSTR) -> Option<String> {
        Some(unsafe { CStr::from_ptr(*s) }.to_string_lossy().into_owned())
    }

    #[inline]
    fn from_str(s: *const LPCWSTR) -> Option<String> {
        Some(unsafe { U16CStr::from_ptr_str(*s) }.to_string_lossy())
    }

    der!(usize, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64,);
}

#[repr(transparent)]
pub struct RawVariant(EVT_VARIANT);

macro_rules! var_transform {
    ($n:ident, $ty: ty, $t:ident, $c:ident, $f:ident, ) => {
        #[inline]
        pub fn $n(&self) -> Option<$ty> {
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
                    $r,
                    [<EvtVarType $c>],
                    [<$c Val>],
                    [<from_ $r>],
                );
            }
        )*
    };

    ($([$r: ident, $ty: ty, $c: ty, $f: ident],)+) => {
        $(
            paste::item! {
                var_transform!(
                    $r,
                    $ty,
                    [<EvtVarType $c>],
                    [<$c Val>],
                    $f,
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
        [usize, SizeT],
    );

    var_transform!(
        [filetime, DateTime<Utc>, FileTime, from_ts],
        [str, String, String, from_str],
        [ansi_str, String, AnsiString, from_ansi_str],
    );

    //    fn systime() -> Option<WinTimestamp>
    //    fn guid() -> Option<Guid>
    //    fn binary() -> Option<Vec<u8>>
    //    fn sid() -> Option<Sid>
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
pub struct VariantBuf(pub Vec<RawVariant>);

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
