use crate::ALLOCATOR;
use core::{
    alloc::{GlobalAlloc, Layout},
    fmt::Debug,
    mem, ptr,
};

#[derive(Debug, Clone)]
pub struct Vector<T>
where
    T: Sized + Clone + Debug,
{
    arr: *mut T,
    len: usize,
    cap: usize,
}

unsafe impl<T: Debug + Clone + Send> Send for Vector<T> {}
unsafe impl<T: Debug + Clone + Sync> Sync for Vector<T> {}

impl<T: Sized + Clone + Debug> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector {
            // We just checked
            arr: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len + 1 == self.cap {
            self.resize();
        }
        unsafe {
            self.arr.add(self.len).write(item);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            let last = self.arr.add(self.len - 1);
            let val = last.read();
            last.write(mem::zeroed());
            self.len -= 1;

            return Some(val);
        }
    }

    fn resize(&mut self) {
        let old_layout = Layout::array::<T>(self.cap).unwrap();
        self.cap = if self.cap == 0 { 4 } else { self.cap * 3 };
        let layout = Layout::array::<T>(self.cap).unwrap();
        let new_ptr = unsafe { ALLOCATOR.alloc(layout) as *mut T };
        if new_ptr.is_null() {
            panic!("Allocating new array failed");
        }

        if !self.arr.is_null() {
            unsafe {
                for i in 0..self.len {
                    new_ptr.add(i).write(self.arr.add(i).read().clone());
                }

                ALLOCATOR.dealloc(self.arr as *mut u8, old_layout);
            }
        }

        self.arr = new_ptr;
    }
}

impl<T: Debug + Clone> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.arr.is_null() {
            return;
        }

        unsafe {
            let layout = Layout::array::<T>(self.cap).unwrap();
            ALLOCATOR.dealloc(self.arr as *mut u8, layout);
        }
    }
}

// impl<T: Sized + Clone + Debug> Display for Vector<T> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         let mut buff = String::from("[");
//         for i in 0..self.len {
//             unsafe {
//                 buff.push_str(format!("{:?}, ", self.arr.add(i).read()).as_str());
//             }
//         }
//         // TODO: Implement recursive solution to fix this
//         buff.pop();
//         buff.pop();
//         buff.push(']');
//         write!(f, "{}", buff)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::ALLOCATOR;

    use super::*;

    #[test]
    fn test_vector_basic() {
        unsafe {
            ALLOCATOR.init();
        }

        let mut vec = Vector::new();
        vec.push(3);
        vec.push(2);
        vec.push(1);
        assert_eq!(vec.len, 3);
        vec.push(0);
        assert_eq!(vec.cap, 12);
    }
}
