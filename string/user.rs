use std::io;
fn main(){
    let mut name = String::new();
    println!("Enter your Name");
    io::stdin().read_line(&mut name).expect("Error in name");
    println!("Good evening {}",name)
}