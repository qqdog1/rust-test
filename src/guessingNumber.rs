use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        guessing();
    }
}

fn guessing() {
    let secret_num = get_random_number();
    println!("New game started, guess a number.");

    loop {
        let user_input = read_user_input();

        if compare_num(user_input, secret_num) {
            break;
        }
    }
}

fn get_random_number() -> u8 {
    return rand::thread_rng().gen_range(1, 101);
}

fn read_user_input() -> u8 {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line.");
    let guess: u8 = guess.trim().parse().expect("Please input a number.");
    return guess;
}

fn compare_num(input: u8, secret: u8) -> bool {
    match input.cmp(&secret) {
        Ordering::Less => {
            println!("Too small.");
            return false
        },
        Ordering::Greater => {
            println!("Too big.");
            return false
        },
        Ordering::Equal => {
            println!("Get it.");
            return true
        }
    }
}