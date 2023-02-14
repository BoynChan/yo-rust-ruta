use std::{rc::Rc, sync::Arc, thread};

struct Owner {
    name: String,
}

struct Gadget {
    id: u32,
    owner: Rc<Owner>,
}

pub fn rc_fn() {
    let s = String::from("Hello World");
    let a = Box::new(s);
    // 下面这行会出错, 因为s的所有权在上面已经转移到了a, 不能再转移给b
    // let b = Box::new(s);

    let a = Rc::new(String::from("hello"));
    // 通过Rc,可以实现一个值有多个引用, 通过引用计数进行内存管理
    // 下面两种方法演示了怎么通过clone机制将变量a的值复制到不同Rc.
    let b = a.clone();
    let c = Rc::clone(&a);
    assert_eq!(3, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));

    let owner = Rc::new(Owner {
        name: String::from("John"),
    });
    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&owner),
    };

    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&owner),
    };
    drop(owner);
    // Rc的引用之间没有承接关系, 我们在clone了两个owner的rc后drop掉了owner本身
    // 可以看到, gadget的两个变量还是可以正常使用
    println!("Gadget1 owner: {}", gadget1.owner.name);
    println!("Gadget2 owner: {}", gadget2.owner.name);

    let s = Rc::new(String::from("value"));
    for _ in 0..10 {
        let s = Rc::clone(&s);
        // 以下这行代码没有办法被执行
        // 表面原因是rc没有实现send trait, 无法通过move语义转移所有权到别的县城中
        // 本质原因是rc没有通过并发原语保持线程之间的引用计数一致性
        // let handle = thread::spawn(move || {
        //     println!("Thread: {}", s);
        // });
    }

    let s = Arc::new(String::from("move around multi-thread"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        // 通过Arc,可以将Rc的引用计数机制扩展到多线程环境中
        let handle = thread::spawn(move || println!("Thread: {}", s));
    }
}
