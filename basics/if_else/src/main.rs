//if_else

/*
demonstraing basic if else in rust
*/


use std::io::{self, Read};

fn main(){
    
    
    let mut x = String::new();
    
    let mut y = String::new();
    
    println!("enter first number: ");

    io::stdin().read_line(&mut x).expect("unable to get input");

    println!("enter second number: ");

    io::stdin().read_line(&mut y).expect("failed to get input");

    let a : f64 = x.trim().parse().expect("failed to turn to float");

    let b : f64 = y.trim().parse().expect("failed to turn to float");

    if a == b {

        println!("{} is equal to {}" , a , b);

    }else if a > b{

        println!("{} is larger than {}" , a, b );

    }else{

       println!("{} is smaller than {}" , a , b);
       
    }

}