
#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
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
    next: List,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = List {
            head: Link::More(Box::new(Node {
                elem: 1,
                next: List {
                    head: Link::More(Box::new(Node {
                        elem: 2,
                        next: List::new(),
                    })),
                },
            })),
        };
        println!("{:?}", list);
        match list {
            List {
                head: Link::More(boxed_node),
            } => {
                assert_eq!(boxed_node.elem, 1);
                match boxed_node.next {
                    List {
                        head: Link::More(boxed_node2),
                    } => {
                        assert_eq!(boxed_node2.elem, 2);
                        match boxed_node2.next {
                            List { head: Link::Empty } => {}
                            _ => panic!("Expected empty list"),
                        }
                    }
                    _ => panic!("Expected second node"),
                }
            }
            _ => panic!("Expected first node"),
        }
    }
}
