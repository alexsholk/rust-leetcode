pub mod problems;

fn main() {
    let mut vec = vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6];
    let result =
        problems::p0026_remove_duplicates_from_sorted_array::Solution::remove_duplicates(&mut vec);

    println!("{:?}\n{:?}", vec, result);
}
