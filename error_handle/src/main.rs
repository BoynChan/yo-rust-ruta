use std::fs;
use std::io::{self, Read};
use std::panic;
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
    let result = panic::catch_unwind(|| {
        println!("Hello");
    });
    assert!(result.is_ok());
    println!("Hello, world!");
}

enum ReadUsernameError {
    IoError(io::Error),
    EmptyUsername(String),
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> Self {
        ReadUsernameError::IoError(err)
    }
}

fn read_username(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
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
