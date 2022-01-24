use rust_sort::alg;

pub mod link;

fn main() {
    let mut test_data: Vec<i32> = vec![1, 3, 7, 16, 42, 12, 9];
    dbg!(&test_data);
    alg::insertion_sort(&mut test_data[..]);
    dbg!(test_data);
}