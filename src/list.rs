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

type RpdsList = rpds::List<Object>;

#[pyclass]
struct List {
    value: RpdsList,
}

impl List {
    fn new() -> Self {
        List {
            value: RpdsList::new(),
        }
    }
}

#[pymethods]
impl List {
    fn push_front(&mut self, py_object: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self.value.push_front(Object::new(py_object)),
        };
        Ok(new_self)
    }

    fn reverse(&self) -> PyResult<Self> {
        let reversed = Self {
            value: self.value.reverse(),
        };
        Ok(reversed)
    }

    #[getter]
    fn first(&self) -> PyResult<PyObject> {
        extract_py_object(self.value.first())
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

#[pyproto]
impl PyIterProtocol for List {
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

py_object_protocol!(List);

#[pyfunction(args = "*")]
fn plist(args: &PyTuple) -> PyResult<List> {
    let mut list = List::new();
    if args.is_empty() {
        return Ok(list);
    } else if args.len() > 1 {
        return Err(PyErr::new::<exceptions::ValueError, _>(
            "Incorrect number of arguments!!",
        ));
    }

    let iterator = args.get_item(0).as_ref().iter().unwrap();
    for element in iterator {
        let element = element.unwrap().extract::<PyObject>()?;
        list = list.push_front(element)?;
    }
    Ok(list)
}

#[pyfunction(args = "*")]
fn l(args: &PyTuple) -> PyResult<List> {
    let mut list = List::new();

    for element in args.iter() {
        let element = element.extract::<PyObject>()?;
        list = list.push_front(element)?;
    }
    Ok(list)
}

pub fn py_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<List>()?;
    m.add_wrapped(wrap_pyfunction!(plist)).unwrap();
    m.add_wrapped(wrap_pyfunction!(l)).unwrap();

    Ok(())
}
