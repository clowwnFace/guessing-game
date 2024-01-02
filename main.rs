use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("GUESS THE NUMBER!");
    println!("##THIS IS A SIMPLE GAME MADE TO TEST MY RUST SKILLS");
    println!("########################ENJOY THE GAME############");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    loop {
        println!("input your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read this line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed this: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("this is too small"),
            Ordering::Greater => println!("this is too big"),
            Ordering::Equal => {
                println!("you win!!!!");
                break;
            }
        }
    }
}
