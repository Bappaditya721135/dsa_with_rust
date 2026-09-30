// LONGEST CONSECUTIVE SEQUENCE 
fn main() {
    //Input:
    // [100, 4, 200, 1, 3, 2]

    // Output:
    // 4

    let arr = vec![100, 4, 200, 1, 3, 2];
    let result: i32 = longest_consecutive_sequence(arr);
    println!("result: {:?}", result);
}


fn longest_consecutive_sequence(arr: Vec<i32>) -> i32{
    let mut longest = 0;
    for i in 0..arr.len() {
        let mut current_longest = 1;
        let mut next = arr[i] + 1;
        loop {

            if arr.contains(&next) {
                current_longest += 1;
                next +=1;
            } else {
                break;
            }
        }
        longest = longest.max(current_longest);
    }
    longest
}