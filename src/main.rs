// SET MATRIX ZEROES 

fn main() {
    let matrix = vec![
        vec![1, 2, 3, 4],
        vec![9, 7, 0, 4],
        vec![5, 1, 3, 9],
    ];

    matrix_zeros(matrix);
}


fn matrix_zeros(matrix: Vec<Vec<i32>>) {
    let rows = matrix.len();
    if rows == 0 {
        return;
    }

    let cols = matrix[0].len();

    let matrix_cp = matrix.clone();

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == 0 {
                println!("0 found");
                println!("i: {:?}", i);
                println!("j: {:?}", j);
            }
        }
    }
}