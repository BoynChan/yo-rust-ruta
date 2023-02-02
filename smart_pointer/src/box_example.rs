pub mod boyn_box {
    use std::ops::Deref;
    enum List {
        Cons(i32, Box<List>),
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

    fn hello_string(s: &String) {
        println!("Hello {}", s);
    }

    use List::{Cons, Nil};

    trait Draw {
        fn draw(&self);
    }

    struct Button {
        id: u32,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Draw button {}", self.id);
        }
    }

    struct Screen {
        id: u32,
    }
    impl Draw for Screen {
        fn draw(&self) {
            println!("Draw screen {}", self.id);
        }
    }

    pub fn box_fn() {
        let b = Box::new(5);
        println!("Box b = {}", b);

        let cons_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);

        let deref_string = MyBox::new(String::from("Boyn"));
        hello_string(&deref_string);

        let arr = [0; 100];
        let arr1 = arr;
        println!("Len arr:{}", arr.len());
        println!("Len arr1:{}", arr1.len());

        let arr = Box::new([0; 100]);
        let arr1 = arr;
        // println!("Len arr:{}", arr.len()); // 这一行编译不通过
        println!("Len arr1:{}", arr1.len());

        let drawer: Vec<Box<dyn Draw>> =
            vec![Box::new(Button { id: 1 }), Box::new(Screen { id: 2 })];
        for e in drawer {
            e.draw();
        }
    }
}
