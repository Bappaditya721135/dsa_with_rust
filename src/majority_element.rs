// Majority Element (note* majority element always exist)

fn main() {
    let arr = vec![2, 2, 1, 1, 1, 2, 2,];

    majority_element(arr);
}

fn majority_element(arr: Vec<i32>) {

    if arr.len() == 0 {
        return;
    }
    let mut candidate = 0;
    let mut count = 0;

    for v in arr {
        if count == 0 {
            candidate = v;
        }

        if v == candidate {
            count +=1;
        } else {
            count -=1;
        }
    }

    println!("candidate: {:?}", candidate);

}