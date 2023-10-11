use std :: io;

fn main(){
    println!("Enter you number: ");
    let mut num = String::new();

    io::stdin()
    .read_line(&mut num)
    .expect("Failed to guess");

    println!("Your number is: {num}")

}