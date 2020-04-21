#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

pub mod macros; #[rustfmt::skip]
pub mod iterators;
pub mod list;
pub mod map;
pub mod object;
pub mod set;
pub mod vector;

pub use crate::list::List;
pub use crate::map::Map;
pub use crate::object::Object;
pub use crate::set::Set;
pub use crate::vector::Vector;

#[pymodule]
fn pyrpds(py: Python, m: &PyModule) -> PyResult<()> {
    list::py_binding(py, m)?;
    map::py_binding(py, m)?;
    set::py_binding(py, m)?;
    vector::py_binding(py, m)?;

    Ok(())
}
