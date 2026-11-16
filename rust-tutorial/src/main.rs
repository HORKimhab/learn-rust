use std::io; 

fn main() {
    let mut name = String::new(); 
    // let mut name: String = String::new();
    // println!("Hello, world!");
    // println!("Hello, Vanda HappyBeef");
    println!("What is your name?"); 
    io::stdin().read_line(&mut name).expect("Faile to read");
    println!("Hello {}!", name.trim());
}
