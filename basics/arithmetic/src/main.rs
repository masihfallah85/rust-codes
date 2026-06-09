//arithmetic

/*basic arithmetic concept and input output in rust */

use std::io;

fn main(){
    
    //getting numbers
    let mut input = String::new();

    let mut input2 = String::new();

    println!("enter first number: ");

    io::stdin().read_line(&mut input).expect("failed to read input");
    
    println!("enter second number(non zero for divison: ");

    io::stdin().read_line(&mut input2).expect("failed to read input2");

    let a : f64 = input.trim().parse().expect("not a valid number");

    let b : f64 = input2.trim().parse().expect("not a valid number");

    //arithnetic 
    println!("addition, {} + {} = {}" , a , b , a + b);

    println!("subtraction, {} - {} = {}" , a , b , a - b);

    println!("multiplication , {} * {} = {}" , a , b , a * b);
   
    println!("division , {} / {} = {}" , a , b , a / b);

    println!("mod , {} mod {} = {}" , a , b , a % b);

}