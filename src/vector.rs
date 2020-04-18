use std::hash::{Hash, Hasher};

use pyo3::class::basic::CompareOp;
use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};
use pyo3::types::PyList;
use pyo3::{exceptions, IntoPy, PyAny, PyCell, PyErr, Python};

#[pyclass]
pub struct Vector {
    value: rpds::Vector<PyObject>,
}

#[pymethods]
impl Vector {
    #[new]
    fn new() -> Self {
        Vector {
            value: rpds::Vector::new(),
        }
    }

    fn set(&self, index: usize, object: PyObject) -> PyResult<Self> {
        match self.value.set(index, object) {
            Some(value) => Ok(Self { value }),
            None => Err(PyErr::new::<exceptions::IndexError, _>("")),
        }
    }

    fn push_back(&mut self, object: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self.value.push_back(object),
        };
        Ok(new_self)
    }

    fn extend(&mut self, list_as_any: &PyAny) -> PyResult<Self> {
        let list = list_as_any.downcast::<PyList>()?;

        let gil = Python::acquire_gil();
        let py = gil.python();

        let mut new_self = Self {
            value: self.value.clone(),
        };
        for element in list {
            new_self = Self {
                value: new_self.value.push_back(element.into_py(py)),
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
        let first = self.value.first();
        Ok(first)
    }

    fn last(&self) -> PyResult<Option<&PyObject>> {
        let last = self.value.last();
        Ok(last)
    }

    fn get(&self, index: usize) -> PyResult<Option<&PyObject>> {
        if index >= self.value.len() {
            return Err(PyErr::new::<exceptions::IndexError, _>(""));
        }

        let element = self.value.get(index);
        Ok(element)
    }
}

impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Add the hash of length so that if two collections are added one after the other it doesn't
        // hash to the same thing as a single collection with the same elements in the same order.
        self.value.len().hash(state);

        let gil = Python::acquire_gil();
        let py = gil.python();
        for element in self.value.iter() {
            let element_hash = super::hash::hash_py_object(py, element);
            element_hash.hash(state);
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

#[pyproto]
impl PyObjectProtocol for Vector {
    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other_as_any: &PyAny, op: CompareOp) -> PyResult<bool> {
        let other_as_cell = other_as_any.downcast::<PyCell<Vector>>()?;
        let other = other_as_cell.borrow();

        match op {
            CompareOp::Eq => Ok(self.value == other.value),
            CompareOp::Ne => Ok(self.value != other.value),
            _ => panic!("Invalid CompareOp"),
        }
    }
}
