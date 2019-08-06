// ----- Basic implementation of list -----

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn length_after(&self) -> u32 {
        match &self.next {
            None => 0,
            Some(node) => 1 + node.length_after(),
        }
    }
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, el: T) {
        let new = Node {
            value: el,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|first| {
            self.head = first.next;
            first.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|first| &first.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|first| &mut first.value)
    }

    pub fn length(&self) -> u32 {
        match &self.head {
            None => 0,
            Some(first) => 1 + first.length_after(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        }
    }
}

// ----- IntoIter -----
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    // Take ownership of self, give it to the iterator.
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// ----- Iter -----
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    // Borrow self, lend to the iterator
    pub fn iter(&self) -> Iter<T> {
        // lifetime elision, everything in fn signature is in 'a
        Iter {
            // Convert the head (an Option<Box<Node<T>>) to a Option<&Node<T>> for the Iter struct.
            next: self.head.as_ref().map::<&Node<T>, _>(|n| &n),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // Convert the head (an Option<Box<Node<T>>) to a Option<&Node<T>> for the Iter struct.
            self.next = node.next.as_ref().map::<&Node<T>, _>(|n| &n);
            &node.value
        })
    }
}

// ----- IterMut -----
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    // Borrow self, lend to the iterator
    pub fn iter_mut(&mut self) -> IterMut<T> {
        // lifetime elision, everything in fn signature is in 'a
        IterMut {
            // Convert the head (an Option<Box<Node<T>>) to a Option<&mut Node<T>> for the Iter struct.
            next: self.head.as_mut().map(|n| &mut **n),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            // Convert the head (an Option<Box<Node<T>>) to a Option<&mut Node<T>> for the Iter struct.
            self.next = node.next.as_mut().map(|n| &mut **n);
            &mut node.value
        })
    }
}

// ----- Tests -----
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn list_new() {
        let mut list: List<i32> = List::new();

        // New lists should be empty
        assert_eq!(list.pop(), None);

        // New lists should have length 0
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn list_push_pop() {
        let mut list: List<i32> = List::new();

        // Add integers 1 to 10
        for n in 1..=10 {
            list.push(n);
            assert_eq!(list.length(), n as u32)
        }

        // Check that they come off in the right order
        for n in (1..=10).rev() {
            assert_eq!(list.pop(), Some(n));
            assert_eq!(list.length(), n as u32 - 1)
        }
    }

    #[test]
    fn list_peek() {
        let mut list: List<i32> = List::new();
        list.push(42);

        assert_eq!(list.peek(), Some(&42));
    }

    #[test]
    fn list_peek_mut() {
        let mut list: List<i32> = List::new();
        list.push(42);

        let value: &mut i32 = list.peek_mut().unwrap();
        *value = -42;

        assert_eq!(list.pop(), Some(-42));
    }

    #[test]
    fn list_into_iter() {
        let mut list: List<i32> = List::new();
        for x in 1..=10 {
            list.push(x)
        }

        let mut list_iter = list.into_iter();
        for x in (1..=10).rev() {
            assert_eq!(list_iter.next(), Some(x));
        }
    }

    #[test]
    fn list_iter() {
        let mut list = List::<i32>::new();
        for x in 1..=10 {
            list.push(x);
        }

        let mut x: i32 = 10;
        for el in list.iter() {
            assert_eq!(el, &x);
            x -= 1;
        }
        assert_eq!(x, 0)
    }

    #[test]
    fn list_iter_mut() {
        let mut list = List::<i32>::new();
        for x in 1..=10 {
            list.push(x);
        }

        let mut x: i32 = 10;
        for el in list.iter_mut() {
            assert_eq!(el, &x);
            *el = 0;
            x -= 1;
        }
        assert_eq!(x, 0);

        for el in list.iter() {
            assert_eq!(el, &0);
        }
    }
}
