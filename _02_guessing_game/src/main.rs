use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    let mut guess = String::new();
   
    io::stdin().read_line(&mut guess)
        .expect("REEEEE Failed to read line");
   
    let guess: u32 = guess.trim().parse()
        .expect("REEEEE Type a number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("Guess the number!");
    println!("What's your guess? : ");



    println!("You guessed {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
