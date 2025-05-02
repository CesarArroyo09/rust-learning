struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

struct Point(i32, i32, i32);

fn main() {
    let username = String::from("user123");
    let email = String::from("user123@mail.com");
    let uri = String::from("https://example.com/user123");
    let active = true;

    let user = User {
        username,
        email,
        uri,
        active,
    };
    let my_point = Point(1, 2, 3);

    // Print all the values from user
    println!("Username: {}", user.username);
    println!("Email: {}", user.email);
    println!("URI: {}", user.uri);
    println!("Active: {}", user.active);
    // Print all the values from my_point
    println!("Point: ({}, {}, {})", my_point.0, my_point.1, my_point.2);
}
