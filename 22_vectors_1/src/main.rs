fn ownership() {
    let numbers = vec![1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("slice = {:?}", slice);
}

fn modifiable() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let slice = &mut numbers[1..4];
    slice[0] = 10; // This modifies the original. slice is only a reference

    // This should fail
    // You can't borrow inmutable after borrowing mutable
    // You can't borrow mutable twice in the same scope
    // let another_slice = &mut numbers[..];

    println!("slice    = {:?}", slice);
    println!("original = {:?}", numbers);
}

fn main() {
    ownership();
    println!();
    modifiable();
}
