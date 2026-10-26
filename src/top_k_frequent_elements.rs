// TOP K FREQUENT ELEMENTS 

use std::collections::HashMap;

fn main() {
    let arr = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result: Vec<i32> = frequent_k_elements(arr, k);
    println!("result: {:?}", result);
}

fn frequent_k_elements(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let arr_length = arr.len();
    let mut number_frequency: HashMap<i32, usize> = HashMap::new();
    let mut result: Vec<i32> = Vec::new();

    for num in arr {
        *number_frequency.entry(num).or_insert(0) +=1;
    }

    let mut bucket: Vec<Vec<i32>> = vec![Vec::new(); arr_length];

    for (k, v) in number_frequency {
        bucket[v].push(k);
    }

    for i in (0..arr_length).rev() {
        for &j in &bucket[i] {
            result.push(j);
        }

        if result.len() == k as usize {
            return result;
        }
    }

    result
}