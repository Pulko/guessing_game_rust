use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn interate_guess(secret_num: &u32, range_limit: &u32) {
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("The guess is not a number!");
                continue;
            }
        };

        if guess < 1 || guess > *range_limit {
            println!("The guess is out of range!");
            continue;
        }

        let format_str = format!("You guessed {} and ", &guess);

        match guess.cmp(&secret_num) {
            Ordering::Greater => println!("{} your guess is greater", format_str),
            Ordering::Less => println!("{} your guess is less", format_str),
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

        let range_limit: u32 = match range_limit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
    interate_guess(&secret_num, &range_limit);
}
