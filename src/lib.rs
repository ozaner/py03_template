use pyo3::prelude::*;

/// Adds two numbers.
#[pyfunction]
fn add(left: usize, right: usize) -> PyResult<usize> {
    Ok(left + right)
}

/// A Python module implemented in Rust.
#[pymodule]
fn py03_template(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result.unwrap(), 4);
    }
}
