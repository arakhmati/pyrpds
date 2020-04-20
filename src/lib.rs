use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

mod macros; #[rustfmt::skip]
mod iterators;
mod list;
mod map;
mod object;
mod set;
mod vector;

#[pymodule]
fn pyrpds(_py: Python, m: &PyModule) -> PyResult<()> {
    list::py_binding(_py, m)?;
    map::py_binding(_py, m)?;
    set::py_binding(_py, m)?;
    vector::py_binding(_py, m)?;

    Ok(())
}
