use std::hash::{Hash, Hasher};

use crate::object::{extract_py_object, Object};
use pyo3::class::basic::CompareOp;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};
use pyo3::{exceptions, PyAny, PyCell, PyErr, PyMappingProtocol, PySequenceProtocol};

type RpdsMap = rpds::map::hash_trie_map::HashTrieMap<Object, Object>;

#[pyclass]
pub struct Map {
    value: RpdsMap,
}

#[pymethods]
impl Map {
    #[new]
    fn new() -> Self {
        Map {
            value: RpdsMap::new(),
        }
    }

    fn insert(&mut self, py_key: PyObject, py_value: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self
                .value
                .insert(Object::new(py_key), Object::new(py_value)),
        };
        Ok(new_self)
    }

    fn remove(&mut self, py_key: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self.value.remove(&Object::new(py_key)),
        };
        Ok(new_self)
    }

    fn get(&self, py_key: PyObject) -> PyResult<PyObject> {
        let key = Object::new(py_key);
        if !self.value.contains_key(&key) {
            return Err(PyErr::new::<exceptions::KeyError, _>("Key not found!"));
        }
        extract_py_object(self.value.get(&key))
    }

    fn keys(&self) -> PyResult<crate::iterators::PyObjectIterator> {
        let mut keys = std::vec::Vec::new();
        for element in self.value.keys() {
            keys.push(extract_py_object(Some(element))?)
        }
        Ok(crate::iterators::PyObjectIterator::new(keys.into_iter()))
    }

    fn values(&self) -> PyResult<crate::iterators::PyObjectIterator> {
        let mut values = std::vec::Vec::new();
        for element in self.value.values() {
            values.push(extract_py_object(Some(element))?)
        }
        Ok(crate::iterators::PyObjectIterator::new(values.into_iter()))
    }

    fn items(&self) -> PyResult<crate::iterators::PyObjectPairIterator> {
        let mut items = std::vec::Vec::new();
        for (key, value) in self.value.iter() {
            items.push((
                extract_py_object(Some(key))?,
                extract_py_object(Some(value))?,
            ))
        }
        Ok(crate::iterators::PyObjectPairIterator::new(
            items.into_iter(),
        ))
    }
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Add the hash of length so that if two collections are added one after the other it doesn't
        // hash to the same thing as a single collection with the same elements in the same order.
        self.value.size().hash(state);
        for (key, value) in self.value.iter() {
            key.hash(state);
            value.hash(state);
        }
    }
}

#[pyproto]
impl PySequenceProtocol for Map {
    fn __len__(&self) -> PyResult<usize> {
        let len = self.value.size();
        Ok(len)
    }

    fn __contains__(&self, py_object: PyObject) -> PyResult<bool> {
        Ok(self.value.contains_key(&Object::new(py_object)))
    }
}

#[pyproto]
impl PyMappingProtocol for Map {
    fn __getitem__(&self, item: PyObject) -> PyResult<PyObject> {
        let key = Object::new(item);
        if !self.value.contains_key(&key) {
            return Err(PyErr::new::<exceptions::KeyError, _>("Key not found!"));
        }
        extract_py_object(self.value.get(&key))
    }
}

py_object_protocol!(Map);
