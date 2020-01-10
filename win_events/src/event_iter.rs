use std::collections::VecDeque;
use std::ptr;

use widestring;
use winapi;
use winapi::um::winbase::INFINITE;
use winapi::um::winevt::{self, EvtClose, EvtNext, EvtQuery, EVT_HANDLE};

use crate::errors::WinEvtError;
use crate::utils;
use crate::win_event::WinEvent;

const EVENTS_BUFFER: usize = 1024;

pub struct WinEventsIter {
    handle: EVT_HANDLE,
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

        let mut returned = 0;
        let mut next: Vec<EVT_HANDLE> = vec![ptr::null_mut(); EVENTS_BUFFER];

        if let Err(e) = utils::check_okay(unsafe {
            EvtNext(
                self.handle,
                EVENTS_BUFFER as u32,
                next.as_mut_ptr(),
                INFINITE,
                0,
                &mut returned,
            )
        }) {
            return match e {
                WinEvtError::NoMoreItems => {
                    self.done = true;
                    None
                }
                _ => {
                    self.done = true;
                    Some(Err(e))
                }
            };
        }

        self.events.extend(
            next.iter()
                .take(returned as usize)
                .map(|&h| Ok(WinEvent::new(h))),
        );

        self.events.pop_front()
    }
}

impl Drop for WinEventsIter {
    fn drop(&mut self) {
        crate::utils::check_okay(unsafe { EvtClose(self.handle) })
            .expect("Couldn't close the windows event query handle")
    }
}

impl WinEventsIter {
    pub fn get_logs_for(name: &str, query: Option<&str>) -> Result<WinEventsIter, WinEvtError> {
        let path = widestring::U16CString::from_str(name).expect("Invalid channel");

        let query = match query {
            None => ptr::null(),
            Some(q) => widestring::U16CString::from_str(q)
                .expect("Invalid query")
                .as_ptr(),
        };

        let handle = utils::not_null(unsafe {
            EvtQuery(
                ptr::null_mut(),
                path.as_ptr(),
                query,
                winevt::EvtQueryChannelPath | winevt::EvtQueryForwardDirection,
            )
        })?;

        Ok(WinEventsIter {
            handle,
            events: VecDeque::with_capacity(EVENTS_BUFFER),
            done: false,
        })
    }
}
