use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}
