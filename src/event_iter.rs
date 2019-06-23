use std::collections::VecDeque;
use std::ptr::null_mut;

use widestring;
use winapi;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::NULL;
use winapi::um::winbase::INFINITE;
use winapi::um::winevt::{self, EvtClose, EvtNext, EvtQuery, EVT_HANDLE};

use crate::errors::WinEvtError;
use crate::win_event::WinEvent;
use winapi::shared::winerror;

const EVENTS_BUFFER: usize = 10;

pub struct WinEventsIter {
    query_handle: EVT_HANDLE,
    done: bool,
    events: VecDeque<Result<WinEvent, WinEvtError>>,
}

impl Iterator for WinEventsIter {
    type Item = Result<WinEvent, WinEvtError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        } else if !self.events.is_empty() {
            return self.events.pop_front();
        }

        let mut returned: DWORD = 0;
        let mut next: [EVT_HANDLE; EVENTS_BUFFER] = [null_mut(); EVENTS_BUFFER];

        let ret = unsafe {
            EvtNext(
                self.query_handle,
                EVENTS_BUFFER as u32,
                &mut next as *mut _ as _,
                INFINITE,
                0,
                &mut returned,
            )
        };

        if ret != 0 {
            self.done = true;

            if ret == winerror::ERROR_NO_MORE_ITEMS as i32 {
                None
            } else {
                Some(Err(WinEvtError::from_dword(ret)))
            }
        } else {
            self.events.extend(
                next.iter()
                    .take(returned as usize)
                    .map(move |&h| Ok(WinEvent::new(h))),
            );

            self.next()
        }
    }
}

impl Drop for WinEventsIter {
    fn drop(&mut self) {
        if unsafe { EvtClose(self.query_handle) } != 0 {
            panic!(format!(
                "Couldn't close a windows event handle: {}",
                WinEvtError::from_last_error(),
            ))
        }
    }
}

impl WinEventsIter {
    pub fn get_logs_for(name: &str, query: Option<&str>) -> Result<WinEventsIter, WinEvtError> {
        let path = widestring::U16String::from_str(name);

        let query = match query {
            None => std::ptr::null(),
            Some(q) => widestring::U16String::from_str(q).as_ptr(),
        };

        let handle = unsafe {
            EvtQuery(
                std::ptr::null_mut(),
                path.as_ptr(),
                query,
                winevt::EvtQueryChannelPath | winevt::EvtQueryForwardDirection,
            )
        };

        if handle == NULL {
            Err(WinEvtError::from_last_error())
        } else {
            Ok(WinEventsIter {
                query_handle: handle,
                events: VecDeque::with_capacity(EVENTS_BUFFER),
                done: false,
            })
        }
    }
}
