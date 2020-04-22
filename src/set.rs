use std::hash::{Hash, Hasher};

use crate::object::{extract_py_object, Object};
use pyo3::class::basic::CompareOp;
use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pyfunction, pymethods, pyproto, PyModule, PyObject, PyResult};
use pyo3::types::PyTuple;
use pyo3::{
    exceptions, wrap_pyfunction, ObjectProtocol, PyAny, PyCell, PyErr, PyIterProtocol, PyRefMut,
    Python,
};

type RpdsSet = rpds::HashTrieSet<Object>;

#[pyclass]
#[derive(Default)]
pub struct Set {
    value: RpdsSet,
}

impl Set {
    #[must_use]
    pub fn new() -> Self {
        Set {
            value: RpdsSet::new(),
        }
    }
}

#[pymethods]
impl Set {
    pub fn add(&mut self, py_object: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self.value.insert(Object::new(py_object)),
        };
        Ok(new_self)
    }

    pub fn remove(&mut self, py_object: PyObject) -> PyResult<Self> {
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

#[pyfunction(args = "*")]
fn pset(args: &PyTuple) -> PyResult<Set> {
    let mut set = Set::new();
    if args.is_empty() {
        return Ok(set);
    } else if args.len() > 1 {
        return Err(PyErr::new::<exceptions::ValueError, _>(
            "Incorrect number of arguments!!",
        ));
    }

    let iterator = args.get_item(0).as_ref().iter().unwrap();
    for element in iterator {
        let element = element.unwrap().extract::<PyObject>()?;
        set = set.add(element)?;
    }
    Ok(set)
}

#[pyfunction(args = "*")]
fn s(args: &PyTuple) -> PyResult<Set> {
    let mut set = Set::new();

    for element in args.iter() {
        let element = element.extract::<PyObject>()?;
        set = set.add(element)?;
    }
    Ok(set)
}

pub fn py_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Set>()?;
    m.add_wrapped(wrap_pyfunction!(pset)).unwrap();
    m.add_wrapped(wrap_pyfunction!(s)).unwrap();

    Ok(())
}
