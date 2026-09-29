// 3SUM 

fn main() {
    let arr = vec![-1, -1, 0, 2, 1];
    let result: Vec<i32> = find_triplets(arr);
    println!("result: {:?}", result);
}

// BRUIT FORCE APPROACH 
// fn find_triplets(arr: Vec<i32>) -> Vec<i32> {
//     for i in 0..arr.len() {
//         for j in (i + 1)..arr.len() {
//             for k in (j + 1)..arr.len() {
//                 let result = arr[i] + arr[j] + arr[k];
//                 if result == 0 {
//                    return vec![arr[i], arr[j], arr[k]];
//                 }
//             }
//         }
//     }
//     vec![]
// }


fn find_triplets(mut arr: Vec<i32>) -> Vec<i32> {

        arr.sort();
    for i in 0..arr.len() {
        if arr[i] == arr[i + 1] {
            continue;
        }
        let mut left = i + 1;
        let mut right = arr.len() - 1;

        while left < right {
            let sum = arr[i] + arr[left] + arr[right];
            if sum == 0 {
            return vec![arr[i], arr[left], arr[right]];
        } else if sum < 0 {
            left +=1;
        } else if sum > 0 {
            right -=1;
        }
        }
        
    }

    vec![]
}


