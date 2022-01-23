pub mod tree;

pub mod alg {
    use rand::Rng;
    use std::cmp::Ordering;

    pub use crate::tree::tree;

    pub fn quick_sort<T: PartialOrd + Ord + Copy>(data: &mut [T]) {
        if data.len() <= 1 {
            return;
        }
        let mut right_count: usize = data.len() - 1;
        let mut left_count: usize = 0;
        let mut pivot_count: usize = 0;
        let pivot: T = *data.get(rand::thread_rng().gen_range(0..data.len())).expect("Out of bounds");
        {
            let mut data_copy: Vec<T> = vec![pivot; data.len()];
            data_copy.clone_from_slice(&data);
            for value in data_copy.iter() {
                match value.cmp(&pivot) {
                    Ordering::Greater => {
                        data[right_count] = *value;
                        right_count -= 1;
                    },
                    Ordering::Less => {
                        data[left_count] = *value;
                        left_count += 1;
                    },
                    Ordering::Equal => {
                        pivot_count += 1;
                    }
                }
            }
        }
        for i in 0..pivot_count {
            data[left_count + i] = pivot;
        }
        quick_sort(&mut data[..left_count]);
        quick_sort(&mut data[right_count+1..]);
    }

    pub fn merge_sort<T: PartialOrd + Ord + Copy>(data: &mut [T]) {
        if data.len() <= 1 {
            return
        }
        let midpoint: usize = data.len() / 2;
        merge_sort(&mut data[..midpoint]);
        merge_sort(&mut data[midpoint..]);
        let mut right_count: usize = data.len() / 2;
        let mut left_count: usize = 0;
        let mut data_copy: Vec<T> = vec![*data.get(0).expect("Out of bounds"); data.len()];
        let mut counter: usize = 0;
        data_copy.clone_from_slice(&data);
        while left_count < midpoint && right_count < data.len() {
            match data_copy[left_count].cmp(&data_copy[right_count]) {
                Ordering::Less => {
                    data[counter] = data_copy[left_count];
                    left_count += 1;
                },
                Ordering::Greater => {
                    data[counter] = data_copy[right_count];
                    right_count += 1;
                },
                Ordering::Equal => {
                    data[counter] = data_copy[left_count];
                    left_count += 1;
                }
            }
            counter += 1;
        }
        if left_count < midpoint {
            for i in left_count..midpoint {
                data[counter] = data_copy[i];
                counter += 1;
            }
        } else {
            for i in right_count..data.len() {
                data[counter] = data_copy[i];
                counter += 1;
            }
        }
    }

    pub fn radix_sort(data: &mut [usize], base: usize) {
        let mut copy: Vec<usize> = vec![0; data.len()];
        let mut count: Vec<usize> = vec![0; base];
        let largest_value: usize = largest(&data);
        for i in 0..dig_count(largest_value) {
            for value in data.iter() {
                let divided: usize = value / (base.pow(i));
                count[divided % base] += 1;
            }
            prefix_sum(&mut count);
            for j in 1..data.len()+1 {
                let index = data.len() - j;
                let remainder = (data[index] / (base.pow(i))) % base;
                count[remainder] -= 1;
                copy[count[remainder]] = data[index];
            }
            data.clone_from_slice(&copy);
            count = vec![0; data.len()];
        }
    }

    fn prefix_sum(data: &mut [usize]) {
        let mut total: usize = 0;
        for i in 0..data.len() {
            data[i] += total;
            total = data[i];
        }
    }

    fn dig_count(value: usize) -> u32 {
        let mut dig_count = 0;
        let mut value_copy = value;
        while value_copy != 0 {
            value_copy /= 10;
            dig_count += 1;
        }
        dig_count
    }

    fn largest(data: &[usize]) -> usize {
        let mut largest: usize = 0;
        for i in data {
            if *i > largest {
                largest = *i;
            }
        }
        largest
    }

    pub fn tim_sort<T: PartialOrd + Ord + Copy>(data: &mut [T], n: usize) {
        let length = data.len();
        for i in (0..length).step_by(n) {
            insertion_sort(&mut data[i..min(i+n, length)]);
        }

        let mut size = n;
        while size < length {
            for left in (0..length).step_by(2 * size) {
                let mid = left + size - 1;
                let right = min(left + 2 * size - 1, length - 1);
                if mid < right {
                    merge(data, left, mid, right);
                }
            }
            size *= 2;
        }
    }

    fn min<T: PartialOrd + Ord + Copy>(a: T, b: T) -> T {
        if let Ordering::Greater = a.cmp(&b) {
            b
        } else {
            a
        }
    }

    fn merge<T: PartialOrd + Ord + Copy>(data: &mut [T], l: usize, m: usize, r: usize) {
        let len1 = m - l + 1;
        let len2 = r - m;
        let mut left: Vec<T> = vec![*data.get(0).unwrap(); len1];
        left.clone_from_slice(&data[l..m+1]);
        let mut right: Vec<T> = vec![*data.get(0).unwrap(); len2];
        right.clone_from_slice(&data[m+1..r+1]);

        let mut i = 0;
        let mut j = 0;
        let mut k = l;

        while i < len1 && j < len2 {
            if left[i] <= right[j] {
                data[k] = left[i];
                i += 1;
            } else {
                data[k] = right[j];
                j += 1;
            }
            k += 1;
        }

        while i < len1 {
            data[k] = left[i];
            k += 1;
            i += 1;
        }

        while j < len2 {
            data[k] = right[j];
            k += 1;
            j += 1;
        }
    }

    pub fn insertion_sort<T: PartialOrd + Ord + Copy>(data: &mut [T]) {
        for i in 1..data.len() {
            let mut insert_index = i-1;
            while data[i] < data[insert_index] && insert_index > 0 {
                insert_index -= 1;
            }
            insert_index += 1;
            let temp = data[i];
            for j in 0..i-insert_index {
                data[i-j] = data[i-j-1];
            }
            data[insert_index] = temp;
        }
    }

    pub fn selection_sort<T: PartialOrd + Ord + Copy>(data: &mut [T]) {
        for i in 0..data.len() {
            let mut smallest = data[i];
            let mut smallest_index = i;
            for j in i+1..data.len() {
                if let Ordering::Less = data[j].cmp(&smallest) {
                    smallest = data[j];
                    smallest_index = j;
                }
            }
            let temp = data[i];
            data[i] = data[smallest_index];
            data[smallest_index] = temp;
        }
    }

    pub fn tree_sort<T: PartialOrd + Ord + Copy>(data: &mut [T]) {
        let mut bst = tree::new(data[0]);
        for i in 1..data.len() {
            bst.add_node(data[i]);
        }
        bst.get_inorder(&mut data[..]);
    }

    pub fn is_sorted<T: PartialOrd + Ord + Copy>(list: &[T]) -> bool {
        for i in 0..list.len()-1 {
            if let Ordering::Greater = list[i].cmp(&list[i+1]) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    // is_sorted tests

    #[test]
    pub fn sorted_list() {
        let test_data = vec![1, 2, 3];
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    #[test]
    pub fn non_sorted_list() {
        let test_data = vec![3, 2, 1];
        assert_ne!(true, alg::is_sorted(&test_data));
    }

    // quick_sort tests

    #[test]
    pub fn backwards_from_100() {
        let mut test_data: Vec<i32> = Vec::new();
        for i in 0..100 {
            test_data.push(100-i);
        }
        alg::quick_sort(&mut test_data[..]);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    #[test]
    pub fn random_100_elements() {
        let mut test_data: Vec<i32> = Vec::new();
        for _ in 0..100 {
            test_data.push(rand::thread_rng().gen_range(0..10000));
        }
        alg::quick_sort(&mut test_data);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    #[test]
    pub fn random_1_000_000_elements() {
        let mut test_data: Vec<i32> = Vec::new();
        for _ in 0..1_000_000 {
            test_data.push(rand::thread_rng().gen_range(0..10_000_000));
        }
        alg::quick_sort(&mut test_data);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    #[test]
    pub fn list_of_same_number() {
        let mut test_data: Vec<i32> = vec![42; 10_000_000];
        alg::quick_sort(&mut test_data);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    // merge_sort tests

    #[test]
    pub fn merge_backwards_from_100() {
        let mut test_data: Vec<i32> = Vec::new();
        for i in 0..100 {
            test_data.push(100-i);
        }
        alg::merge_sort(&mut test_data[..]);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    #[test]
    pub fn merge_random_100_elements() {
        let mut test_data: Vec<i32> = Vec::new();
        for _ in 0..100 {
            test_data.push(rand::thread_rng().gen_range(0..10000));
        }
        alg::merge_sort(&mut test_data);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    #[test]
    pub fn merge_random_1_000_000_elements() {
        let mut test_data: Vec<i32> = Vec::new();
        for _ in 0..1_000_000 {
            test_data.push(rand::thread_rng().gen_range(0..10_000_000));
        }
        alg::merge_sort(&mut test_data);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    // #[test]
    // pub fn merge_list_of_same_number() {
    //     let mut test_data: Vec<i32> = vec![42; 10_000_000];
    //     alg::merge_sort(&mut test_data);
    //     assert_eq!(true, alg::is_sorted(&test_data));
    // }

    // radix_sort tests

    #[test]
    pub fn radix_backwards_from_100() {
        let mut test_data: Vec<usize> = Vec::new();
        for i in 0..100 {
            test_data.push(100 - i);
        }
        alg::radix_sort(&mut test_data, 10);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    #[test]
    pub fn radix_random_100_elements() {
        let mut test_data: Vec<usize> = Vec::new();
        for _ in 0..100 {
            test_data.push(rand::thread_rng().gen_range(0..10000));
        }
        alg::radix_sort(&mut test_data, 10);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    #[test]
    pub fn radix_random_1_000_000_elements() {
        let mut test_data: Vec<usize> = Vec::new();
        for _ in 0..1_000_000 {
            test_data.push(rand::thread_rng().gen_range(0..10_000_000));
        }
        alg::radix_sort(&mut test_data, 10);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    #[test]
    pub fn insert_backwards_from_100() {
        let mut test_data: Vec<i32> = Vec::new();
        for i in 0..100 {
            test_data.push(100 - i);
        }
        alg::insertion_sort(&mut test_data);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    // tree sort tests

    #[test]
    pub fn tree_backwards_from_100() {
        let mut test_data: Vec<i32> = Vec::new();
        for i in 0..100 {
            test_data.push(100 - i);
        }
        alg::tree_sort(&mut test_data);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    #[test]
    pub fn tree_random_100_elements() {
        let mut test_data: Vec<i32> = Vec::new();
        for _ in 0..100 {
            test_data.push(rand::thread_rng().gen_range(0..10_000));
        }
        alg::tree_sort(&mut test_data[..]);
        assert_eq!(true, alg::is_sorted(&test_data));
    }

    #[test]
    pub fn tree_random_1_000_000_elements() {
        let mut test_data: Vec<i32> = Vec::new();
        for _ in 0..1_000_000 {
            test_data.push(rand::thread_rng().gen_range(0..10_000_000));
        }
        alg::tree_sort(&mut test_data[..]);
        assert_eq!(true, alg::is_sorted(&test_data[..]));
    }

    // selection sort tests

    #[test]
    pub fn selection_backwards_from_100() {
        let mut test_data: Vec<i32> = Vec::new();
        for i in 0..100 {
            test_data.push(100 - i);
        }
        alg::selection_sort(&mut test_data[..]);
        assert_eq!(true, alg::is_sorted(&test_data[..]));
    }

    #[test]
    pub fn selection_random_100_elements() {
        let mut test_data: Vec<i32> = Vec::new();
        for _ in 0..100 {
            test_data.push(rand::thread_rng().gen_range(0..10_000));
        }
        alg::selection_sort(&mut test_data[..]);
        assert_eq!(true, alg::is_sorted(&test_data[..]));
    }
}