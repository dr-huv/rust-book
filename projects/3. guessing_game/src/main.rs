use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess!");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess:u32 = guess.trim().parse().expect("Please enter a valid number"); there is a better way to do this

        let guess:u32 = match guess.trim().parse() { //instead of using expect for the result type we are using match
            //We are telling it what to do in case of an Ok (normal exec) or Err
            Ok(num) => num, //this will just return num, and it will be assigned to guess
            Err(_) => continue, //this will casue the loop to skip the current iteration and move on to the next, i.e it will ask again
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Guess too low!"),
            Ordering::Greater => println!("Guess too high!"),
            Ordering::Equal => {
                println!("You win!");
                break; //breaks out of loop
            }
        }
    }

}