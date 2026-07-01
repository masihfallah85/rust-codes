//array

/*
simple array and its methods
*/

fn main(){

    let mut arr : [i32 ; 5] = [5, 8 , 10 , 9 , 2];

    arr.sort();

    println!("length of array = {}" , arr.len());

    println!("is array empty? {}"  , arr.is_empty());

    println!("what is it's length? {}" , arr.len());

    println!("what is its first element? {:?}" , arr.first());

    println!("what is its last element? {:?}" , arr.last());

    println!("does it have 9? {}" , arr.contains(&9));

    println!("what is at index 2? {:?}" , arr.get(2));

    for (index , item) in arr.iter().enumerate(){

        println!("at index {} we have {}" , index , item);
    } 

    let sqr_arr = arr.map(|x| x * x);

    for (index , item) in sqr_arr.iter().enumerate(){

        println!("at index {} we have {}" , index , item);
    } 

    arr.reverse();

    for (index , item) in arr.iter().enumerate(){

        println!("at index {} we have {}" , index , item);
    } 

    arr.fill(0);

    for (index , item) in arr.iter().enumerate(){

        println!("at index {} we have {}" , index , item);
    } 
}