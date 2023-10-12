fn main(){
    let variable = [1,2,3];
    println!("{:?}",variable);

    let variable1 = [4,5,6];
    println!("{}",variable1[2]);

    let mut variable1 = [4,5,6];
    variable1[2]=5;
    println!("{:?}",variable1);


    // 2D Array 
    let array = [[1,2,3],[4,5,6],[7,8,9]];
    println!("{:?}",array[0][1]);

}