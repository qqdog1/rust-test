fn main() {
    let numbers:(i8, u32, f64) = (-5, 1234567890, 3.123);
    let (mmm, long, pi) = numbers;

    println!("{}", mmm);
    println!("{}", long);
    println!("{}", pi);
    println!("{}", numbers.0);
    println!("{}", numbers.2);
}