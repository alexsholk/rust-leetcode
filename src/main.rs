pub mod problems;

fn main() {
    let result = problems::p0013_roman_to_integer::Solution::roman_to_int(String::from("MCMXCIV"));
    println!("{:?}", result);
}
