fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: &String) {
    println!("{}", s);
}

fn own_vec(vector: &Vec<i32>) -> Vec<i32> {
    let mut new_vector: Vec<i32> = Vec::new();
    for number in vector {
        new_vector.push(*number); // The * dereferences the borrowing so getting the value itself
    }
    new_vector.push(10);
    println!("{:?}", vector);
    new_vector
}

// Borrowing is the mechanism by which Rust allows you to lend ownership of a variable
// to a function or another part of your program without actually transferring ownership
// of the variable.
// When you borrow a variable, you are essentially saying
// "I want to use this variable for a little while, but I promise I won't modify it."
// The borrowing behavior is given by the & symbol. So when we say
// `my_str` as an argument of a function, what happens is that the function now owns
// the variable.
// On the other hand, when saying `&my_str` we are telling the compiler that the value
// of the `my_str` variable is going to be lent to the function that is being passed to.
fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello, world!");

    // this compiles no problem!
    own_integer(my_int);
    println!("{}", my_int);

    // When passing an argument to a function the value is now owned by the function
    // for strings
    own_string(&my_string); // take ownership of my_string
    println!("{:?}", my_string);

    let new_vector = own_vec(&my_vec);
    println!("Vector passed as an argument: {:?}", my_vec);
    println!("New vector: {:?}", new_vector);
}
