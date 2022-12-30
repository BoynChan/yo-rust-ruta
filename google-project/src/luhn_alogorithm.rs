// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let mut r: Vec<u32> = cc_number
        .replace(" ", "")
        .chars()
        .map(|c| c.to_digit(10))
        .filter(|c| match c {
            Some(_) => return true,
            None => return false,
        })
        .map(|c| {
            if let Some(v) = c {
                return v;
            }
            return 0;
        })
        .collect();
    if r.len() < 2 {
        return false;
    }
    let mut index = r.len() - 1;
    let mut cnt = 0;
    let mut count = 0;
    dbg!(&r);
    loop {
        cnt += 1;
        let x = r[index];
        if cnt % 2 == 0 {
            let mut y = x * 2;
            if y > 10 {
                let ten_digit = y / 10;
                let single_digit = y % 10;
                y = ten_digit + single_digit;
            }
            r[index] = y
        }
        count += r[index];
        if index == 0 {
            break;
        }
        index -= 1;
    }
    dbg!(&r);
    dbg!(&count);
    count % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
