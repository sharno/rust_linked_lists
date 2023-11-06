use std::mem;

#[derive(Debug)]
struct Node<T> {
    value: T,
    rem: Link<T>,
}

#[derive(Debug, Default)]
enum Link<T> {
    #[default]
    Empty,
    Elem(Box<Node<T>>),
}

#[derive(Debug)]
pub struct List<T>(Link<T>);

impl<T> List<T> {
    pub fn new() -> Self {
        return List(Link::Empty);
    }

    pub fn cons(&mut self, value: T) {
        self.0 = Link::Elem(Box::new(Node {
            value,
            rem: mem::take(&mut self.0),
        }));
    }

    pub fn snoc(&mut self) -> Option<T> {
        if let Link::Elem(node) = mem::take(&mut self.0) {
            self.0 = node.rem;
            return Option::Some(node.value);
        }
        return Option::None;
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = mem::take(&mut self.0);
        while let Link::Elem(mut node) = curr {
            curr = mem::take(&mut node.rem);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        list.cons(1);
        list.cons(2);
        list.cons(3);

        assert_eq!(list.snoc(), Some(3));
    }
}
