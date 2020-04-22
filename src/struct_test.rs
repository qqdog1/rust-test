fn main() {
    let person = Person {name: String::from("Hi"), age: 8};
    println!("{:?}", &person);
    println!("{:#?}", &person);
}

#[derive(Debug)]
struct Person {
    name : String,
    age : u8,
}