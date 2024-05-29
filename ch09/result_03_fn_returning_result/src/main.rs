use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    println!("================== Original:");
    println!("{:?}", read_username_from_file());
    println!("================== Optimized:");
    println!("{:?}", read_username_from_file_optimized());
    println!("================== Chained:");
    println!("{:?}", read_username_from_file_chained());
    println!("================== Shortest:");
    println!("{:?}", read_username_from_file_shortest());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello-test.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_optimized() -> Result<String, io::Error> {
    let mut username_file = File::open("hello-test.txt")?;

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_chained() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello-test.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello-shortest-test.txt")
}
