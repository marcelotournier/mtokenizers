/// MTokenizers
/// Fast Tokenizers for usage in Rust and Python.

mod tokenizers;
use pyo3::prelude::*;
use tokenizers::latin1::{encode, decode};

#[pyfunction]
fn encode_latin1(input_string: &str, max_len: usize) -> PyResult<Vec<u8>> {
    Ok(encode(input_string, max_len))
}

#[pyfunction]
fn decode_latin1(vector: Vec<u8>) -> PyResult<String> {
    Ok(decode(vector))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn mtokenizers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_latin1, m)?)?;
    m.add_function(wrap_pyfunction!(decode_latin1, m)?)?;
    
    Ok(())
}
