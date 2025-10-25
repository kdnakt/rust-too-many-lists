use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut list: List<i32> = List::new();
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        list.push_front(1);
        assert!(list.head.is_some());
        assert!(list.head.as_ref().unwrap().borrow().elem == 1);
        assert!(list.tail.is_some());
        assert!(list.tail.as_ref().unwrap().borrow().elem == 1);
        list.push_front(2);
        assert!(list.head.is_some());
        assert!(list.head.as_ref().unwrap().borrow().elem == 2);
        assert!(list.tail.is_some());
        assert!(list.tail.as_ref().unwrap().borrow().elem == 1);
    }
}
