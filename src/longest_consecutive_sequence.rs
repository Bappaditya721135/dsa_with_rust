// LONGEST CONSECUTIVE SEQUENCE 
use std::collections::HashSet;
fn main() {
    //Input:
    // [100, 4, 200, 1, 3, 2]

    // Output:
    // 4

    let arr = vec![100, 4, 200, 1, 3, 2];
    let result: i32 = longest_consecutive_sequence(arr);
    println!("result: {:?}", result);
}

// BRUIT FORCE APPROACH 
// fn longest_consecutive_sequence(arr: Vec<i32>) -> i32{
//     let mut longest = 0;
//     for i in 0..arr.len() {
//         let mut current_longest = 1;
//         let mut next = arr[i] + 1;
//         loop {

//             if arr.contains(&next) {
//                 current_longest += 1;
//                 next +=1;
//             } else {
//                 break;
//             }
//         }
//         longest = longest.max(current_longest);
//     }
//     longest
// }


fn longest_consecutive_sequence(arr: Vec<i32>) -> i32 {
    let set: HashSet<i32> = arr.into_iter().collect();
    let mut longest = 0;

    for &num in &set {
        
        // CHECK IF THIS NUM IS THE BEGINNING OF A SEQUENCE 
        if !set.contains(&(num - 1)) {
            let mut next = num + 1;
            let mut count = 1;
            while set.contains(&next) {
                count +=1;
                next +=1;
            }
            longest = longest.max(count);
        }
    }

    longest
}