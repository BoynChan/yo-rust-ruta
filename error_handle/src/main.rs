use std::io::{self, Read};
use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt");
    let _ = match f {
        Ok(f) => f,
        Err(e) => {
            let k = e.kind();
            if k == ErrorKind::NotFound {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file:{:?}", e),
                }
            } else {
                panic!("Problem open file:{:?}", e);
            }
        }
    };
    println!("Hello, world!");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_in_shore() -> Result<String, io::Error> {
    let mut s = String::new();
    // 可以自动打包Result或者Option返回值，需要注意，不能混用。
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
