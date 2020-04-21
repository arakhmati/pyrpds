#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

#[pymodule]
fn pyrpds(py: Python, m: &PyModule) -> PyResult<()> {
    pyrpds_rust::list::py_binding(py, m)?;
    pyrpds_rust::map::py_binding(py, m)?;
    pyrpds_rust::set::py_binding(py, m)?;
    pyrpds_rust::vector::py_binding(py, m)?;

    Ok(())
}
