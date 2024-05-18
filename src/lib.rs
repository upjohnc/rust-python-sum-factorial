use std::fs;

use pyo3::prelude::*;

/// From the file path, it reads the first line and returns
/// the sum of factorials of the individual digits
#[pyfunction]
fn file_sum_factorial(file_path: String) -> PyResult<i32> {
    Ok(read_file(file_path))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_factorial_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(file_sum_factorial, m)?)?;
    Ok(())
}

fn read_file(file_path: String) -> i32 {
    let mem = fs::read_to_string(file_path).expect("Should have been a file");
    let t = mem.lines().collect::<Vec<_>>()[0];
    let mut result = vec![];
    for i in t.chars() {
        result.push(i.to_string().parse::<i32>().unwrap());
    }
    sum_factorial(&result)
}

fn sum_factorial(mine: &Vec<i32>) -> i32 {
    let mut sum_value = 0;
    for i in mine {
        sum_value += factorial(*i);
    }
    sum_value
}

fn factorial(number: i32) -> i32 {
    if number <= 1 {
        return 1;
    }

    factorial(number - 1) * number
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_factorial(value: i32, expected: i32) {
        let result = factorial(value);
        assert_eq!(
            result, expected,
            "Testing: value {}, expected {}",
            value, expected
        );
    }

    #[test]
    fn test_factorial() {
        for (value, expected) in [(1, 1), (2, 2), (3, 6), (4, 24)] {
            check_factorial(value, expected);
        }
    }

    fn check_sum_factorial(value: Vec<i32>, expected: i32) {
        let result = sum_factorial(&value);
        assert_eq!(
            result, expected,
            "Testing: value {:?}, expected {}",
            value, expected
        );
    }

    #[test]
    fn test_sum_factorial() {
        for (value, expected) in [(vec![1, 2, 3], 9), (vec![1], 1)] {
            check_sum_factorial(value, expected)
        }
    }

    #[test]
    fn big_test() {
        let result = read_file("./src/text.txt".to_string());
        assert_eq!(result, 3);
    }
}
