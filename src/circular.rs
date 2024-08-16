use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct CircularList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

struct Node<T> {
    elem: T,
    next: Option<Weak<RefCell<Node<T>>>>
}

impl<T> CircularList<T> {
    pub fn new() -> CircularList<T> {
        CircularList {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, elem: T) {
        match &self.head {
            Some(n) => {
                let new_node = Node {
                    elem: elem,
                    next: self.head.take().map(|r| Rc::downgrade(&r)),
                };

                self.head = Some(Rc::new(RefCell::new(new_node)));
            },
            None => {
                self.head = Some(Rc::new(RefCell::new(Node {
                    elem: elem,
                    next: None,
                })));
                self.head.as_ref().unwrap().borrow_mut().next = Some(Rc::downgrade(&self.head.as_ref().unwrap()));
                self.tail = self.head.as_ref().map(|r| r.clone());
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|r| {
            self.head = r.borrow().next.as_ref().map(|r| r.upgrade().unwrap());
            r.to_owned().borrow().elem
        })
    }
}