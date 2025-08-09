// Compound Data Types
// arrays, tuples, slices, and strigs (slice string)

fn main() {
    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);
    // let mix = [1, 2, "apple", true]
    // println!("Mixed Array: {:?}", mix);
    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("Fruits: {:?}", fruits);
    println!("First fruit: {}", fruits[0]);
    println!("Second fruit: {}", fruits[1]);
    println!("Third fruit: {}", fruits[2]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("Mixed Tuple: {:?}", my_mix_tuple);

    // Slices: [1, 2, 3, 4, 5]
    let numbers_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", numbers_slice);

    let animal_slices: &[&str] = &["dog", "cat", "bird"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices: &[&String] = &[
        &"1984".to_string(),
        &"Brave New World".to_string(),
        &"Fahrenheit 451".to_string(),
    ];
    println!("Book Slice: {:?}", book_slices);

    // Strings vs String Slices(&str)
    // Strings [growable, mutable, heap-allocated, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold  Says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold  Says: {}", stone_cold);

    // B- &str (String Slice)
    let slice_string: String = String::from("Hello, World!");
    let slice: &str = &slice_string;
    println!("Slice value: {}", slice);

    let slice_string: String = String::from("Hello, World!");
    let slice: &str = &slice_string[0..5]; // Slicing the first 5 characters
    println!("Slice value: {}", slice);
}
