use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret: u32 = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret}");

    println!("Guess the number from 1 to 100!");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please give me a number!");

        match guess.cmp(&secret) {
            Ordering::Less => println!("{guess}: Too small!"),
            Ordering::Equal => {
                println!("{guess} is correct! You win!");
                break;
            }
            Ordering::Greater => println!("{guess}: Too big!"),
        }
    }
}
