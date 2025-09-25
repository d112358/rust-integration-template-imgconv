use pyo3::prelude::*;
use std::path::PathBuf;

/// Python wrapper for convert_image
#[pyfunction]
fn convert_image_py(input: String, output: String) -> PyResult<()> {
    let input = PathBuf::from(input);
    let output = PathBuf::from(output);

    imgconv_core::convert_image(&input, &output)
        .map_err(|e: String| pyo3::exceptions::PyRuntimeError::new_err(e))
}

#[pymodule]
fn imgconv_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(convert_image_py, m)?)?;
    Ok(())
}