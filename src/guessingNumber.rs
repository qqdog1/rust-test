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
    rand::thread_rng().gen_range(1, 101)
}

fn read_user_input() -> u8 {
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        match guess.trim().parse(){
            Ok(num) => return num,
            Err(_) => continue
        }
    }
}

fn compare_num(input: u8, secret: u8) -> bool {
    match input.cmp(&secret) {
        Ordering::Less => {
            println!("Too small.");
            false
        },
        Ordering::Greater => {
            println!("Too big.");
            false
        },
        Ordering::Equal => {
            println!("Get it.");
            true
        }
    }
}