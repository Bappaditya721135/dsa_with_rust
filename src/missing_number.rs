fn main() {
    let arr = vec![0, 1, 3, 4, 5];
    find_missing_number(arr);
}


// fn find_missing_number(arr: Vec<i32>) {
//     let n = arr.len();
//     let mut total_sum: i32 = ((n * (n + 1))/2).try_into().unwrap();

//     for v in arr {
//         total_sum -=v;
//     }

//     println!("total sum: {:?}", total_sum);

// }

fn find_missing_number(arr: Vec<i32>) {
    let n = arr.len();
    let mut all_xor = n as i32;

    for i in 0..n {
        all_xor = all_xor ^ i as i32;
        all_xor = all_xor ^ arr[i];
    }

    println!("all xor: {:?}", all_xor);
}