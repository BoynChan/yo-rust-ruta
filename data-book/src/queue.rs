#[derive(Debug)]
struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            cap: 0,
            data: vec![],
        }
    }

    fn is_empty(&self) -> bool {
        return self.cap == 0;
    }

    fn enqueue(&mut self, item: T) {
        self.data.push(item);
        self.cap = self.cap + 1
    }

    fn dequeue(&mut self) -> T {
        if self.size() == 0 {
            panic!("queue empty")
        }
        let res = self.data.remove(0);
        self.cap = self.cap - 1;
        res
    }

    fn size(&self) -> usize {
        self.cap
    }
}

#[test]
fn test_queue() {
    let mut q = Queue::new();
    assert_eq!(q.size(), 0);
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    assert_eq!(q.size(), 3);

    assert_eq!(q.dequeue(), 1);
    assert_eq!(q.dequeue(), 2);
    assert_eq!(q.dequeue(), 3);

    assert_eq!(q.size(), 0);
}

fn joseph<'a>(name: &Vec<&'a str>, num: i32) -> &'a str {
    let mut q = Queue::new();
    for n in name {
        q.enqueue(n);
    }

    while q.size() > 1 {
        for _i in 0..num {
            let name = q.dequeue();
            q.enqueue(name);
        }
        q.dequeue();
    }
    q.dequeue()
}

#[test]
fn test_joseph() {
    let name = vec!["Brad", "Kew", "Jane", "Susan", "David", "Shieber"];
    assert_eq!(joseph(&name, 8), "David");
}
