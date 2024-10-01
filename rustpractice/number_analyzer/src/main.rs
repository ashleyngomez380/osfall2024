// Creating the array of size 10
const ARRAY : [i32;10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Implementing the function

fn is_even(n: i32) -> bool{

    // return true if even
    if n %2 == 0{
        return true;
    }
    // false otherwise
    else {
        return false;
        }
    


}

fn main(){
    // This is the for loop
    for i in 0 .. ARRAY.len(){
       

        if ARRAY[i]%15 == 0 {
            println!("FizzBuzz");
        }
        else if ARRAY[i]%5 ==0 {
            println!("Buzz");
        }
        else if ARRAY[i] %3 ==0  {
            println!("Fizz");
        }
        else {
            println!("{}",is_even(ARRAY[i]));
        }
    }
    // This is the while loop
    let mut sum =0;
    let mut n = 0;

    while n < ARRAY.len() {
        sum += ARRAY[n] as i32;

    n += 1;
    }
    println!("Sum is {}",sum);

    // This is to print the largest number in the array

    //Initilize the largest
    let mut larg =ARRAY[0];

    for i in 1 .. ARRAY.len(){

        if ARRAY[i] > larg{
            larg = ARRAY[i];
        }
    } 
    println!("This is the largest num {}",larg);
}
