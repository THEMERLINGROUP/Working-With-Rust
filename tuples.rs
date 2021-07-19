//Tuples group together values of different types
//Max 12 elements
pub fn run() {
    //Name, Medical Properties, Medical Rating
    //According to pfaf.org
let plant: (&str, &str, i8) = ("Viscum album", "Immune, cardiovascular, stress, and anixety boost", 3);
println!("{} is a {} and its medical rating accoring to pfaf.org is {} ", plant.0, plant.1, plant.2);
}