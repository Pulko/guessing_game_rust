use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn interate_guess(secret_num: &u32) {
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the guess");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed {} and..", &guess);

        match guess.cmp(&secret_num) {
            Ordering::Greater => println!("...your guess is greater"),
            Ordering::Less => println!("...your guess is less"),
            Ordering::Equal => {
                println!("It is right!");
                break;
            }
        }
    }
}

fn get_secret_number(limit: u32) -> u32 {
    thread_rng().gen_range(1..=limit)
}

fn iterate_range_limit() -> u32 {
    loop {
        println!("Please input the range limit to get a secret number from:");

        let mut range_limit = String::new();

        io::stdin()
            .read_line(&mut range_limit)
            .expect("Failed to read the range limit");

        let range_limit: u32 = range_limit.trim().parse().expect("Please type a number!");

        if range_limit > 1 {
            println!("The range is from 1 to {} inclusive", &range_limit);
            return range_limit;
        } else {
            println!("The range limit is too small!");
            continue;
        }
    }
}

fn main() {
    println!("Guess the number!");

    let range_limit = iterate_range_limit();
    let secret_num = get_secret_number(range_limit);
    // println!("The secret number is {secret_num}");

    interate_guess(&secret_num);
}
