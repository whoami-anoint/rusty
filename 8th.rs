fn main() {
    let _x: i32; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    // assert_eq!(x, 5);
    println!("Success!");
}


// By prefixing the variable name with an underscore, you're essentially saying, "Yes, I know I'm not using this variable, and it's intentional. Please don't warn me about it."
