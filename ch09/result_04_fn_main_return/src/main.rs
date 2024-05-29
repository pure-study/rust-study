use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello-test.txt")?;
    println!("{:?}", greeting_file);

    Ok(())
}
