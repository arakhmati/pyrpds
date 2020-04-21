#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]

use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

mod macros; #[rustfmt::skip]
mod iterators;
mod list;
mod map;
mod object;
mod set;
mod vector;

#[pymodule]
fn pyrpds(py: Python, m: &PyModule) -> PyResult<()> {
    list::py_binding(py, m)?;
    map::py_binding(py, m)?;
    set::py_binding(py, m)?;
    vector::py_binding(py, m)?;

    Ok(())
}
