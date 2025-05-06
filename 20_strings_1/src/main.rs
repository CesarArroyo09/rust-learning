fn print_str(s: &str) {
    let mut new_string = s.to_string();
    new_string.push_str(" add extra spiceeee!!!");

    let another_string = format!("{} also with spcice", s);

    println!("{}", s);
    println!("{}", new_string);
    println!("{}", another_string);
}

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = "hello, world";
    print_str(s);

    // String is growable and mutable whereas str is not
    // String is owned by the code that creates it
    let salutation = String::from("hello");
    print_string(salutation);
}
