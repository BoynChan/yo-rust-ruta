use std::collections::HashMap;

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

// it has some display issues if n > 10.
fn divide_by_n(mut det_number: i32, n: i32) -> String {
    let mut s = Stack::new();
    while det_number > n - 1 {
        let rem = det_number % n;
        if n == 0 {
            return "".to_string();
        }
        det_number = det_number / n;
        s.push(rem);
    }
    s.push(det_number);
    let mut r = String::new();
    while !s.is_empty() {
        r.push_str(s.pop().unwrap().to_string().as_str());
    }
    r
}

fn divide_by_two(mut det_number: i32) -> String {
    divide_by_n(det_number, 2)
}

fn infix_expression_to_postfix_expression(exp: &str) -> String {
    let mut op_stack = Stack::new();
    let mut postfix = "".to_string();
    let keys = vec!['+', '-', '*', '/', '(', ')'];
    let values = vec![1, 1, 2, 2, 0, 0];

    let priority: HashMap<_, _> = keys.into_iter().zip(values.into_iter()).collect();

    exp.chars().into_iter().for_each(|c| {
        if priority.get(&c).is_none() {
            postfix.push_str(c.to_string().as_str());
        } else if c == '(' {
            op_stack.push(c);
        } else if c == ')' {
            loop {
                let inner_op: char = op_stack.pop().unwrap();
                if inner_op == '(' {
                    break;
                }
                postfix.push_str(inner_op.to_string().as_str());
            }
        } else {
            if !op_stack.is_empty()
                && priority.get(op_stack.peek().unwrap()).unwrap() >= priority.get(&c).unwrap()
            {
                let op = op_stack.pop();
                postfix.push_str(op.unwrap().to_string().as_str());
            }
            op_stack.push(c);
        }
    });
    while !op_stack.is_empty() {
        postfix.push_str(op_stack.pop().unwrap().to_string().as_str());
    }

    postfix
}

fn calculate_expression(exp: &str) -> i32 {
    let s = infix_expression_to_postfix_expression(exp);
    let mut num_stack = Stack::new();
    let keys = vec!['A', 'B', 'C', 'D'];
    let values = vec![1, 2, 3, 4];
    let num_map: HashMap<_, _> = keys.into_iter().zip(values.into_iter()).collect();
    s.chars().into_iter().for_each(|c| {
        if let Some(nc) = num_map.get(&c) {
            num_stack.push(*nc);
        } else {
            let op1 = num_stack.pop().unwrap();
            let op2 = num_stack.pop().unwrap();
            if c == '+' {
                num_stack.push(op1 + op2);
            } else if c == '-' {
                num_stack.push(op1 - op2);
            } else if c == '*' {
                num_stack.push(op1 * op2);
            } else if c == '/' {
                num_stack.push(op1 / op2);
            }
        }
    });
    num_stack.pop().unwrap()
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

#[test]
fn test_divide_by_two() {
    assert_eq!(divide_by_two(6), "110");
    assert_eq!(divide_by_two(5), "101");
    assert_eq!(divide_by_two(1), "1");
    assert_eq!(divide_by_two(2), "10");
    assert_eq!(divide_by_two(0), "0");
}

#[test]
fn test_divide_by_n() {
    assert_eq!(divide_by_n(3, 3), "10");
    assert_eq!(divide_by_n(1, 3), "1");
    assert_eq!(divide_by_n(2, 3), "2");
}

#[test]
fn test_infix_expression_to_postfix_expression() {
    assert_eq!(infix_expression_to_postfix_expression("A+B*C+D"), "ABC*D++");
    assert_eq!(
        infix_expression_to_postfix_expression("(A+B)*(C+D)"),
        "AB+CD+*"
    );
    assert_eq!(
        infix_expression_to_postfix_expression("(A+B)+(C+D)"),
        "AB+CD++"
    )
}

#[test]
fn test_calculate_expression() {
    assert_eq!(calculate_expression("A+B*C+D"), 11);
}
