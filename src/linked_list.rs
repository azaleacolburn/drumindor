use core::{fmt::Debug, ptr::NonNull};

#[derive(Debug, Clone)]
pub struct LinkedList<T>
where
    T: Debug + Clone,
{
    value: T,
    next: Option<NonNull<LinkedList<T>>>,
}

impl<T> LinkedList<T>
where
    T: Debug + Clone,
{
    pub fn new(start: T) -> LinkedList<T> {
        LinkedList {
            value: start,
            next: None,
        }
    }

    pub fn push(&mut self, value: T) {
        let mut ptr: NonNull<LinkedList<T>> = unsafe { NonNull::new_unchecked(self) };
        loop {
            match unsafe { ptr.read().next } {
                Some(val) => ptr = val,
                None => break,
            }
        }

        self.next = Some(unsafe {
            NonNull::new_unchecked(&mut LinkedList::new(value) as *mut LinkedList<T>)
        });
    }

    /// Removes node at index, connecting the two halfs of the list, self becomes the value
    /// *before* prev in the list
    pub fn remove(&mut self, index: usize) {
        let mut ptr: NonNull<LinkedList<T>> = unsafe { NonNull::new_unchecked(self) };
        for _ in 0..index - 1 {
            match unsafe { ptr.read().next } {
                Some(val) => ptr = val,
                None => return panic!("Index exceeds size of linked list"),
            }
        }
        let before_node = unsafe { ptr.read() };
        match before_node.next {
            Some(val) => {
                let removed_val = unsafe { val.read() };
                match removed_val.next {
                    Some(next) => before_node.next = Some(next),
                    None => before_node.next = None,
                }
            }
            None => return panic!("Index exceeds size of linked list"),
        }
    }

    /// Replaces the value in self
    pub fn replace(&mut self, value: T) {
        self.value = value;
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn is_head(&self) -> bool {
        self.next.is_some()
    }

    /// Sets self to be self.next
    /// Returns a reference to self.next.value
    /// If self is head, None is returned and self is not modified
    pub fn next(&mut self) -> Option<&T> {
        match self.next {
            Some(node) => {
                *self = unsafe { node.read() };
                Some(&self.value)
            }
            None => None,
        }
    }
}
