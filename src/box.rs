use crate::alloc;
use core::{alloc::Layout, fmt::Debug, ptr::NonNull};

#[derive(Debug, Clone)]
pub struct Box<T>
where
    T: Debug + Clone,
{
    ptr: NonNull<T>,
}

impl<T> Box<T>
where
    T: Debug + Clone,
{
    pub fn new(value: T) -> Box<T> {
        let layout = Layout::new::<T>();
        unsafe {
            let raw = alloc(layout) as *mut T;
            if raw.is_null() {
                panic!("Null ptr returned when allocating box");
            }
            raw.write(value);
            Box {
                ptr: NonNull::new_unchecked(raw),
            }
        }
    }

    pub fn value(&self) -> T {
        unsafe { self.ptr.read() }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}
