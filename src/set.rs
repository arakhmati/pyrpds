use std::hash::{Hash, Hasher};

use crate::object::{extract_py_object, Object};
use pyo3::class::basic::CompareOp;
use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};
use pyo3::{PyAny, PyCell, PyIterProtocol, PyRefMut};

type RpdsSet = rpds::set::hash_trie_set::HashTrieSet<Object>;

#[pyclass]
pub struct Set {
    value: RpdsSet,
}

#[pymethods]
impl Set {
    #[new]
    fn new() -> Self {
        Set {
            value: RpdsSet::new(),
        }
    }

    fn insert(&mut self, py_object: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self.value.insert(Object::new(py_object)),
        };
        Ok(new_self)
    }

    fn remove(&mut self, py_object: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self.value.remove(&Object::new(py_object)),
        };
        Ok(new_self)
    }
}

impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Add the hash of length so that if two collections are added one after the other it doesn't
        // hash to the same thing as a single collection with the same elements in the same order.
        self.value.size().hash(state);
        for element in self.value.iter() {
            element.hash(state);
        }
    }
}

#[pyproto]
impl PySequenceProtocol for Set {
    fn __len__(&self) -> PyResult<usize> {
        let len = self.value.size();
        Ok(len)
    }

    fn __contains__(&self, py_object: PyObject) -> PyResult<bool> {
        Ok(self.value.contains(&Object::new(py_object)))
    }
}

#[pyproto]
impl PyIterProtocol for Set {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<crate::iterators::PyObjectIterator> {
        let mut elements = std::vec::Vec::new();
        for element in slf.value.iter() {
            elements.push(extract_py_object(Some(element))?)
        }

        Ok(crate::iterators::PyObjectIterator::new(
            elements.into_iter(),
        ))
    }
}

py_object_protocol!(Set);
