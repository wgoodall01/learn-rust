use std::rc::Rc;

// ----- List implementation -----
pub struct List<T> {
    head: Option<Rc<Node<T>>>,
}

pub struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                value: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|n| n.next.clone()),
        }
    }

    pub fn head<'a>(&'a self) -> Option<&'a T> {
        self.head.as_ref().map(|n| &n.value)
    }

    pub fn split(&self) -> (Option<&T>, List<T>) {
        (self.head(), self.tail())
    }
}

// TODO: Why does this work, and #[derive(Clone)] not work?
impl<T> Clone for List<T> {
    fn clone(&self) -> List<T> {
        List {
            head: self.head.clone(),
        }
    }
}

// ----- Iter -----
pub struct Iter<'a, T> {
    node: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            node: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.map(|node| {
            self.node = node.next.as_ref().map(|n| &**n);
            &node.value
        })
    }
}

// ----- Drop -----
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut latest: Option<Rc<Node<T>>> = self.head.take();
        while let Some(rf) = latest {
            match Rc::try_unwrap(rf) {
                Ok(mut node) => latest = node.next.take(),
                Err(_) => break,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn list_new() {
        let mut _list: List<i32> = List::new();
    }

    #[test]
    fn list_head_tail() {
        let mut list: List<i32> = List::new();
        for x in 1..=10 {
            list = list.push(x);
        }

        for x in (1..=10).rev() {
            assert_eq!(list.head(), Some(&x));
            list = list.tail();
        }
    }

    #[test]
    fn list_iter() {
        let mut list: List<i32> = List::new();
        for x in 1..=10 {
            list = list.push(x);
        }

        let mut x: i32 = 10;
        for val in list.iter() {
            assert_eq!(val, &x);
            x -= 1;
        }
    }

}
