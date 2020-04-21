use std::hash::{Hash, Hasher};

use crate::object::{extract_py_object, Object};
use pyo3::class::basic::CompareOp;
use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pyfunction, pymethods, pyproto, PyModule, PyObject, PyResult};
use pyo3::types::PyTuple;
use pyo3::{
    exceptions, wrap_pyfunction, AsPyRef, ObjectProtocol, PyAny, PyCell, PyErr, PyIterProtocol,
    PyRefMut, Python,
};
use std::panic;

type RpdsVector = rpds::Vector<Object>;

#[pyclass]
struct Vector {
    value: RpdsVector,
}

impl Vector {
    fn new() -> Self {
        Vector {
            value: RpdsVector::new(),
        }
    }
}

#[pymethods]
impl Vector {
    fn set(&self, index: usize, py_object: PyObject) -> PyResult<Self> {
        match self.value.set(index, Object::new(py_object)) {
            Some(value) => Ok(Self { value }),
            None => Err(PyErr::new::<exceptions::IndexError, _>(
                "Index out of bounds!",
            )),
        }
    }

    fn append(&mut self, py_object: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self.value.push_back(Object::new(py_object)),
        };
        Ok(new_self)
    }

    fn extend(&mut self, iterator: PyObject) -> PyResult<Self> {
        let gil_guard = Python::acquire_gil();
        let py = gil_guard.python();

        let iterator = iterator.as_ref(py).iter().unwrap();

        let mut new_self = Self {
            value: self.value.clone(),
        };
        for element in iterator {
            let element = element.unwrap().extract::<PyObject>()?;
            let object = Object::new(element);
            new_self = Self {
                value: new_self.value.push_back(object),
            };
        }
        Ok(new_self)
    }

    fn get(&self, index: usize) -> PyResult<PyObject> {
        if index >= self.value.len() {
            return Err(PyErr::new::<exceptions::IndexError, _>(
                "Index out of bounds!",
            ));
        }

        extract_py_object(self.value.get(index))
    }
}

impl Hash for Vector {
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
impl PySequenceProtocol for Vector {
    fn __len__(&self) -> PyResult<usize> {
        let len = self.value.len();
        Ok(len)
    }

    fn __getitem__(&self, index: isize) -> PyResult<PyObject> {
        self.get(index as usize)
    }
}

#[pyproto]
impl PyIterProtocol for Vector {
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

py_object_protocol!(Vector);

#[pyfunction(args = "*")]
fn pvector(args: &PyTuple) -> PyResult<Vector> {
    let mut vector = Vector::new();
    if args.is_empty() {
        return Ok(vector);
    } else if args.len() > 1 {
        return Err(PyErr::new::<exceptions::ValueError, _>(
            "Incorrect number of arguments!!",
        ));
    }

    let iterator = args.get_item(0).as_ref().iter().unwrap();
    for element in iterator {
        let element = element.unwrap().extract::<PyObject>()?;
        vector = vector.append(element)?;
    }
    Ok(vector)
}

#[pyfunction(args = "*")]
fn v(args: &PyTuple) -> PyResult<Vector> {
    let mut vector = Vector::new();

    for element in args.iter() {
        let element = element.extract::<PyObject>()?;
        vector = vector.append(element)?;
    }
    Ok(vector)
}

pub fn py_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vector>()?;
    m.add_wrapped(wrap_pyfunction!(pvector)).unwrap();
    m.add_wrapped(wrap_pyfunction!(v)).unwrap();

    Ok(())
}
