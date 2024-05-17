use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_factorial_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

fn sum_factorial(mine: Vec<i32>) -> i32 {
    let mut sum_value = 0;
    for i in mine {
        sum_value += factorial(i);
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

    #[test]
    fn test_one() {
        let result = factorial(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_two() {
        let result = factorial(2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_three() {
        let result = factorial(3);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_four() {
        let result = factorial(4);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_sum_one() {
        let result = sum_factorial(vec![1]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_sum_three() {
        let result = sum_factorial(vec![1, 2, 3]);
        assert_eq!(result, 9);
    }
}
