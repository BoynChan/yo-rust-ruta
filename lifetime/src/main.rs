#[derive(Debug)]
struct ImportanceSentence<'a> {
    first_sentence: &'a str,
}

// 在这里用到了生命周期约束，这个约束的意思是，我们要求print_and_return_part的返回值的生命周期，必须大于等于announcement的生命周期
// 'a : 'b声明了一个生命周期约束，这个约束的意思是，'a的生命周期必须大于等于'b的生命周期
impl<'a> ImportanceSentence<'a> {
    fn print_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Annoucement:{}", announcement);
        return self.first_sentence;
    }
}
fn main() {
    println!("Longer str:{}", longer("hello", "world1"));
    let sentence = String::from("call me maybe. hey hello");
    let fs = sentence.split(".").next().expect("can not find '.'");
    let i = ImportanceSentence { first_sentence: fs };
    println!("important sentence: {:?}", i);
}

// 几乎所有的生命周期讲解，都会以这个例子作为开始。
// 他的作用是返回两个字符串中更长的一个，然而，事情在Rust中并没有那么简单，
// 因为条件判断的作用，导致rust没有办法分清楚x和y的生命周期，如果返回了x，那么y的就到此结束，如果返回了y，那么x的就结束
// 因此，我们要在x和y的引用中，加入对于生命周期的说明，以编译器能够正确的判断出x和y的生命周期
// 需要注意的是，我们在指定生命周期的时候，实际上并不会改变变量的真实生命周期，只是告诉编译器，两个变量的生命周期条件约束
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    return y;
}
