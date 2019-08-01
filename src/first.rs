use std::mem;

enum Link<T> {
    Empty,
    More(Box<Node<T>>)
}

struct Node<T> {
    value: T,
    next: Link<T>
}

pub struct List<T> {
    head: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{head: Link::Empty}
    }

    pub fn push(&mut self, el:T){
        let new = Node{
            value: el,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new))
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(first) => {
                self.head = first.next;
                Some(first.value)
            }
        }
    }

    pub fn length(&self) -> u32 {
        self.head.length_after()
    }
}

impl<T> Link<T> {
    pub fn length_after(&self) -> u32 {
        match &self {
            Link::Empty => 0,
            Link::More(node) => 1 + node.next.length_after()
        }
    }
}

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
}
