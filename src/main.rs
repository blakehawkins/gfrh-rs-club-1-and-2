#[macro_use] extern crate failure;
extern crate rand;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate ferris_print;

use std::io;
use std::cmp::Ordering;

use failure::Error;
use rand::Rng;


#[derive(Fail, Debug)]
#[fail(display = "What!? Nine thousand?  It's over nine thouuuuuusaaaaaand!!!")]
pub struct EasterEggErr;

fn run() -> Result<(), Error> {
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
            bail!(EasterEggErr);
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less    => println!("Too low."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal   => {
                ferrisprint!("You win!");
                break;
            },
        }
    }

    Ok(())
}


fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        writeln!(stderr, "caused by: {}", e.as_fail()).expect(errmsg);

        writeln!(stderr, "backtrace: {:?}", e.backtrace()).expect(errmsg);

        ::std::process::exit(1);
    }
}
