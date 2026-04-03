pub mod problems;

fn main() {
    let result = problems::p0338_counting_bits::Solution::count_bits(5);

    println!("{:?}", result);
}
