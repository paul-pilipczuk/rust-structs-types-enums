
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone_number: String,
}

fn main() {
    println!("{:?}", Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: "23johnny@gmail.com".to_string(),
        phone_number: "123-456-7890".to_string(),
    });
}