fn main(){
    let reg_num = "21BSDC129";
    let course = match reg_num{
        "21BSDC111" => "Basit",
        "21BSDC107" => "Aagnik",
        "21BSDC129" => "Abhishek",
        _ => "unknown"
    };
    println!("The student is {}",course);
}