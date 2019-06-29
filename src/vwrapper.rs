use std::alloc;
use std::ptr;

use std::ops::Deref;
use winapi::um::winevt::EVT_VARIANT;
use winapi::um::winevt::PEVT_VARIANT;

pub struct WevWrapper {
    pointer: PEVT_VARIANT,
    size: usize,
    align: usize,
    layout: alloc::Layout,
}

impl WevWrapper {
    pub fn new() -> Result<Self, alloc::LayoutErr> {
        WevWrapper::sized(1024 * 4)
    }

    pub fn sized(size: usize) -> Result<Self, alloc::LayoutErr> {
        let align = alloc::Layout::new::<EVT_VARIANT>().align();
        let layout = alloc::Layout::from_size_align(size, align)?;

        if size == 0 {
            return Ok(WevWrapper {
                pointer: ptr::null_mut(),
                size: 0,
                align,
                layout,
            });
        }

        let bytes = unsafe { alloc::alloc_zeroed(layout) };

        if bytes == ptr::null_mut() {
            panic!("Couldn't allocate a windows event variant object")
        }

        Ok(WevWrapper {
            pointer: bytes as PEVT_VARIANT,
            size,
            align,
            layout,
        })
    }

    pub fn get_pointer<'a, 'b: 'a>(&'b mut self) -> (&'a mut EVT_VARIANT, usize) {
        (unsafe { self.pointer.as_mut().unwrap() }, self.size)
    }

    pub fn resize(&mut self, new_size: usize) -> Result<(), alloc::LayoutErr> {
        unsafe {
            self.dealloc();

            if new_size == 0 {
                self.pointer = ptr::null_mut();
                self.size = 0;
            } else {
                let layout = alloc::Layout::from_size_align(new_size, self.align)?;
                self.layout = layout;

                self.pointer = alloc::alloc_zeroed(layout) as PEVT_VARIANT;

                if self.pointer == ptr::null_mut() {
                    panic!("Couldn't allocate a windows event variant object")
                }
            }
        };

        Ok(())
    }

    unsafe fn dealloc(&mut self) {
        if self.size > 0 {
            alloc::dealloc(self.pointer as *mut u8, self.layout);
        }
    }
}

impl Deref for WevWrapper {
    type Target = EVT_VARIANT;

    fn deref(&self) -> &Self::Target {
        assert_ne!(self.size, 0);
        unsafe { self.pointer.as_ref() }.unwrap()
    }
}

impl Drop for WevWrapper {
    fn drop(&mut self) {
        unsafe { self.dealloc() };
    }
}
