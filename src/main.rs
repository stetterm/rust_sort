use rand::Rng;
use std::{time::Instant, vec};

use rust_sort::alg;

pub mod link;

const NUM_ELEMENTS: usize = 10_000;
const NUM_TRIALS: u128 = 10;
const MIN_VALUE: i64 = -100_000;
const MAX_VALUE: i64 = 100_000;

fn main() {
    let mut data = vec![];
    let mut data_cpy = vec![];
    let mut new_val: i64;

    let mut sel_total: u128 = 0;
    let mut mer_total: u128 = 0;
    let mut quick_total: u128 = 0;
    let mut tim_total: u128 = 0;
    let mut tree_total: u128 = 0;

    for _ in 0..NUM_TRIALS {
        for _ in 0..NUM_ELEMENTS {
            new_val = rand::thread_rng().gen_range(MIN_VALUE..MAX_VALUE);
            data.push(new_val);
            data_cpy.push(new_val);
        }

        // // Selection Sort
        // let res_time = time_sort(alg::selection_sort, &mut data[..]);
        // assert!(alg::is_sorted(&data));
        // println!("Selection Sort took ~{} milliseconds to complete",
        //     res_time,
        // );
        // sel_total += res_time;
        // data.clone_from_slice(&data_cpy[..]);

        // Merge Sort
        let res_time = time_sort(alg::merge_sort, &mut data[..]);
        assert!(alg::is_sorted(&data));
        println!("Merge Sort took ~{} milliseconds to complete",
            res_time,
        );
        mer_total += res_time;
        data.clone_from_slice(&data_cpy[..]);

        // Quick Sort
        let res_time = time_sort(alg::quick_sort, &mut data[..]);
        assert!(alg::is_sorted(&data));
        println!("Quick Sort took ~{} milliseconds to complete",
            res_time,
        );
        quick_total += res_time;
        data.clone_from_slice(&data_cpy[..]);

        // Tim Sort
        let res_time = time_sort(alg::tim_sort, &mut data[..]);
        assert!(alg::is_sorted(&data));
        println!("Tim Sort took ~{} milliseconds to complete",
            res_time,
        );
        tim_total += res_time;
        data.clone_from_slice(&data_cpy[..]);

        // Tree Sort
        let res_time = time_sort(alg::tree_sort, &mut data[..]);
        assert!(alg::is_sorted(&data));
        println!("Tree Sort took ~{} milliseconds to complete",
            res_time,
        );
        tree_total += res_time;
        data.clone_from_slice(&data_cpy[..]);
    }

    println!("\nSelection Sort avg:\t{}\nMerge Sort avg:\t\t{}\nQuick Sort avg:\t\t{}\nTim Sort avg:\t\t{}\nTree Sort avg:\t\t{}\n",
        sel_total/NUM_TRIALS,
        mer_total/NUM_TRIALS,
        quick_total/NUM_TRIALS,
        tim_total/NUM_TRIALS,
        tree_total/NUM_TRIALS,
    );
}

fn time_sort<T: FnMut(&mut [D]) -> (), D: PartialOrd + Ord + Copy>(mut sort_alg: T, data: &mut [D]) -> u128 {
    let start_time = Instant::now();
    sort_alg(data);
    let elapsed = start_time.elapsed();
    assert!(alg::is_sorted(data));
    elapsed.as_millis()
}