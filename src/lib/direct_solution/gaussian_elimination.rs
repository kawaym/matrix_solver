pub fn method(matrix: &mut Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let m = matrix.len();
    let n = matrix[0].len() - 1;

    let mut row = 0;
    for col in 0..n {
        let mut pivot_row = row;
        for i in (row + 1)..m {
            if matrix[i][col].abs() > matrix[pivot_row][col].abs() {
                pivot_row = i;
            }
        }

        if matrix[pivot_row][col].abs() < 1e-12 {
            continue;
        }

        matrix.swap(row, pivot_row);

        for i in (row + 1)..m {
            let factor = matrix[i][col] / matrix[row][col];
            for j in col..=n {
                matrix[i][j] -= factor * matrix[row][j];
            }
        }

        row += 1;
        if row >= m {
            break;
        }
    }

    let mut solution = vec![0.0; n];
    for i in (0..m).rev() {
        let mut leading_var = None;
        for j in 0..n {
            if matrix[i][j].abs() > 1e-12 {
                leading_var = Some(j);
                break;
            }
        }

        match leading_var {
            Some(j) => {
                let mut sum = 0.0;
                for k in (j + 1)..n {
                    sum += matrix[i][k] * solution[k];
                }
                solution[j] = (matrix[i][n] - sum) / matrix[i][j];
            }
            None => {
                if matrix[i][n].abs() > 1e-12 {
                    return None;
                }
            }
        }
    }

    Some(solution)
}
