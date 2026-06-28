//function

/*

implementing simple factoriel function 

*/


fn factoriel(n : i64) -> i64{

    if n == 0{

        return 1;

    }else {

        return  n * factoriel(n - 1);
    }
}


use std::io;

fn main(){

    let mut x = String::new();

    println!("enter integer: ");

    io::stdin().read_line(&mut x).expect("unable to read line");

    let n : i64 = x.trim().parse().expect("unable to turn to int");

    println!("{}! = {}" , n , factoriel(n));


}