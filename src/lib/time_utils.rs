use std::time::Instant;

pub fn calculate_direct_runtime(
    matrix: &mut Vec<Vec<f64>>,
    f: &mut dyn FnMut(&mut Vec<Vec<f64>>) -> Option<Vec<f64>>,
) -> Option<Vec<f64>> {
    let start = Instant::now();

    let result = f(matrix);

    let duration = start.elapsed();

    println!("{}", duration.as_nanos());

    result
}

pub fn calculate_iteractve_runtime(
    matrix: &mut Vec<Vec<f64>>,
    max_iter: Option<usize>,
    tolerance: Option<f64>,
    f: &mut dyn FnMut(&mut Vec<Vec<f64>>, Option<usize>, Option<f64>) -> Option<Vec<f64>>,
) -> Option<Vec<f64>> {
    let start = Instant::now();

    let result = f(matrix, max_iter, tolerance);

    let duration = start.elapsed();

    println!("{}", duration.as_nanos());

    result
}
