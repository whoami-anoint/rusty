fn main() {
    let mut a = 20;
    println!("{}", a);
    a = 24;
    println!("{}", a);

    const AP: i8 = 28;
    println!("{}", AP); // for constant it should be defined either string or int

    const TEST: &str = "lol"; // Corrected constant string
    println!("{}", TEST);
}
