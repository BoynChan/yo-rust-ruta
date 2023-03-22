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

// use stack to check parentheses
fn par_checker(par: &str) -> bool {
    let len = par.len();
    let mut s = Stack::new();
    let char_list: Vec<char> = par.chars().into_iter().collect();

    for i in 0..len {
        let c = char_list[i];
        if c == '(' {
            s.push(c);
        } else if c == ')' {
            if s.is_empty() {
                return false;
            }
            s.pop();
        }
    }
    s.is_empty()
}

fn por_match(open: char, close: char) -> bool {
    let o = "([{";
    let c = ")]}";
    o.find(open) == c.find(close)
}

fn par_checker2(par: &str) -> bool {
    let len = par.len();
    let mut s = Stack::new();
    let char_list: Vec<char> = par.chars().into_iter().collect();

    for i in 0..len {
        let c = char_list[i];
        if c == '(' || c == '[' || c == '{' {
            s.push(c);
        } else if c == ')' || c == '}' || c == ']' {
            if s.is_empty() {
                return false;
            }
            if !por_match(*s.peek().unwrap(), c) {
                return false;
            }
            s.pop();
        }
    }
    s.is_empty()
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
#[test]
fn test_par_checker() {
    assert_eq!(par_checker("()"), true);
    assert_eq!(par_checker("(())"), true);
    assert_eq!(par_checker("(()"), false);
    assert_eq!(par_checker("())"), false);
}

#[test]
fn test_par_checker2() {
    assert_eq!(par_checker2("()"), true);
    assert_eq!(par_checker2("(())"), true);
    assert_eq!(par_checker2("(()"), false);
    assert_eq!(par_checker2("())"), false);
    assert_eq!(par_checker2("[[{(())}]]"), true);
    assert_eq!(par_checker2("[(){}]"), true);
    assert_eq!(par_checker2("[[(])]"), false);
    assert_eq!(par_checker2("{}{}{}"), true);
}
