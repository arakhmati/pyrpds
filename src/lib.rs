use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

mod list;
mod object;
mod set;
mod vector;

#[pymodule]
fn pyrpds(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<list::List>()?;
    m.add_class::<vector::Vector>()?;
    m.add_class::<set::Set>()?;

    Ok(())
}
