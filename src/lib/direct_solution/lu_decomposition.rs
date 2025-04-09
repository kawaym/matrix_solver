pub fn method(matrix: &mut Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let n = matrix.len();
    let m = matrix[0].len();

    if m != n + 1 {
        return None;
    }

    let a: Vec<Vec<f64>> = matrix.iter().map(|row| row[..n].to_vec()).collect();
    let b: Vec<f64> = matrix.iter().map(|row| row[n]).collect();

    let (l, u) = lu_decomposition(&a)?;

    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..i {
            sum += l[i][j] * y[j];
        }
        y[i] = b[i] - sum;
    }

    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = 0.0;
        for j in (i + 1)..n {
            sum += u[i][j] * x[j];
        }

        if u[i][i].abs() < 1e-12 {
            return None;
        }

        x[i] = (y[i] - sum) / u[i][i];
    }

    Some(x)
}

fn lu_decomposition(matrix: &Vec<Vec<f64>>) -> Option<(Vec<Vec<f64>>, Vec<Vec<f64>>)> {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut l = vec![vec![0.0; m]; m];
    let mut u = matrix.clone();

    for i in 0..m {
        l[i][i] = 1.0;
    }

    for k in 0..m.min(n) {
        if u[k][k].abs() < 1e-12 {
            return None;
        }

        for i in (k + 1)..m {
            let factor = u[i][k] / u[k][k];
            l[i][k] = factor;

            for j in k..n {
                u[i][j] -= factor * u[k][j];
            }
        }
    }

    Some((l, u))
}
