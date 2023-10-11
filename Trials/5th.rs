fn main(){
    // let one = 1;
    // let two: i8 = 2;
    // let three = 3i8;
    // let four = 4_i8;
    // println!("{}",one);
    // println!("{}",two);
    // println!("{}",three);
    // println!("{}",four);

    let five = 1i32;
    println!("{} -byte",std::mem::size_of_val(&five));
    let six = 2i16;
    println!("{} -byte",std::mem::size_of_val(&six));

    let seven = five as i8;
    println!("{} -byte",std::mem::size_of_val(&seven));
}

