use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_row_from_line(line: String, expected_size: Option<usize>) -> Option<Vec<f64>> {
    let parts: Vec<f64> = line
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .collect();

    if let Some(size) = expected_size {
        if parts.len() == size {
            return Some(parts);
        }

        return None;
    }

    if parts.len() == 0 {
        return None;
    }

    Some(parts)
}

pub fn read_matrix(filename: &str) -> Vec<Vec<f64>> {
    let mut vector: Vec<Vec<f64>> = Vec::new();

    if let Ok(lines) = read_lines_from_file(filename) {
        let mut count = 0;
        let mut size = 0;
        for line in lines.flatten() {
            if count == 0 {
                if let Some(row) = read_row_from_line(line, None) {
                    size = row.len();
                    vector.push(row);
                }
                count += 1;
            } else {
                if let Some(row) = read_row_from_line(line, Some(size)) {
                    vector.push(row)
                }
            }
        }
    }

    vector
}
