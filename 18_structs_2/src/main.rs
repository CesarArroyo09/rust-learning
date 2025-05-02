#[derive(Debug)]

struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

// impl means implementation
impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    // Let's try the implementation
    let mut myself = User::new(
        "Cesar".to_string(),
        "cesar@mail.com".to_string(),
        "http://my/uri/127".to_string(),
    );

    println!("Here is myself\n{:?}", myself);

    myself.deactivate();
    println!("Now after deactivation: {:?}", myself);
}
