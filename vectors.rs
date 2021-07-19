//Resizable arrays
use std::mem;
pub fn run() {
let mut numbers: Vec<i32> = vec![1,2,22,3,4];
numbers[2] = 20;
//Add on to vector
numbers.push(5);
numbers.push(6);
numbers.pop();
println!("{:?}", numbers);
println!("Single value: {}", numbers[0]);
//GET VECTOR LENGTH
println!("Vector length: {}", numbers.len());
//stack allocated
println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
//Get slice
let slice: &[i32] = &numbers[0..2];
println!("Slice: {:?}", slice);
//Loop through vector values
for x in numbers.iter() {
    println!("Numbers: {}", x);
}
//Loop and mutate
for x in numbers.iter_mut() {
    *x *= 2;
}
println!("Numbers Vec: {:?}", numbers);
}