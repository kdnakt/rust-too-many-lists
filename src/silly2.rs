pub struct List<'a, T> {
    pub data: T,
    pub prev: Option<&'a List<'a, T>>,
}

impl<'a, T> List<'a, T> {
    pub fn push<U>(
        prev: Option<&'a List<'a, T>>,
        data: T,
        callback: impl FnOnce(&List<'a, T>) -> U,
    ) -> U {
        let list = List { data, prev };
        callback(&list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        List::push(None, 3, |list| {
            assert_eq!(list.data, 3);
            List::push(Some(list), 5, |list| {
                assert_eq!(list.data, 5);
                assert_eq!(list.prev.unwrap().data, 3);
            });
        });
    }
}
