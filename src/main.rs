// SPIRAL MATRIX 
fn main() {
    let matrix = vec![
        vec![2, 9, 4, 5],
        vec![3, 6, 3, 8],
        vec![11, 7, 2,10],
    ];

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
    let mut right = cols as i32 - 1;
    let mut bottom = rows as i32 - 1;
    let mut left = 0;

    println!("matrix: {:?}", matrix);
    while top <= bottom || left <= right {
        // left to right 

        for i in left..=right {
            println!("left to right values: {:?}", matrix[top as usize][i as usize])
        }
        top +=1;

        // top to bottom
        for i in 
        // right to left 

        // bottom to top 
    }

    // println!("matrix: {:?}", matrix);
    // println!("top: {:?}", top);
    // println!("right: {:?}", right);
    // println!("bottom: {:?}", bottom);
    // println!("left: {:0}", left);
    
}