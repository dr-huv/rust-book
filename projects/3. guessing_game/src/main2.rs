use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    let mut guess = String::new();
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100); //thread_rng is a particular rng, gen_range generates a number within (1,100) inclusive of the bounds 1 and 100
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading input");

    //now we will get an error in match unless we convert guess to a number from a string, we do this by 

    let guess: u32 = guess.trim().parse().expect("Please enter a number!") //here trim returns a strign with noi whitespace, parse converts that to int
    //here u32 is mentioned, so it converts it into u32, then same error handling if it gives us an error
    //secret number will become a u32 as well, since we will be calling comp on it and no default datatype was assigned
    //also we used shadowing, whcih allows us todo type conversions
    println!("You have guessed {guess}");

    //match is basically like switch case
    match guess.cmp(&secret_number){ //cmp is a method that can be called on any value that can be compared
        //cmp returns one of the 3 ordering variants when it is used, if guess<secret number. Less is returned
        //match is an expression that has "arms" each arm tells what to do for a particular pattern that is matched, here, we are trying to match it for one of
        //the 3 enum variants of Ordering resulted from cmp
        Ordering::Less => println!("Too small!"), //Ordering is another enum with 3 variants Less,Greater,Equal
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}