use std::hash::{Hash, Hasher};

use pyo3::class::basic::CompareOp;
use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};
use pyo3::{PyAny, PyCell, Python};

#[pyclass]
pub struct List {
    value: rpds::List<PyObject>,
}

#[pymethods]
impl List {
    #[new]
    fn new() -> Self {
        List {
            value: rpds::List::new(),
        }
    }

    fn push_front(&mut self, object: PyObject) -> PyResult<Self> {
        let py_list = Self {
            value: self.value.push_front(object),
        };
        Ok(py_list)
    }

    fn drop_first(&mut self) -> PyResult<Self> {
        let value = match self.value.drop_first() {
            Some(list) => list,
            None => panic!("drop_first failed!"),
        };
        let py_list = Self { value };
        Ok(py_list)
    }

    fn reverse(&self) -> PyResult<Self> {
        let reversed = Self {
            value: self.value.reverse(),
        };
        Ok(reversed)
    }

    fn first(&self) -> PyResult<Option<&PyObject>> {
        let first = self.value.first();
        Ok(first)
    }

    fn last(&self) -> PyResult<Option<&PyObject>> {
        let last = self.value.last();
        Ok(last)
    }
}

impl Hash for List {
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
impl PySequenceProtocol for List {
    fn __len__(&self) -> PyResult<usize> {
        let len = self.value.len();
        Ok(len)
    }
}

#[pyproto]
impl PyObjectProtocol for List {
    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other_as_any: &PyAny, op: CompareOp) -> PyResult<bool> {
        let other_as_cell = other_as_any.downcast::<PyCell<List>>()?;
        let other = other_as_cell.borrow();

        match op {
            CompareOp::Eq => Ok(self.value == other.value),
            CompareOp::Ne => Ok(self.value != other.value),
            _ => panic!("Invalid CompareOp"),
        }
    }
}
