extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Hello, world!");
    guest_game();
}

fn guest_game() {
    println!("Guess the number");

    let mut guess = String::new();
    let rdn = rand::thread_rng().gen_range(1, 100);
    println!("Please input your guess , luck number is {}", rdn);
    loop {
        io::stdin().read_line(&mut guess)
            .expect("input a incorrect guess");
        let guess: i32 = guess.trim().parse()
            .expect("please input a number");
        let b = guess == rdn;
        println!("your guess is {}", b);
        if b {
            break;
        }
    }
}
