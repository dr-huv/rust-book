use std::io; //this is how we perform io operations, we need to include this in every io related program

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //variables are immutable by default (i.e they are constants) add mut keyword to make them mutable, 
    //String::new() is how we initialise a string data type :: is used for assosciated functions

    io::stdin() //we use this for standard input from terminal
        .read_line(&mut guess) //read_line reads in a string and stores it in guess, read_line appends to a string, it does not overwrite
        //we are passing guess as a reference , so we use &, by using references we can save memory, also references are
        //immutable by default so we do &mut guess instead of &guess in the argument
        //.read_line() also returns a Result type which is an enum(canbhave multiple states) has 2 states Ok and Err
        //Ok means operation was succesful, Err means its failed and it also stores Err information
        .expect("Failed to read line."); //This is a method of the Result type returned by .read_line()

        //Note:- stdin() has a method read_line() this returns a Result type which has a method expect(), expect is not part of stdin()!!

    println!("You guessed: {guess}"); 
    // println!("You guessed: {}", guess); //We could have also done this
}
