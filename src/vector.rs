use core::{
    alloc::Layout,
    fmt::{Debug, Display},
};

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
            last.write(std::mem::zeroed());
            self.len -= 1;

            return Some(val);
        }
    }

    fn resize(&mut self) {
        self.cap *= 3;
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
}

impl<T: Sized + Clone + Debug> Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buff = String::from("[");
        for i in 0..self.len {
            unsafe {
                buff.push_str(format!("{:?}, ", self.arr.add(i).read()).as_str());
            }
        }
        // TODO: Implement recursive solution to fix this
        buff.pop();
        buff.pop();
        buff.push(']');
        write!(f, "{}", buff)
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
