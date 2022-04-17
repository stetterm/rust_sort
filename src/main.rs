use rand::Rng;
use std::{time::Instant, vec};

use rust_sort::alg;

pub mod link;

const NUM_ELEMENTS: usize = 100_000;
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
    let mut heap_total: u128 = 0;

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

        let res_time = time_sort(alg::heap_sort, &mut data[..]);
        assert!(alg::is_sorted(&data));
        println!("Heap Sort took ~{} milliseconds to complete",
            res_time,
        );
        heap_total += res_time;
    }

    println!("\nSelection Sort avg:\t{}\n\
            Merge Sort avg:\t\t{}\n\
            Quick Sort avg:\t\t{}\n\
            Tim Sort avg:\t\t{}\n\
            Tree Sort avg:\t\t{}\n\
            Heap Sort avg: \t\t{}\n",
        sel_total/NUM_TRIALS,
        mer_total/NUM_TRIALS,
        quick_total/NUM_TRIALS,
        tim_total/NUM_TRIALS,
        tree_total/NUM_TRIALS,
        heap_total/NUM_TRIALS,
    );

    println!("Starting Radix Sort tests\n");

    let mut radix_data: Vec<usize> = vec![];
    let mut new_val: usize;

    let mut radix_total = 0;

    for _ in 0..NUM_TRIALS {
        for _ in 0..NUM_ELEMENTS {
            new_val = rand::thread_rng().gen_range(0..MAX_VALUE) as usize;
            radix_data.push(new_val);
        }

        let res_time = time_sort(alg::radix_sort, &mut radix_data[..]);
        assert!(alg::is_sorted(&radix_data));
        println!("Radix Sort took ~{} milliseconds to complete",
            res_time,
        );
        radix_total += res_time;

        radix_data = vec![];
    }

    println!("\nRadix Sort avg: {}\n", radix_total/NUM_TRIALS);
}

fn time_sort<T: FnMut(&mut [D]) -> (), D: PartialOrd + Ord + Copy>(mut sort_alg: T, data: &mut [D]) -> u128 {
    let start_time = Instant::now();
    sort_alg(data);
    let elapsed = start_time.elapsed();
    elapsed.as_millis()
}