use std::io;

fn main() {
    
    println!("Guess the number!");
    println!("What's your guess? : ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("REEEEE Failed to read line");

    println!("You guessed {}", guess);
}
