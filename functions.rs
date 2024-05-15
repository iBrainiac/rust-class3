fn main (){
 // sum -> takes in 2 parameters
let age :i32 = {
    123* 44
};
println!("hey age {}" ,age);

sum(number_1: 23, number_2: 632);
sum2(number_1: 93, number_2: 732);
sum3(number_1: 82, number_2: 932);

}
fn sum(number_1: i32, number_2: i32){
    let number_3: i32 = number_1+number_2;
    println!("1)sum of 1 and 2 {}", number_3); 
}

fn sum2(number_1: i32, number_2: i32){
    let number_3: i32 = number_1+number_2;
    println!("2)sum of 1 and 2 {}", number_3);
    return number_3; 
}


fn sum3(number_1: i32, number_2: i32)->i32{
    let number_3: i32 = number_1+number_2;
    println!("sum of 1 and 2 {}", number_3);
    return number_3; 
}


// fn sum3(number_1: i32, number_2: i32)->i32{
//     let number_3: i32 = number_1+number_2;
//     println!("sum of 1 and 2 {}", number_3);
//     number_3  
// }


