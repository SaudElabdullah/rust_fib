use pyo3::prelude::*;

#[pyfunction]
fn rust_fibonacci(num: u32) -> u32{
    if num == 0 {
        return 0;
    }
    else if num == 1{
        return  1;
    }
    else {
        return rust_fibonacci(num - 1) + rust_fibonacci(num - 2);
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_fib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_fibonacci, m)?)?;
    Ok(())
}