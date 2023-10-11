fn main(){
    let reg_num = "21BSDC129";
    let student = match reg_num{
        "20BSDC108"=>"Aagnik",
        "21BSDC111" => "Basit",
        "21BSDC129" => "Abishek",
        _ => "unknown"
    };
        println!("He is our student: {}",student);
}