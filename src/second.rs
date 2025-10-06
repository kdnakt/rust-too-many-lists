use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(boxed_node) => {
                self.head = boxed_node.next;
                Some(boxed_node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = List {
            head: Some(Box::new(Node {
                elem: 1,
                next: Some(Box::new(Node {
                    elem: 2,
                    next: None,
                })),
            })),
        };
        println!("{:?}", list);
        match &list {
            List {
                head: Some(boxed_node),
            } => {
                assert_eq!(boxed_node.elem, 1);
                match &boxed_node.next {
                    Some(boxed_node) => {
                        assert_eq!(boxed_node.elem, 2);
                        match boxed_node.next {
                            None => {}
                            _ => panic!("Expected empty link"),
                        }
                    }
                    _ => panic!("Expected second node"),
                }
            }
            _ => panic!("Expected first node"),
        }
    }

    #[test]
    fn push_and_pop() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
