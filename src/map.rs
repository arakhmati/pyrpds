use std::hash::{Hash, Hasher};

use crate::object::{extract_py_object, Object};
use pyo3::class::basic::CompareOp;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pyfunction, pymethods, pyproto, PyModule, PyObject, PyResult};
use pyo3::types::{PyDict, PyTuple};
use pyo3::{
    exceptions, wrap_pyfunction, AsPyRef, ObjectProtocol, Py, PyAny, PyCell, PyErr, PyIterProtocol,
    PyMappingProtocol, PyRefMut, PySequenceProtocol, Python, ToPyObject,
};

type RpdsMap = rpds::HashTrieMap<Object, Object>;

#[pyclass]
#[derive(Default)]
pub struct Map {
    value: RpdsMap,
}

impl Map {
    #[must_use]
    pub fn new() -> Self {
        Map {
            value: RpdsMap::new(),
        }
    }
}

#[pymethods]
impl Map {
    fn set(&mut self, py_key: PyObject, py_value: PyObject) -> PyResult<Self> {
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
        self.get(item)
    }
}

#[pyproto]
impl PyIterProtocol for Map {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<crate::iterators::PyObjectIterator> {
        let mut elements = std::vec::Vec::new();
        for element in slf.value.keys() {
            elements.push(extract_py_object(Some(element))?)
        }

        Ok(crate::iterators::PyObjectIterator::new(
            elements.into_iter(),
        ))
    }
}

py_object_protocol!(Map);

#[pyfunction(args = "*")]
fn pmap(args: &PyTuple) -> PyResult<Map> {
    let mut map = Map::new();
    if args.is_empty() {
        return Ok(map);
    } else if args.len() > 1 {
        return Err(PyErr::new::<exceptions::ValueError, _>(
            "Incorrect number of arguments!!",
        ));
    }

    let gil_guard = Python::acquire_gil();
    let py = gil_guard.python();

    let dict = args.get_item(0).extract::<Py<PyDict>>()?;
    let dict = dict.as_ref(py);
    for key_value_pair in dict.items() {
        let key_value_pair = key_value_pair.downcast::<PyTuple>()?;
        let key = key_value_pair.get_item(0).to_object(py);
        let value = key_value_pair.get_item(1).to_object(py);
        map = map.set(key, value)?;
    }
    Ok(map)
}

#[pyfunction(kwargs = "**")]
fn m(kwargs: Option<&PyDict>) -> PyResult<Map> {
    let mut map = Map::new();

    if kwargs == None {
        return Ok(map);
    }
    let kwargs = kwargs.unwrap();

    let gil_guard = Python::acquire_gil();
    let py = gil_guard.python();

    for (key, value) in kwargs.iter() {
        let key = key.to_object(py);
        let value = value.to_object(py);
        map = map.set(key, value)?;
    }
    Ok(map)
}

pub fn py_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Map>()?;
    m.add_wrapped(wrap_pyfunction!(pmap)).unwrap();
    m.add_wrapped(wrap_pyfunction!(m)).unwrap();

    Ok(())
}
