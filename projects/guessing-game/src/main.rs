use std::io;

fn main() {
    println!("guess the number!");

    println!("please enter the guess!");

    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(_) => (),
        Err(error) => {
            println!("Failed to read line: {error}");
        }
    }

    println!("you guessed: {}", guess);
}
