//loop_code

/*

implementing 3 looping method in rust

*/

use std::{io::{self, Read}, iter};

fn exponent_loop(power : i64 , base : f64) -> f64{
    
    if power == 0{

        return 1.0 ;

    }

    let mut iteration : i64 = 0;

    let mut result : f64 = 1.0;

    let total  = loop{
           
           if iteration == power{

               break result;

           }else{

              iteration += 1;

              result *= base;
           }


    };

    return total;
}

fn exponent_while(power : i64 , base : f64) -> f64{

    if power == 0{

        return 1.0;
    }

    let mut iteration : i64 = 1;

    let mut result : f64 = 1.0;

    while iteration <= power{

        result *= base;

        iteration += 1;
    }

    return result;

}

fn exponent_for(power : i64 , base : f64) -> f64{

    if power == 0{

        return 1.0;
    }

    let mut result : f64 = 1.0;

    for i in 1..power + 1{

        result *=  base;
    }

    return result;
}

fn main(){

    let mut x = String::new();

    let mut y = String::new();

    println!("enter base: ");

    io::stdin().read_line(&mut x).expect("unable to read line");

    println!("enter power: ");

    io::stdin().read_line(&mut y).expect("unable to read line");

    let power :i64 = y.trim().parse().expect("unable toturn to integer");

    let base : f64 = x.trim().parse().expect("unable to turn to float");

    println!("{} ^ {} using loop = {}", base  , power , exponent_loop(power, base));

    println!("{} ^ {} using while = {}", base  , power , exponent_while(power, base));

    println!("{} ^ {} using for = {}", base  , power , exponent_for(power, base));

}