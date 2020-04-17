use pyo3::import_exception;
use pyo3::{PyObject, Python};

import_exception!(io, UnsupportedOperation);

pub fn hash_py_object(py: Python, element: &PyObject) -> isize {
    let element_hash_object = element.call_method0(py, "__hash__");
    let element_hash = match element_hash_object {
        Err(_) => Err(UnsupportedOperation::py_err("Element cannot be hashed")),
        Ok(x) => x.extract::<isize>(py),
    };
    match element_hash {
        Err(_) => 0,
        Ok(x) => x,
    }
}
