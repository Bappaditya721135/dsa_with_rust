// SUBARRAY SUM EQUALS TO K 

fn main() {
    let arr = vec![2, 7, 5, 8, 1, 3];
    let target = 9;

    let result: i32 = subarray_sum_count(arr, target);
    println!("result: {:?}", result);
}

// BRUIT FORCE APPROACH 
// fn subarray_sum_count(arr: Vec<i32>, target: i32) -> i32 {
//     let mut count = 0;

//     for i in 0..arr.len() {
//         let mut sum = 0;
//         for j in i..arr.len() {
//             sum += arr[j];
//             if sum == target {
//                 count +=1;
//             }
//         }
//     }
//     count 
// }

// OPTIMIZE APPROACH 
fn subarray_sum_count(arr: Vec<i32>, target: i32) -> i32 {
    let mut count = 0;
    let mut prefix_sum = 0;
    for num in arr {
        prefix_sum += num;
        let needed = prefix_sum - target;
        println!("prefix sum: {:?}, needed: {:?}", prefix_sum, needed);
    }
    count
}