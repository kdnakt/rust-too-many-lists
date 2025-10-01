use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

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
            head: Link::More(Box::new(Node {
                elem: 1,
                next: Link::More(Box::new(Node {
                    elem: 2,
                    next: Link::Empty,
                })),
            })),
        };
        println!("{:?}", list);
        match list {
            List {
                head: Link::More(boxed_node),
            } => {
                assert_eq!(boxed_node.elem, 1);
                match boxed_node.next {
                    Link::More(boxed_node) => {
                        assert_eq!(boxed_node.elem, 2);
                        match boxed_node.next {
                            Link::Empty => {}
                            _ => panic!("Expected empty link"),
                        }
                    },
                    _ => panic!("Expected second node"),
                }
            }
            _ => panic!("Expected first node"),
        }
    }
}
