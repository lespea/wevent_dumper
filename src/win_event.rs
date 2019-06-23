use crate::errors::WinEvtError;
use winapi::um::winevt::{EvtClose, EVT_HANDLE};

pub struct WinEvent {
    handle: EVT_HANDLE,
}

impl Drop for WinEvent {
    fn drop(&mut self) {
        if unsafe { EvtClose(self.handle) } != 0 {
            panic!(format!(
                "Couldn't close a windows event handle: {}",
                WinEvtError::from_last_error(),
            ))
        }
    }
}

impl WinEvent {
    pub fn new(handle: EVT_HANDLE) -> Self {
        WinEvent { handle }
    }
}
