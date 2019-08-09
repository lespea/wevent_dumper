use winapi::shared::ntdef::NULL;
use winapi::shared::winerror;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winevt::EVT_HANDLE;

use crate::errors::{WinError, WinEvtError};

#[inline(always)]
pub fn not_null(e: EVT_HANDLE) -> Result<EVT_HANDLE, WinEvtError> {
    if e == NULL {
        Err(WinEvtError::from_last_error())
    } else {
        Ok(e)
    }
}

#[inline(always)]
pub fn check_okay(b: i32) -> Result<(), WinEvtError> {
    if b == 0 {
        Err(WinEvtError::from_last_error())
    } else {
        Ok(())
    }
}

#[inline(always)]
pub fn check_okay_check(b: i32) -> Result<(), WinError> {
    if b == 0 {
        Err(match unsafe { GetLastError() } {
            winerror::ERROR_INSUFFICIENT_BUFFER => WinError::InsufficientBuffer,
            winerror::ERROR_NO_MORE_ITEMS => WinError::NoMoreItems,
            e => WinError::Err(WinEvtError::from_dword(e)),
        })
    } else {
        Ok(())
    }
}
