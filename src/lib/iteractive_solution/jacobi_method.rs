const MAX_ITER: usize = 100;
const TOLERANCE: f64 = 1e-3;

pub fn method(
    matrix: &mut Vec<Vec<f64>>,
    max_iter: Option<usize>,
    tolerance: Option<f64>,
) -> Option<Vec<f64>> {
    let n = matrix.len();
    let m = matrix[0].len();

    if m != n + 1 {
        return None;
    }

    let a: Vec<Vec<f64>> = matrix.iter().map(|row| row[..n].to_vec()).collect();
    let b: Vec<f64> = matrix.iter().map(|row| row[n]).collect();

    let mut x_old = vec![0.0; n];
    let mut x_new = vec![0.0; n];

    for _ in 0..max_iter.unwrap_or(MAX_ITER) {
        for i in 0..n {
            let mut sum = 0.0;
            for j in 0..n {
                if i != j {
                    sum += a[i][j] * x_old[j];
                }
            }

            if a[i][i].abs() < 1e-12 {
                return None;
            }

            x_new[i] = (b[i] - sum) / a[i][i];
        }

        let mut error = 0.0;
        for i in 0..n {
            error += (x_new[i] - x_old[i]).abs();
        }

        println!("{}", error);
        if error < tolerance.unwrap_or(TOLERANCE) {
            return Some(x_new);
        }

        x_old.copy_from_slice(&x_new);
    }

    None
}
