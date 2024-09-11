/*
Leaving this here for posterity, it's just not good.
*/

use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct CircularList<T> {
    head: Option<Weak<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}

struct Node<T> {
    elem: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> CircularList<T> {
    pub fn new() -> CircularList<T> {
        CircularList {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.head.is_none() {
            let head_node = Rc::new(RefCell::new(Node {
                elem: elem,
                next: None,
            }));
            head_node.borrow_mut().next = Some(head_node.clone());
            self.head = Some(Rc::downgrade(&head_node));
            self.tail = Some(Rc::downgrade(&head_node));
            drop(head_node);
        } else {
            let new_node = Rc::new(RefCell::new(Node {
                elem: elem,
                next: self.tail.as_ref().unwrap().upgrade(),
            }));

            if let Some(head_node) = self.head.as_ref().unwrap().upgrade() {
                head_node.borrow_mut().next = Some(new_node.clone());
            }
            
            self.tail = Some(Rc::downgrade(&new_node));
            drop(new_node);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else if self.head.as_ref().unwrap().ptr_eq(&self.tail.as_ref().unwrap()) {
            if let Some(head_node) = self.head.take().unwrap().upgrade() {
                self.tail = None;
                head_node.borrow_mut().next = None;
                if let Ok(r) = Rc::try_unwrap(head_node) {
                    let node = r.into_inner();
                    return Some(node.elem);
                } else {
                    panic!("Something went wrong 0.");
                }
            } else {
                panic!("Something went wrong 1.")
            }
        } else {
            if let Some(head_node) = self.head.as_ref().unwrap().upgrade() {
                self.tail.take().map(|weak_ref| {
                    let upgraded = weak_ref.upgrade().unwrap();
                    head_node.borrow_mut().next = upgraded.borrow().next.clone();
                    if let Ok(r) = Rc::try_unwrap(upgraded) {
                        let node = r.into_inner();
                        self.tail = Some(Rc::downgrade(&node.next.unwrap()));

                        node.elem
                    } else {
                        panic!("Something went wrong 2");
                    }
                })
            } else {
                panic!("Something went wrong 3")
            }
        }
    }
}

impl<T> Drop for CircularList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::CircularList;

    #[test]
    fn add_one() {
        let mut l = CircularList::new();
        l.push(1);
        assert_eq!(Some(1), l.pop());
    }

    #[test]
    fn add_stuff() {
        let mut l: CircularList<i32> = CircularList::new();
        l.push(1); l.push(2); l.push(3);

        assert_eq!(Some(3), l.pop());
        assert_eq!(Some(2), l.pop());
        assert_eq!(Some(1), l.pop());
        assert_eq!(None, l.pop());
    }

    #[test]
    fn pop_none() {
        let mut l: CircularList<i32> = CircularList::new();
        assert_eq!(None, l.pop());
    }

    #[test]
    fn test_drop() {
        let mut l: CircularList<i32> = CircularList::new();
        l.push(1); l.push(2); l.push(3);
    }
}