fn main(){
    println!("{}","Good night");
    let a = 20;
    let b = 30;
    let c = multiply(a,b);
    println!("{}",c);
    let emoji:char = '😁';
    println!("");
    let lol = true;
    println!("{}",lol);
    println!();
    println!("{}",emoji);
}
fn multiply(a:i32,b:i32)-> i32{
    return a*b;
}