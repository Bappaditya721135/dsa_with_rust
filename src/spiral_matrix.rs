// SPIRAL MATRIX 
fn main() {
    // let matrix = vec![
    //     vec![2, 9, 4],
    //     vec![3, 6, 3],
        // vec![11, 7, 2,10],
        // vec![11, 7, 2,10]
    // ];

//     let matrix = vec![
//     vec![1,  2,  3],
//     vec![4,  5,  6],
//     vec![7,  8,  9],
//     vec![10, 11, 12],
// ];

let matrix = vec![
    vec![1],
    vec![2],
    vec![3],
    vec![4],
];

//     let matrix = vec![
//     vec![1],
//     vec![2],
//     vec![3],
// ];

    // let matrix = vec![
    //     vec![1, 2, 3],
    //     vec![4, 5, 6],
    //     vec![7, 8, 9],
    // ];

    spiral_matrix(matrix);

    // OUTPUT [2, 9, 4, 5, 8, 10, 2, 7, 11, 3, 6, 3]
}


fn spiral_matrix(matrix: Vec<Vec<i32>>) {
    let rows = matrix.len();

    if rows == 0 {
        return;
    }

    let cols = matrix[0].len();

    if cols == 0 {
        return;
    }


    let mut top = 0;
    let mut right = cols as i32 - 1; // 3
    let mut bottom = rows as i32 - 1; // 2
    let mut left = 0;


    let mut result = Vec::with_capacity(rows * cols);

    println!("matrix: {:?}", matrix);

    
    while top <=bottom  && left <= right {
        println!("\n");
        println!("top: {:?}", top);
        println!("bottom: {:?}", bottom);
        println!("left: {:?}", left);
        println!("right: {:?}", right);
        // left to right 
        for i in left..=right {

            result.push(matrix[top as usize][i as usize]);
        }
        top +=1;
        println!("result after first for loop: {:?}", result);
        // top to bottom
        for i in top..=bottom {
            // println!("top to bottom: {:?}", matrix[i as usize][right as usize]);
            result.push(matrix[i as usize][right as usize]);
            // println!("top to bottom i value: {:?}", i);
        }
        right -=1;
        println!("result after second for loop: {:?}", result);
        // right to left 
        if top <= bottom {
            for i in (left..=right).rev() {
            result.push(matrix[bottom as usize][i as usize]);
        }
        bottom -=1;
        }
        println!("result after third for loop: {:?}", result);

        if left <= right {
             // bottom to top 
        for i in (top..=bottom).rev() {
            result.push(matrix[i as usize][left as usize])
        }
        left +=1;
    }
    println!("result after fourth for loop: {:?}", result);
       
    }

    println!("result: {:?}", result);

    
}