fn main() {

    let mut matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    rotate_matrix(&mut matrix);
}

// i=1,
// j=2

// matrix[i][j] = 6
// matrix[j][i] = 8


fn rotate_matrix(mut matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    
    if rows == 0 {
        return;
    }
    let cols = matrix[0].len();

    if cols == 0 {
        return;
    }

    for i in 0..rows {
        for j in (i + 1)..rows {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp ;
        }
    }

    // NOW REVERSE THE MATRIX 
    for row in matrix.iter_mut() {
        row.reverse();
    }
    

    println!("after matrix: {:?}", matrix);
}