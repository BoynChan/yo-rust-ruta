#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person {
            name: name,
            age: age,
        }
    }

    fn display(self: &mut Person, age: u8) {
        // &self为&&mut Person
        // 但是通过Deref, 可以跟Person进行匹配
        let Person { name, age } = &self;
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
