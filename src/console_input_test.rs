use std::io;

fn main() {
    let mut input = String::new();
    loop {
        println!("input anything:");

        io::stdin().read_line(&mut input).expect("Failed to read line.");

        println!("{}", input);
        // the result string will append on the address if input string let outside of while
    }
}