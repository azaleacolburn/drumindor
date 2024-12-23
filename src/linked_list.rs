use crate::r#box::Box;
use core::fmt::Debug;

#[derive(Debug, Clone)]
pub struct LinkedList<T>
where
    T: Debug + Clone,
{
    value: T,
    next: Option<Box<LinkedList<T>>>,
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

    pub fn push(&mut self, value: T) -> LinkedList<T> {
        let anchor = self;
        while self.next.unwrap().value().next.is_some() {
            self = self.next.unwrap().value().next;
        }

        anchor.
    }
}
