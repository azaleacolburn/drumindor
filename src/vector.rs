use crate::{alloc, dealloc};
use core::{alloc::Layout, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Vector<T>
where
    T: Sized + Clone + Debug,
{
    arr: *mut T,
    len: usize,
    cap: usize,
    layout: Layout,
}

impl<T: Sized + Clone + Debug> Vector<T> {
    pub fn new() -> Vector<T> {
        let cap = 4;
        let layout = Layout::array::<T>(cap).unwrap();
        let arr = unsafe { alloc(layout) as *mut T };
        if arr.is_null() {
            panic!("Allocating new array failed");
        }
        Vector {
            arr,
            len: 0,
            cap,
            layout,
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
            last.write(core::mem::zeroed());
            self.len -= 1;

            return Some(val);
        }
    }

    pub fn set(&mut self, item: T, pos: usize) {
        if self.cap <= pos {
            self.set_cap(pos);
            self.len = pos + 1;
        }

        unsafe {
            self.arr.add(pos).write(item);
        }
    }

    pub fn get(&self, pos: usize) -> T {
        if self.len <= pos {
            panic!("Position out of bounds of Vector");
        }

        unsafe { self.arr.add(pos).read() }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn set_cap(&mut self, cap: usize) {
        self.cap = cap;
        let layout = Layout::array::<T>(self.cap).unwrap();
        let new_ptr = unsafe { alloc(layout) as *mut T };
        if new_ptr.is_null() {
            panic!("Allocating new array failed");
        }

        for i in 0..self.len {
            unsafe {
                new_ptr.add(i).write(self.arr.add(i).read().clone());
            }
        }
        unsafe {
            dealloc(self.arr as *mut u8, self.layout);
        }
        self.arr = new_ptr;
        self.layout = layout;
    }

    fn resize(&mut self) {
        self.set_cap(self.cap * 3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_basic() {
        let mut vec = Vector::new();
        vec.push(3);
        vec.push(2);
        vec.push(1);
        assert_eq!(vec.len, 3);
        vec.push(0);
        assert_eq!(vec.cap, 12);
    }
}
