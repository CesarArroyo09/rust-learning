fn loop_and_panic(numbers: Vec<i32>) {
    for num in numbers {
        if num < 0 {
            panic!("Negative number found!");
        }
        println!("Number: {}", num);
    }
}

fn main() {
    // panic!("total and complete panic! please leave!");
    loop_and_panic(vec![1, 2, 3, 4, -5]);
}
