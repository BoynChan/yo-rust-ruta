pub mod company;
use company::Company;
use std::collections::HashMap;
fn main() {
    let mut v = vec![5, 7, 2, 6, 8, 2, 45, 1];
    if let Some(m) = medium_number(&mut v) {
        println!("medium number of v:{}", m);
    }

    println!("apple to pigLatin:{}", toPigLatin(&String::from("apple")));
    println!("first to pigLatin:{}", toPigLatin(&String::from("first")));

    let mut c = Company::new();
    c.add_employee(&String::from("Add Zhanpeng to Tech"));
    c.add_employee(&String::from("Add Zhanpeng to Office"));
    c.add_employee(&String::from("Add mfer to Office"));
    c.add_employee(&String::from("Add Azuki to Office"));
    dbg!(c.deparment_list(&String::from("Office")));
}

fn toPigLatin(s: &String) -> String {
    let vowel = ['a', 'e', 'i', 'o', 'u'];
    for v in vowel {
        if s.starts_with(v) {
            let s = format!("{}-hay", s);
            return s;
        }
    }
    let mut first: char = ' ';
    for c in s.chars() {
        first = c;
        break;
    }
    if let Some(new_s) = s.strip_prefix(first) {
        let s = format!("{}-{}ay", new_s, String::from(first));
        return s;
    }

    return String::from("");
}

fn medium_number(v: &mut Vec<i32>) -> Option<&i32> {
    sort(v);
    return v.get(v.len() / 2);
}

fn sort(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        let mut min_idx = i;
        for j in i..v.len() {
            if v[j] < v[min_idx] {
                min_idx = j;
            }
        }
        let temp = v[i];
        v[i] = v[min_idx];
        v[min_idx] = temp;
    }
}

fn hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("team1"), 1);
    scores.insert(String::from("team2"), 3);

    for (k, v) in &scores {
        println!("K:{}, V:{}", k, v);
    }

    let v1 = vec!["team1", "team2", "team3"];
    let v2 = vec![1, 2, 3];

    let mut h1: HashMap<_, _> = v1.into_iter().zip(v2.into_iter()).collect();
    dbg!(&h1);

    h1.entry("team1").or_insert(50);
}

fn string() {
    let data = "123";
    let s = data.to_string();
    let s = "123".to_string();

    let s1 = String::from("123");
    let s2 = String::from("456");
    let s3 = String::from("789");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Format string:{}", s);

    let s = "你好";
    for c in s.chars() {
        println!("c: {}", c);
    }

    for b in s.bytes() {
        println!("b: {}", b);
    }
}
fn vector() {
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    v1.push(1);

    println!("v1[0] {}", &v1[0]);
    print_index(&v2, 1);
    print_index(&v2, 3);

    let first = &v1[0];
    println!("v1[0] = {}", first);
    v1.push(6);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 10;
    }
    for i in &v {
        println!("{}", i);
    }
}

fn print_index(v: &Vec<i32>, index: usize) {
    if let Some(r) = v.get(index) {
        println!("v[{}] = {}", index, r);
    } else {
        println!("v[{}] out of index", index);
    }
}
