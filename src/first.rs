
#[derive(Debug)]
pub struct List {
    head: Link,
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
                        next: List { head: Link::Empty },
                    })),
                },
            })),
        };
        println!("{:?}", list);
    }
}
