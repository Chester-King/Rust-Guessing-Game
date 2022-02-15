// Importing Crates

use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Main Function

fn main() {

    // Using println macro
    println!("Guess the number!");

    // Using rand crate and binding it to secret_number variable
    // the type of secret_number is inferred by rust
    let secret_number = rand::thread_rng().gen_range(1..101);

    // loop until break
    loop {
        println!("Please input your guess.");

        // initialize guess with instance of String
        let mut guess = String::new();

        // use io library to get input
        // expect will cause the program to crash if something goes wrong
        // read_line takes a mutable string reference in which it will update the value taken as input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // variable shadowing - guess variable is shadowed by parsing it to u32
        // parse returns a Result type and Result is an enum that has the variants Ok or Err. 
        // We’re using a match expression here, as we did with the Ordering result of the cmp method.
        // https://doc.rust-lang.org/std/result/enum.Result.html
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Unfortunately this isn't a valid number");
                continue;
            },
        };

        // Printing the guess
        println!("You guessed: {}", guess);

        // using cmp to compare. We can only compare the same types using cmp
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
