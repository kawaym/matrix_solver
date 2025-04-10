use matrix_solver::direct_solution::gaussian_elimination::method as gauss_elimination;
use matrix_solver::direct_solution::lu_decomposition::method as lu_decomposition;
use matrix_solver::file_utils;
use matrix_solver::iteractive_solution::jacobi_method::method as jacobi_method;
use matrix_solver::time_utils;

fn main() {
    let mut matrix = file_utils::read_matrix("matrix.txt");

    println!("{:?}", matrix);

    let result = time_utils::calculate_direct_runtime(&mut matrix, &mut gauss_elimination);
    let result = time_utils::calculate_direct_runtime(&mut matrix, &mut lu_decomposition);
    let result =
        time_utils::calculate_iteractve_runtime(&mut matrix, None, None, &mut jacobi_method);
    println!("{:?}", result);
}
