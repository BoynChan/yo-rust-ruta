use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(elem: T) -> Node<T> {
        Node { elem, next: None }
    }
}

#[derive(Debug)]
struct LVec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Copy + Debug + std::cmp::PartialEq> LVec<T> {
    fn new() -> Self {
        LVec {
            size: 0,
            head: None,
        }
    }

    fn push(&mut self, item: T) {
        let node = Node::new(item);
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            let mut h = self.head.as_mut().unwrap();
            while h.next.is_some() {
                h = h.next.as_mut().unwrap();
            }
            h.next = Some(Box::new(node));
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let mut h = self.head.as_mut().unwrap();
        while h.next.is_some() && h.next.as_ref().unwrap().next.is_some() {
            h = h.next.as_mut().unwrap();
        }
        self.size -= 1;
        h.next.take().map(|link| link.elem)
    }

    fn insert(&mut self, pos: usize, item: T) {
        if pos > self.size {
            panic!("Index out of bounds");
        }
        let mut h = self.head.as_mut().unwrap();
        for _ in 0..pos - 1 {
            h = h.next.as_mut().unwrap();
        }
        self.size += 1;
        let mut node = Node::new(item);
        node.next = h.next.take();
        h.next = Some(Box::new(node));
    }

    fn find(&self, item: T) -> bool {
        if self.is_empty() {
            return false;
        }
        let mut h = self.head.as_ref().unwrap();
        while h.next.is_some() {
            if h.elem == item {
                return true;
            }
            h = h.next.as_ref().unwrap();
        }
        return false;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index > self.size {
            return None;
        }
        let mut h = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            h = h.next.as_mut().unwrap();
        }
        let mut n = h.next.take().unwrap();
        h.next = n.next.take();
        self.size -= 1;
        Some(n.elem)
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[test]
fn test_lvec() {
    let mut l = LVec::new();
    assert_eq!(l.is_empty(), true);
    l.push(1);
    l.push(2);
    l.push(3);
    l.push(4);
    assert_eq!(l.size(), 4);
    assert_eq!(l.is_empty(), false);
    assert_eq!(l.find(1), true);
    assert_eq!(l.find(5), false);
    assert_eq!(l.pop().unwrap(), 4);
    l.insert(3, 5);
    assert_eq!(l.pop().unwrap(), 5);
    l.remove(2);
    assert_eq!(l.pop().unwrap(), 2);
}
