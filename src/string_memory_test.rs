fn main() {
    let a = get_string();
    let b = a;
    println!("{}", b);
    // println!(b);
    //
    // a.clear();
    //
    // println!("{}", a);
    // println!("{}", b);
}

fn get_string() -> String {
    String::from("abc")
}