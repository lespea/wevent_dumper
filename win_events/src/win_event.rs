use winapi::um::winevt::{EvtClose, EVT_HANDLE};

/// Holds on to a evt handle to make sure we cleanup after we're done with it
pub struct WinEvent {
    pub(crate) handle: EVT_HANDLE,
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
        WinEvent { handle }
    }
}
