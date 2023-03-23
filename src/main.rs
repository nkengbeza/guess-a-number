mod utils;

use rand::Rng;
use std::cmp::Ordering;
use utils::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut counter = 0;
    let mode;

    println!("<<< GUESSING GAME >>>");
    println!("Game Level\n1. Easy\n2. Intermediate\n3. Difficult\n4. Legend");
    loop {
        println!("Select a level (press Q to exit):");
        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => {
                if buf.trim().eq_ignore_ascii_case("q") {
                    panic!("Exiting...");
                }
                let num = match buf.trim().parse::<isize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid choice");
                        continue;
                    }
                };
                if num < 1 || num > 4 {
                    println!("Invalid choice");
                    continue;
                }
                mode = GamerLevel::new(num);
                break;
            }
            Err(err) => {
                panic!("An error occurred reading from stdin {:?}", err);
            }
        };
    }

    let bounds = match mode {
        GamerLevel::Easy(val) => val,
        GamerLevel::Intermediate(val) => val,
        GamerLevel::Difficult(val) => val,
        GamerLevel::Legend(val) => val,
    };

    loop {
        counter += 1;
        println!("\nROUND {}.\nEnter a number (press Q to exit): ", counter);
        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => {
                if buf.trim().eq_ignore_ascii_case("q") {
                    println!("Bye!!!");
                    break;
                }
                let num = match buf.trim().parse::<isize>() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Enter a valid number.");
                        continue;
                    }
                };
                let rand_num: isize = rng.gen_range(bounds.lower..=bounds.higher);
                match num.cmp(&rand_num) {
                    Ordering::Less => println!("Your number is less! You entered: {}, secret number: {}", num, rand_num),
                    Ordering::Greater => println!("Your number is greater! You entered: {}, secret number: {}", num, rand_num),
                    Ordering::Equal => println!("You won! You entered: {}, secret number: {}", num, rand_num),
                }
            }
            Err(err) => panic!("An error occurred reading from the standard input. {:?}", err)
        }
    }
}

