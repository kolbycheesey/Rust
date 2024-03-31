use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // let apples = 5;     //immutable
    // let mut bananas = 5; //mutable  //complains at this time because it is never read before it is changed
    // println!("Apples {bananas}");

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess: ");

    let mut guess = String::new();
    // let mut guess = Double::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("The secret number is : {secret_number}");
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    // apples = 7; //error
    // bananas = 7;
    // println!("Apples {bananas}");
}
