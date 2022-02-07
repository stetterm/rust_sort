use rust_sort::alg;

pub mod link;

fn main() {
    let mut test_data: Vec<i32> = vec![5, 3, 42, 6, 1, 77, 156, 1, -4];
    alg::tim_sort(&mut test_data, 5);
    dbg!(test_data);
}