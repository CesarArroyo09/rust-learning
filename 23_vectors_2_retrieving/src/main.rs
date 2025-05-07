fn get_item(index: usize) {
    let vec = vec![1, 2, 3, 4, 5];

    let value = vec.get(index).unwrap();

    println!("The value at index {} is {:?}", index, value);
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(2);

    // empty vector to force panic
    // let vec: Vec<i32> = vec![];

    // Retrieve value at a specific index
    let third_value: i32 = vec[2];
    println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    let last_value: &i32 = vec.last().unwrap();
    println!("The last value in the vector is: {}", last_value);

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }
}
