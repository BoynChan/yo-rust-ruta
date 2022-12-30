struct User {
    activate: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) ->u32 {
        self.width * self.height
    }

    fn can_hold(&self,other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let u1 = User{
        email: String::from("@gmail.com"),
        username: String::from("User"),
        activate: true,
        sign_in_count: 1,
    };
    let u2 = build_user(String::from("email"), String::from("username"));
    // we create a new u3 variable with u2 values and move u2 to u3 at the same time, 
    // we cannot use u2 anymore after construct
    // because of we move String in user which is a non Copy trait.
    let u3 = User {
        activate: false,
        ..u2
    };
    println!("Hello, world!");
    // println!("User:{}",u1);
    let ret = Rectangle{width:dbg!(10 + 5),height:10};
    dbg!(&ret);
    println!("Rectangle:{:#?}",ret);
    println!("Area:{}",cal_area(&ret));

    println!("Area from method:{}",ret.area());
    let ret2 = Rectangle{width:5,height:5};
    let ret3 = Rectangle{width:50,height:50};
    println!("can hold {}",ret.can_hold(&ret2));
    println!("can hold {}",ret.can_hold(&ret3));

    let ret4 = Rectangle::square(1);
    dbg!(&ret4);
}

fn cal_area(r: &Rectangle) ->u32 {
    r.height * r.width
}

fn build_user(email: String,username:String) -> User {
    User {
        email,
        username,
        activate:true,
        sign_in_count:1
    }
}
