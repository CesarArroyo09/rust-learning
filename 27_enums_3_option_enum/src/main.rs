fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None // Options are enums with variantes `None` or Some(T)
    //  `T` being a generic data type.
    } else {
        Some(x / y)
    }
}

fn main() {
    let a = 10;
    let b = 2;
    // let b = 0;

    let result = divide(a, b);
    println!("{:?}", result.unwrap()); // This panics if result is None

    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Error: division by zero"),
    }
}
