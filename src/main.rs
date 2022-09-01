use std::{cmp::Ordering, io, ops::RangeInclusive, thread, time::Duration};

use rand::Rng;

fn main() -> Result<(), std::num::ParseIntError> {
    println!("Welcome to the tutorial application!");
    println!("1 - Number guesser");
    println!("2 - Countdown");
    println!("What do you want to launch?");

    loop {
        let mut launch = String::new();

        io::stdin()
            .read_line(&mut launch)
            .expect("Failed to read line!");

        let launch: u32 = match launch.trim().parse() {
            Ok(num) => num,
            Err(e) => return Err(e),
        };

        match launch {
            1 => {
                break number_guesser().expect("Error when executing!");
            }
            2 => {
                break countdown().expect("Error when executing!");
            }
            0 | 3..=u32::MAX => continue,
        }
    }

    println!("Au revoir!");

    Ok(())
}

// Number guesser game
fn number_guesser() -> Result<(), std::num::ParseIntError> {
    println!("Guess the number!");

    let secret_number = generate_number_in_range(1..=100);

    // println!("The secret number is: {secret_number}");

    'game_loop: loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => return Err(e),
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break 'game_loop;
            }
        }
    }

    Ok(())
}

// Countdown in seconds
fn countdown() -> Result<(), std::num::ParseIntError> {
    println!("What are we counting down to?");

    let mut title = String::new();

    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line!");

    println!("What is the lower bound?");

    let mut lower_bound = String::new();

    io::stdin()
        .read_line(&mut lower_bound)
        .expect("Failed to read line!");

    let lower_bound: u32 = match lower_bound.trim().parse() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };

    println!("What is the upper bound?");

    let mut upper_bound = String::new();

    io::stdin()
        .read_line(&mut upper_bound)
        .expect("Failed to read line!");

    let upper_bound: u32 = match upper_bound.trim().parse() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };

    for number in (lower_bound..=upper_bound).rev() {
        println!("T-{number} until {title}");
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

// Random number generator in a given u32-Range
fn generate_number_in_range(range: RangeInclusive<u32>) -> u32 {
    rand::thread_rng().gen_range(range)
}
