
fn variables() {
    // 创建一个可变的变量，可以对这个变量进行赋值，但是不能改变类型
    let mut x = 5;
    x = 6; // that's ok

    // 创建一个不可变的变量，不可以赋值，但是可以重新声明并改变类型
    let y = 5;
    // y = 8; // It will occurs an error.
    let y = "a"; // but it will works.

    let a = [1,2,3,4,5];

    
}