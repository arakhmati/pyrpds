use pyo3::{exceptions, import_exception, PyErr, PyObject, PyResult, Python};
use std::hash::{Hash, Hasher};

import_exception!(io, UnsupportedOperation);

pub struct Object {
    pub py_object: PyObject,
}

impl Object {
    #[must_use]
    pub fn new(py_object: PyObject) -> Self {
        Object { py_object }
    }
}

impl PartialEq for Object {
    #[inline]
    fn eq(&self, object: &Object) -> bool {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let args = (&object.py_object,);
        let py_eq = self.py_object.call_method1(py, "__eq__", args);
        let eq = match py_eq {
            Err(_) => Err(PyErr::new::<exceptions::NotImplementedError, _>(
                "__eq__ method is not implemented!",
            )),
            Ok(x) => x.extract::<bool>(py),
        };
        match eq {
            Ok(eq) => eq,
            _ => panic!("__eq__ failed!"),
        }
    }
}

impl Eq for Object {}

fn hash_object(py: Python, object: &Object) -> PyResult<isize> {
    let element_hash_object = object.py_object.call_method0(py, "__hash__");
    match element_hash_object {
        Err(_) => Err(PyErr::new::<exceptions::NotImplementedError, _>(
            "__hash__ method is not implemented!",
        )),
        Ok(x) => x.extract::<isize>(py),
    }
}

impl Hash for Object {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let object_hash_result = hash_object(py, self);

        match object_hash_result {
            Ok(object_hash) => object_hash.hash(state),
            _ => panic!("__hash__ failed!"),
        };
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();

        Self { py_object: self.py_object.clone_ref(py) }
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn extract_py_object(object: Option<&Object>) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    match object {
        Some(object) => Ok(object.py_object.clone_ref(py)),
        None => Err(PyErr::new::<exceptions::RuntimeError, _>(
            "Invalid call. Most likely container is empty!",
        )),
    }
}
