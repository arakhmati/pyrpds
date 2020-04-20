use std::hash::{Hash, Hasher};

use super::object::{extract_optional_py_object, Object};
use pyo3::class::basic::CompareOp;
use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};
use pyo3::types::PyList;
use pyo3::{exceptions, IntoPy, PyAny, PyCell, PyErr, Python};
use std::panic;

#[pyclass]
pub struct Vector {
    value: rpds::Vector<Object>,
}

#[pymethods]
impl Vector {
    #[new]
    fn new() -> Self {
        Vector {
            value: rpds::Vector::new(),
        }
    }

    fn set(&self, index: usize, py_object: PyObject) -> PyResult<Self> {
        match self.value.set(index, Object::new(py_object)) {
            Some(value) => Ok(Self { value }),
            None => Err(PyErr::new::<exceptions::IndexError, _>("")),
        }
    }

    fn push_back(&mut self, py_object: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self.value.push_back(Object::new(py_object)),
        };
        Ok(new_self)
    }

    fn extend(&mut self, list_as_any: &PyAny) -> PyResult<Self> {
        let py_list = list_as_any.downcast::<PyList>()?;

        let gil = Python::acquire_gil();
        let py = gil.python();

        let mut new_self = Self {
            value: self.value.clone(),
        };
        for element in py_list {
            let py_object = element.into_py(py);
            let object = Object::new(py_object);
            new_self = Self {
                value: new_self.value.push_back(object),
            };
        }
        Ok(new_self)
    }

    fn drop_last(&mut self) -> PyResult<Self> {
        let value = match self.value.drop_last() {
            Some(vector) => vector,
            None => panic!("drop_last failed!"),
        };
        let py_vector = Self { value };
        Ok(py_vector)
    }

    fn first(&self) -> PyResult<Option<&PyObject>> {
        let first = extract_optional_py_object(self.value.first());
        Ok(first)
    }

    fn last(&self) -> PyResult<Option<&PyObject>> {
        let last = extract_optional_py_object(self.value.last());
        Ok(last)
    }

    fn get(&self, index: usize) -> PyResult<Option<&PyObject>> {
        if index >= self.value.len() {
            return Err(PyErr::new::<exceptions::IndexError, _>(""));
        }

        let element = extract_optional_py_object(self.value.get(index));
        Ok(element)
    }
}

impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Add the hash of length so that if two collections are added one after the other it doesn't
        // hash to the same thing as a single collection with the same elements in the same order.
        self.value.len().hash(state);
        for element in self.value.iter() {
            element.hash(state);
        }
    }
}

#[pyproto]
impl PySequenceProtocol for Vector {
    fn __len__(&self) -> PyResult<usize> {
        let len = self.value.len();
        Ok(len)
    }
}

py_object_protocol!(Vector);
