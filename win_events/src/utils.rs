use winapi::um::winevt::EVT_HANDLE;

use crate::errors::WinEvtError;
use winapi::shared::minwindef::{BOOL, FALSE};

/// If the handle is null, then return an error otherwise we know it's okay
#[inline(always)]
pub fn not_null(e: EVT_HANDLE) -> Result<EVT_HANDLE, WinEvtError> {
    if e.is_null() {
        Err(WinEvtError::from_last_error())
    } else {
        Ok(e)
    }
}

/// If the return is 0 then it's an error
#[inline(always)]
pub fn check_okay(b: i32) -> Result<(), WinEvtError> {
    if b == 0 {
        Err(WinEvtError::from_last_error())
    } else {
        Ok(())
    }
}

/// If the return is false then it's an error
#[inline(always)]
pub fn check_bool(b: BOOL) -> Result<(), WinEvtError> {
    if b == FALSE {
        Err(WinEvtError::from_last_error())
    } else {
        Ok(())
    }
}
