use matrix_solver::direct_solution::gaussian_elimination::method;

fn main() {
    let mut matrix = vec![
        vec![1.5, 2.0, 1.0, -1.0, -2.0, 1.0, 1.0],
        vec![3.0, 3.0, -1.0, 16.0, 18.0, 1.0, 1.0],
        vec![1.0, 1.0, 3.0, -2.0, -6.0, 1.0, 1.0],
        vec![1.0, 1.0, 99.0, 19.0, 2.0, 1.0, 1.0],
        vec![1.0, -2.0, 16.0, 1.0, 9.0, 10.0, 1.0],
        vec![1.0, 3.0, 1.0, -5.0, 1.0, 1.0, 95.0],
        // vec![2.0, 6.0, 2.0, -10.0, 2.0, 2.0, 190.0],
    ];

    let mut matrix2: Vec<Vec<f64>> = vec![
        vec![3.0, -1.0, -1.0, 1.0],
        vec![-1.0, 3.0, -1.0, 2.0],
        vec![-1.0, -1.0, 3.0, 1.0],
    ];

    let result = method(&mut matrix2);

    println!("{:?}", result);
}
