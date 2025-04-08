pub fn method(matrix: &mut Vec<Vec<f64>>) {
    let m = matrix.len();
    let n = matrix[0].len() - 1;

    let mut pivot_row = 1;
    let mut pivot_column = 1;

    while pivot_row <= m && pivot_column <= n {
        let i_max = find_max(matrix, m, pivot_row, pivot_column);

        if matrix[i_max][pivot_column] == 0.0 {
            pivot_column += 1;
        } else {
            swap_row(matrix, pivot_row, i_max);

            for i in pivot_row + 1..m {
                let factor = matrix[i][pivot_column] / matrix[pivot_row][pivot_column];
                matrix[i][pivot_column] = 0.0;

                for j in pivot_column + 1..n {
                    matrix[i][j] = matrix[i][j] - matrix[pivot_row][j] * factor;
                }
            }

            pivot_row += 1;
            pivot_column += 1;
        }
    }
}

fn find_max(
    matrix: &mut Vec<Vec<f64>>,
    size: usize,
    pivot_row: usize,
    pivot_column: usize,
) -> usize {
    let mut max = 0.0;
    let mut index: usize = 1;

    for i in pivot_row..size {
        if matrix[i][pivot_column] > max {
            max = matrix[i][pivot_column];
            index = i;
        }
    }

    index
}

fn swap_row(matrix: &mut Vec<Vec<f64>>, source_row: usize, target_row: usize) {
    let tmp = matrix[source_row].clone();
    matrix[source_row] = matrix[target_row].clone();
    matrix[target_row] = tmp;
}
