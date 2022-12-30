enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop box");
    }
}

use std::{ops::Deref, rc::Rc};

use crate::List::{Cons, Nil};
fn main() {
    let b = Box::new(5);
    println!("Box b = {}", b);

    let cons_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let deref_string = MyBox::new(String::from("Boyn"));
    hello_string(&deref_string);

    let list_a = Rc::new(RcList::Cons(
        1,
        Rc::new(RcList::Cons(
            2,
            Rc::new(RcList::Cons(3, Rc::new(RcList::Nil))),
        )),
    ));
    println!("count after creating a = {}", Rc::strong_count(&list_a));

    let list_b = RcList::Cons(10, Rc::clone(&list_a));
    println!("count after creating b = {}", Rc::strong_count(&list_a));
    {
        let list_c = RcList::Cons(15, Rc::clone(&list_a));
        println!("count after creating c = {}", Rc::strong_count(&list_a));
    }
    println!("count after leaving c = {}", Rc::strong_count(&list_a));
}

fn hello_string(s: &String) {
    println!("Hello {}", s);
}
