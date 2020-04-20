use std::hash::{Hash, Hasher};

use super::object::{extract_optional_py_object, Object};
use pyo3::class::basic::CompareOp;
use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};
use pyo3::{PyAny, PyCell};

#[pyclass]
pub struct List {
    value: rpds::List<Object>,
}

#[pymethods]
impl List {
    #[new]
    fn new() -> Self {
        List {
            value: rpds::List::new(),
        }
    }

    fn push_front(&mut self, py_object: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self.value.push_front(Object::new(py_object)),
        };
        Ok(new_self)
    }

    fn drop_first(&mut self) -> PyResult<Self> {
        let value = match self.value.drop_first() {
            Some(list) => list,
            None => panic!("drop_first failed!"),
        };
        let new_self = Self { value };
        Ok(new_self)
    }

    fn reverse(&self) -> PyResult<Self> {
        let reversed = Self {
            value: self.value.reverse(),
        };
        Ok(reversed)
    }

    fn first(&self) -> PyResult<Option<&PyObject>> {
        let first = extract_optional_py_object(self.value.first());
        Ok(first)
    }

    fn last(&self) -> PyResult<Option<&PyObject>> {
        let last = extract_optional_py_object(self.value.last());
        Ok(last)
    }
}

impl Hash for List {
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
impl PySequenceProtocol for List {
    fn __len__(&self) -> PyResult<usize> {
        let len = self.value.len();
        Ok(len)
    }
}

py_object_protocol!(List);
