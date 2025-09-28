
#[derive(Debug)]
pub enum List {
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
        let list: List = List::More(Box::new(Node {
            elem: 1,
            next: List::More(Box::new(Node {
                elem: 2,
                next: List::More(Box::new(Node {
                    elem: 3,
                    next: List::Empty,
                })),
            })),
        }));
        println!("{:?}", list);
    }
}
