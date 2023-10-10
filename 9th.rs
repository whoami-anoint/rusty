fn main(){
    println!("{}","Hello");
    let a = 12;
    let b = 20;
    let c = sum(a,b);
    println!("{}",c);
}

fn sum(a:i32,b:i32)-> i32{
    a + b
}