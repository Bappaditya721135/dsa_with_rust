// MAXIMUM SUBARRAY 
fn main() {
    let arr = vec![-2, 2, -3, -1, -4];
    let result: i32 = max_subarr(arr);
    println!("result: {:?}", result);
}

//BRUIT FORCE APPROACH
// fn max_subarr(arr: Vec<i32>) -> i32 {
//     let mut max_sum = arr[0];
//     for sub_start in 0..arr.len() {
//         let mut current_sum = 0;
//         for sub_end in sub_start..arr.len() {
//             current_sum += arr[sub_end];
//             if current_sum > max_sum {
//                 max_sum = current_sum;
//             }
            
//         }
//     }
//     max_sum
// }


fn max_subarr(arr: Vec<i32>) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = 0;
    for i in 0..arr.len() {
        current_sum += arr[i];
        max_sum = max_sum.max(current_sum);

        if current_sum < 0 {
            current_sum = 0;
        }
    }
    max_sum
}