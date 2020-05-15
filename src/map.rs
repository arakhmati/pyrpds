use std::hash::{Hash, Hasher};

use pyo3::class::PyObjectProtocol;
use pyo3::prelude::{pyclass, pyfunction, pymethods, pyproto, PyModule, PyObject, PyResult};
use pyo3::types::{PyDict, PyTuple};
use pyo3::{
    exceptions, wrap_pyfunction, AsPyRef, IntoPy, ObjectProtocol, Py, PyAny, PyCell, PyErr,
    PyIterProtocol, PyMappingProtocol, PyRefMut, PySequenceProtocol, Python, ToPyObject,
};

use crate::object::{extract_py_object, Object};

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
    pub fn set(&self, py_key: PyObject, py_value: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self
                .value
                .insert(Object::new(py_key), Object::new(py_value)),
        };
        Ok(new_self)
    }
    pub fn discard(&self, py_object: PyObject) -> PyResult<Self> {
        let object = Object::new(py_object);

        let new_self = Self {
            value: self.value.remove(&object),
        };

        Ok(new_self)
    }

    pub fn remove(&self, py_key: PyObject) -> PyResult<Self> {
        let key = Object::new(py_key);

        if !self.value.contains_key(&key) {
            return Err(PyErr::new::<exceptions::KeyError, _>(key.to_string()));
        }

        let new_self = Self {
            value: self.value.remove(&key),
        };

        Ok(new_self)
    }

    pub fn get(&self, py_key: PyObject) -> PyResult<PyObject> {
        let key = Object::new(py_key);
        if !self.value.contains_key(&key) {
            return Err(PyErr::new::<exceptions::KeyError, _>(key.to_string()));
        }
        extract_py_object(self.value.get(&key))
    }

    pub fn keys(&self) -> PyResult<crate::vector::Vector> {
        let mut keys = crate::vector::Vector::new();
        for element in self.value.keys() {
            keys = keys.append(extract_py_object(Some(element))?)?;
        }
        Ok(keys)
    }

    pub fn values(&self) -> PyResult<crate::vector::Vector> {
        let mut values = crate::vector::Vector::new();
        for element in self.value.values() {
            values = values.append(extract_py_object(Some(element))?)?;
        }
        Ok(values)
    }

    pub fn itervalues(&self) -> PyResult<crate::vector::Vector> {
        self.values()
    }

    pub fn items(&self) -> PyResult<crate::vector::Vector> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        let mut items = crate::vector::Vector::new();
        for (key, value) in self.value.iter() {
            let element = vec![
                extract_py_object(Some(key))?,
                extract_py_object(Some(value))?,
            ];
            let element = PyTuple::new(py, element);
            let element = Py::from(element);
            items = items.append(element.into_py(py))?;
        }
        Ok(items)
    }

    pub fn iteritems(&self) -> PyResult<crate::vector::Vector> {
        self.items()
    }

    #[allow(clippy::needless_pass_by_value)]
    #[args(args = "*")]
    pub fn update(&self, args: &PyTuple) -> PyResult<Self> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        let mut new_self = Self {
            value: self.value.clone(),
        };

        for arg in args.iter() {
            let object: PyObject = arg.into_py(py);
            let iterator = arg.iter().unwrap();

            for key in iterator {
                let key: PyObject = key.unwrap().into_py(py);
                let value = object.call_method1(py, "__getitem__", (key.clone_ref(py),))?;
                new_self = new_self.set(key, value)?;
            }
        }

        Ok(new_self)
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

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pmap({{")?;

        let length = self.value.size();
        for (index, (key, value)) in self.value.iter().enumerate() {
            write!(f, "{}: {}", key, value)?;
            if index != length - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}})")
    }
}

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
