use std::io;
use std::io::{Write};

fn main() {
    print!("What's you name: ");
    io::stdout().flush().expect("stdout flush failed");

    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("fail to read name");
    println!("Hello, world! {}", name);
}
