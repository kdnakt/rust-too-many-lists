
#[derive(Debug)]
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list: List = List::Elem(1, Box::new(List::Elem(2, Box::new(List::Empty))));
        println!("{:?}", list);
    }
}
