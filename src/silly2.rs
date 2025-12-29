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

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter { next: Some(self) }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a List<'a, T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.prev;
            &node.data
        })
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

    #[test]
    fn iter() {
        List::push(None, 3, |list| {
            assert_eq!(list.iter().copied().sum::<i32>(), 3);
            List::push(Some(list), 5, |list| {
                assert_eq!(list.iter().copied().sum::<i32>(), 8);
                List::push(Some(list), 11, |list| {
                    assert_eq!(list.iter().copied().sum::<i32>(), 19);
                });
            });
        });
    }
}
