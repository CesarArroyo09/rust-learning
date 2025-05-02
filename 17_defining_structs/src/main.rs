#[derive(Debug)]

struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}

fn main() {
    let cesar = Person {
        first_name: "Cesar".to_string(),
        last_name: "Arroyo".to_string(),
        age: Some(100), // For Options you need to use Some instead of the direct value
    };
    println!("{:?}", cesar);
    println!("The person's first name is: {}", cesar.first_name);
    println!("The person's age is: {:?}", cesar.age);
    // println!(
    //     "{:?}",
    //     Person {
    //         first_name: "John".to_string(),
    //         last_name: "Doe".to_string(),
    //         age: 25,
    //     }
    // );
}
