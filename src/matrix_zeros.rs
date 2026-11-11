// SET MATRIX ZEROES 

fn main() {
    // let matrix = vec![
    //     vec![1, 0, 3, 4],
    //     vec![9, 7, 0, 4],
    //     vec![5, 1, 3, 9],
    // ];

    let matrix = vec![
    vec![2, 0, 3],
    vec![1, 5, 6],
    vec![7, 8, 9],
];
// let matrix = vec![
//     vec![1, 2, 3],
//     vec![0, 5, 6],
//     vec![7, 8, 9],
// ];

    matrix_zeros(matrix);
}


// fn matrix_zeros(matrix: Vec<Vec<i32>>) {
//     let rows = matrix.len();
//     if rows == 0 {
//         return;
//     }

//     let cols = matrix[0].len();

//     let mut matrix_cp = matrix.clone();

//     for i in 0..rows {
//         for j in 0..cols {
//             if matrix[i][j] == 0 {
//                 // set the entire to zerors 
//                 for col in 0..cols {
//                     matrix_cp[i][col] = 0;
//                 }

//                 // set the entire col to zerors 

//                 for row in 0..rows {
//                     matrix_cp[row][j] = 0;
//                 }

//                 println!("matrix cp: {:?}", matrix_cp);
//             }
//         }
//     }
// }


// fn matrix_zeros(mut matrix: Vec<Vec<i32>>) {
//     let rows = matrix.len();

//     if rows == 0 {
//         return;
//     }

//     let cols = matrix[0].len();

//     let mut zero_rows = vec![false; rows];
//     let mut zero_cols = vec![false; cols];

    

//     for i in 0..rows {
//         for j in 0..cols {
//             if matrix[i][j] == 0 {
//                 zero_rows[i] = true;
//                 zero_cols[j] = true;
//             }
//         }
//     }

//     for i in 0..rows {
//         for j in 0..cols {
//             if zero_rows[i] == true || zero_cols[j] == true {
//                 matrix[i][j] = 0;
//             }
//         }
//     }
//     println!("matrix: {:?}", matrix);
// }

fn matrix_zeros(mut matrix: Vec<Vec<i32>>) {

    let rows = matrix.len();

    if rows == 0 {
        return;
    }
    let cols = matrix[0].len();

    if cols == 0 {
        return;
    }

    let mut first_row_zero = false;
    let mut first_col_zero = false;

    println!("matrix: {:?}", matrix);
    // check if there are any zero in the first row 
    for i in 0..cols {
        if matrix[0][i] == 0 {
            first_row_zero = true;
        }
    }

    // check if there are any zero in the first col 
    for j in 0..rows {
        if matrix[j][0] == 0 {
            first_col_zero = true;
        }
    }


    // now mark the first col & row index to zero 
    for i in 1..rows {
        for j in 1..cols {
            if matrix[i][j] == 0 {
                matrix[0][j] = 0;
                matrix[i][0] = 0;
            }
        }
    }

    // now if the row or the col is zero make all the elements zero 
    for i in 1..rows {
        for j in 1..cols {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }

    
    // check if first col is zero then make the row zerors 
    if first_col_zero {
        for i in 0..rows {
            matrix[i][0] = 0;
            // println!("i vlaue: {:?}", matrix[i][0]);
        }
    }

    // check if first row is zeor then make all the col in that row zeror 
    if first_row_zero {
        for j in 0..cols {
            matrix[0][j] = 0;
        }
    }

    println!("matrix after: {:?}", matrix);

    
}