use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

mod macros; #[rustfmt::skip]
mod list;
mod map;
mod object;
mod set;
mod vector;

#[pymodule]
fn pyrpds(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<list::List>()?;
    m.add_class::<vector::Vector>()?;
    m.add_class::<set::Set>()?;
    m.add_class::<map::Map>()?;

    Ok(())
}
