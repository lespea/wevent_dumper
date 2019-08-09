use winapi::um::winevt::{EvtClose, EVT_HANDLE};

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
    pub fn new(handle: EVT_HANDLE) -> Self {
        WinEvent { handle }
    }
}
