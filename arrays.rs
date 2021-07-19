//Arrays - fixed list where elements are the same data types
use std::mem;
pub fn run() {
let mut numbers: [i32; 5] = [1,2,3,4,5];
numbers[2] = 20;
println!("{:?}", numbers);
println!("Single value: {}", numbers[0]);
//GET ARRAY LENGTH
println!("Array length: {}", numbers.len());
//stack allocated
println!("Array occupies {} bytes", mem::size_of_val(&numbers));
//Get slice
let slice: &[i32] = &numbers[0..2];
println!("Slice: {:?}", slice);
}