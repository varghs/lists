use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct CircularList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}

struct Node<T> {
    elem: T,
    next: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> CircularList<T> {
    pub fn new() -> CircularList<T> {
        CircularList {
            head: None,
            tail: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CircularList;

    #[test]
    fn add_stuff() {
        let mut l: CircularList<i32> = CircularList::new();
        l.push(1); l.push(2); l.push(3);

        assert_eq!(Some(3), l.pop());
        assert_eq!(Some(2), l.pop());
        assert_eq!(Some(1), l.pop());
    }
}