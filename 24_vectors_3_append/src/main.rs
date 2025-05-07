fn main() {
    let mut v = vec![1, 2, 3];
    println!("Original value: {:?}", v);
    v.push(4);
    println!("First addition: {:?}", v);

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("Second addition: {:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    // this is because append is actually moving the values from other_numbers to v
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("Third addition: {:?}", v);

    // inserts item at a given index
    v.insert(0, 0);
    println!("Fourth addition: {:?}", v);
}
