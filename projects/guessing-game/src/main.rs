use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // inclusive range

    println!("the secret number is {secret_number}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Couldn't read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    loop {
        println!("please enter the guess!");

        match guess.cmp(&secret_number) {
            Ordering::Equal => println!("Correct match"),
            Ordering::Greater => println!("Guess too big"),
            Ordering::Less => println!("Guess too small"),
        };
    }

    // overshadow previous "guess"
    // parse converts string -> another type (:u32 in this case)

    println!("you guessed: {}", guess);
}
