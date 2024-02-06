// Resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 22, 3, 4];
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);
    numbers.pop();

    println!("Vector: {:?}", numbers);

    // Single value
    println!("First value: {}", numbers[0]);

    // Vector length
    println!("Vector length: {}", numbers.len());

    // Stack allocated size
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get a slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in &numbers {
        println!("Number: {}", x);
    }

    // Loop and mutate
    for x in &mut numbers {
        *x *= 2;
    }
    println!("Doubled numbers: {:?}", numbers);
}
