use std::hash::{Hash, Hasher};

use pyo3::{IntoPy, PyAny, PyCell, Python};
use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::class::basic::CompareOp;
use pyo3::prelude::{pyclass, pymethods, PyObject, pyproto, PyResult};
use pyo3::types::PyList;
use rpds;

#[pyclass]
pub struct Vector {
    vector: rpds::Vector<PyObject>,
}

#[pymethods]
impl Vector {
    #[new]
    fn new() -> Self {
        Vector {
            vector: rpds::Vector::new(),
        }
    }

    fn set(&self, index: usize, object: PyObject) -> PyResult<Self> {
        let vector = match self.vector.set(index, object) {
            Some(vector) => vector,
            None => panic!("set failed!"),
        };
        let py_vector = Self { vector };
        Ok(py_vector)
    }

    fn push_back(&mut self, object: PyObject) -> PyResult<Self> {
        let py_vector = Self {
            vector: self.vector.push_back(object),
        };
        Ok(py_vector)
    }

    fn extend(&mut self, list_as_any: &PyAny) -> PyResult<Self> {
        let list = list_as_any.downcast::<PyList>()?;

        let gil = Python::acquire_gil();
        let py = gil.python();

        let mut py_vector = Self { vector: self.vector.clone() };
        for element in list {
            py_vector = Self {
                vector: py_vector.vector.push_back(element.into_py(py)),
            };
        }
        Ok(py_vector)
    }

    fn drop_last(&mut self) -> PyResult<Self> {
        let vector = match self.vector.drop_last() {
            Some(vector) => vector,
            None => panic!("drop_last failed!"),
        };
        let py_vector = Self { vector };
        Ok(py_vector)
    }

    fn first(&self) -> PyResult<Option<&PyObject>> {
        let first = self.vector.first();
        Ok(first)
    }

    fn last(&self) -> PyResult<Option<&PyObject>> {
        let last = self.vector.last();
        Ok(last)
    }

    fn get(&self, index: usize) -> PyResult<Option<&PyObject>> {
        let element = self.vector.get(index);
        Ok(element)
    }
}

impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Add the hash of length so that if two collections are added one after the other it doesn't
        // hash to the same thing as a single collection with the same elements in the same order.
        self.vector.len().hash(state);

        let gil = Python::acquire_gil();
        let py = gil.python();
        for element in self.vector.iter() {
            let element_hash = super::hash::hash_py_object(py, element);
            element_hash.hash(state);
        }
    }
}

#[pyproto]
impl PySequenceProtocol for Vector {
    fn __len__(&self) -> PyResult<usize> {
        let len = self.vector.len();
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
            CompareOp::Eq => Ok(self.vector == other.vector),
            CompareOp::Ne => Ok(self.vector != other.vector),
            _ => panic!("Invalid CompareOp"),
        }
    }
}
