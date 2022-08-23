use std::io;
use std::cmp::Ordering;

use anyhow::{Context, Result};
use rand::Rng;


fn run() -> Result<()> {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        if guess > 9_000_u32 {
            None.context("What!? Nine thousand?  It's over nine thouuuuusaaaaand!!!")?;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less    => println!("Too low."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal   => {
                println!("You win!");
                break;
            },
        }
    }

    Ok(())
}


fn main() -> Result<()> {
    run()
}
