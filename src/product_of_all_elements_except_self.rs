// PRODUCT OF ALL ELEMENTS EXCEPT SELF 

fn main() {
    let arr = vec![2, 4, 3, 7];

    product(arr);
}

fn product(arr: Vec<i32>) {
    let mut result: Vec<i32> = vec![1; arr.len()];
    let mut product = 1;
    // left product 
    for i in 0..arr.len() {
        result[i] = product;
        product = product * arr[i];
    }
    product = 1;
    // right product 
    for i in (0..arr.len()).rev() {
        result[i] = result[i] * product;
        product = product * arr[i]
    }
    
    println!("result: {:?}", result);

}
