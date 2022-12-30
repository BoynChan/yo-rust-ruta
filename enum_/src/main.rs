#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}


#[derive(Debug)]
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

impl Coin {
    fn value(&self) ->u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel=> 5,
            Coin::Dime=> 10,
            Coin::Quarter(state)=> {
                println!("State fo quarter:{}",state);
                25
            },
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    dbg!(four);
    let home = IpAddr::V4(127, 0, 0, 1);
    let remote = IpAddr::V6(String::from("ipv6 addr"));
    dbg!(home);
    dbg!(remote);
    println!("Hello, world!");
    
    // some about Option
    let o1 = Some(1);
    let o2:Option<i32> = None;

    println!("Value in coin:{}",value_in_cents(Coin::Penny));
    println!("Value in coin:{}",Coin::Penny.value());
    println!("Value in coin:{}",Coin::Quarter(String::from("New York")).value());

    // a match syntactic sugar
    let o3 = Some(3);
    if let Some(max) = o3 {
        println!("if let has matched some type in value: {}",max);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel=> 5,
        Coin::Dime=> 10,
        Coin::Quarter(_)=> 25,
    }
}

fn plus_one_option(x: Option<i32>) ->Option<i32> {
    match x {
        None=>None,
        Some(i)=>Some(i+1),
    }
}
