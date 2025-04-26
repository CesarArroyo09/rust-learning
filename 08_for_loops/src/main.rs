fn main() {
    println!("First example");
    for i in 1..=10 {
        println!("i = {}", i);
    }
    print!("\n");

    for i in (1..=5).rev() {
        println!("{}", i);
    }
    println!();

    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("{}", n);
    }
}
