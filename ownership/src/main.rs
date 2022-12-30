fn main() {
    let x = 3;
    basic_copy(x);
    println!("can still use x {}",x);

    let s = String::from("123");
    move_in(s);
    // println!("we can not use s again, {}",s);

    let mut s2 = String::from("123");
    s2 = move_in_and_return(s2);
    println!("we can use s2 again, {}",s2);
    let s3 = move_in_and_return(s2);
    println!("or use it new value, {}",s3);

    let s4 = String::from("1234");

    println!("len of s4 {}",len_with_reference(&s4));

    let s5 = String::from("awwd b e");
    println!("first word index:{}",first_word_use_iter(&s5));
    
    let slice_1 = first_word_use_slice(&s5);
    println!("first word: {}",slice_1);
}

fn first_word_use_slice(s:&String) -> String {
    let bs = s.as_bytes();

    for (i,&c) in bs.iter().enumerate() {
        if c == b' '{
            return String::from(&s[..i]);
        }
    } 
    return String::from(&s[..])
}

fn first_word_use_iter(s :&String) -> usize {
    let bs = s.as_bytes();

    for (i,&c) in bs.iter().enumerate() {
        if c == b' '{
            return i;
        }
    }
    s.len()
}

fn len_with_reference(s:&String) ->usize {
    s.len()
}

fn move_in_and_return(s:String) -> String{
    s
}

fn move_in(s:String) {
    println!("move in {}",s);
}

fn basic_copy(x:i32) {
    println!("basicCopy {}",x);
}
