use std::io;

fn main() {
    let a  = [3; 5];
    // let a = [3, 3, 3, 3, 3];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

// The program resulted in a runtime error at the point of using an invalid value (value greater than max index) 
// in the indexing operation. The program exited with an error message and didn't execute the final println! 
// statement. When you attempt to access an element using indexing, Rust will check that the index you’ve specified 
// is less than the array length. If the index is greater than or equal to the length, Rust will panic. 
// This check has to happen at runtime, especially in this case, because the compiler can't possibly know 
// what value a user will enter when they run the code later.

