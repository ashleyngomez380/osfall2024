const SECRET: i32 = 26;
fn check_guess(guess: i32, secret: i32) -> i32 {
    
    if guess == secret{
        return 0;
    }
    else if guess > secret{
        return 1;
    }
    else {
        return -1;
    }
}

fn main() {
    let mut guesses =0;
    let mut guess = 22 ;

    loop {

        let k = check_guess(guess,SECRET);
        guess -= k;
        println!("{}",guess);
        guesses += 1;
        if k == 0{
            println!("Correct");
            break;
        }
        else if k == 1{
            println!("Too high");
        }
        else {
            println!("Too Low");

        }
    }
    println!("This is the number of guesses it took {}",guesses)
}
