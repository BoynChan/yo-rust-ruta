#[derive(Debug)]
struct Deque<T> {
    data: Vec<T>,
}

impl<T> Deque<T> {
    fn new() -> Self {
        Deque { data: vec![] }
    }

    fn is_empty(&self) -> bool {
        return self.data.is_empty();
    }

    fn add_rear(&mut self, item: T) {
        self.data.insert(0, item);
    }

    fn add_front(&mut self, item: T) {
        self.data.push(item);
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn remove_rear(&mut self) -> T {
        self.data.remove(0)
    }

    fn remove_front(&mut self) -> T {
        self.data.pop().unwrap()
    }
}

#[test]
fn test_deque() {
    let mut q = Deque::new();
    q.add_front(1);
    q.add_front(2);
    q.add_front(3);
    assert_eq!(q.size(), 3);
    q.add_rear(4);
    q.add_rear(5);
    q.add_rear(6);
    assert_eq!(q.size(), 6);
    assert_eq!(q.remove_front(), 3);
    assert_eq!(q.remove_rear(), 6);
    assert_eq!(q.remove_front(), 2);
    assert_eq!(q.remove_rear(), 5);
    assert_eq!(q.size(), 2);
    assert_eq!(q.remove_front(), 1);
    assert_eq!(q.remove_rear(), 4);
    assert_eq!(q.size(), 0);
}

fn pal_checker(pal: &str) -> bool {
    let mut d = Deque::new();
    for c in pal.chars() {
        d.add_rear(c);
    }
    loop {
        if d.size() == 0 || d.size() == 1 {
            return true;
        }
        if d.remove_front() != d.remove_rear() {
            return false;
        }
    }
    true
}

#[test]
fn test_pal_checker() {
    assert_eq!(pal_checker("aba"), true);
    assert_eq!(pal_checker("addd12b21ddda"), true);
    assert_eq!(pal_checker("abc"), false);
    assert_eq!(pal_checker("ab"), false);
}
