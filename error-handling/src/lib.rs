use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

// Result returns Type or Error
pub fn open_file() -> Result<File, Error>{
    File::open("scoops.txt")
        
} 


pub fn open_or_create() {
    File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

// return errors - Propagating the error

pub fn proper_file () -> Result<String, Error>{
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}