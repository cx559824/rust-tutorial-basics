// Ownership, Borrowing and References

// Ownership
// ---------
// C, C++ -> Memory Management Control Issue
// Garbage Collector solved this issue, but created a new issue -> Slow Performance:
// [stopping/Resuming the program]
// OWNERSHIP introduced by Rust to solve memory safety issues
// and high performance at the same time
// What is Ownership?
// Every value has a single owner [every variable has one value, and it is its sole owner].

// Ownership Rules
// 1. Each value in Rust has a variable that's called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// Example, Rule 1
// fn main() {
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1);
//     println!("Length of '{}' is {}.", s1, len)
// }
//
// fn calculate_length(s: &str) -> usize {
//     s.len()
// }

// Example, Rule 2
// fn main() {
//     let s1 = String::from("RUST");
//     let s2 = s1;
//
//     // println!("{}", s1); // This will not work since there can only be one owner
//     println!("{}", s2)
// }

//Example, Rule 3
fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);
}
// s1 goes out of scope and its value will be droppped

// fn print_lost(s: &str) {
//     println!("{}", &s1);
// }

fn calculate_length(s: &str) -> usize {
    s.len()
}
