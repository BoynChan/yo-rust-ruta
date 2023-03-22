#[derive(Debug)]
pub struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top = self.top - 1;
        return self.data.pop();
    }

    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn size(&self) -> usize {
        self.top
    }
}

#[test]
fn test_stack() {
    let mut s = Stack::new();
    s.push(1);
    s.push(3);
    assert_eq!(s.peek(), Some(&3));
    assert_eq!(s.pop().unwrap(), 3);
    assert_eq!(s.size(), 1);
    assert_eq!(s.pop().unwrap(), 1);
    assert_eq!(s.is_empty(), true);
}
