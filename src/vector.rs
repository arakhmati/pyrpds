use std::borrow::Borrow;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pyfunction, pymethods, pyproto, PyModule, PyObject, PyResult};
use pyo3::types::PyTuple;
use pyo3::{
    exceptions, wrap_pyfunction, AsPyRef, ObjectProtocol, PyAny, PyCell, PyErr, PyIterProtocol,
    PyRefMut, Python,
};

use crate::object::{extract_py_object, Object};

type RpdsVector = rpds::Vector<Object>;

#[pyclass]
#[derive(Default)]
pub struct Vector {
    value: RpdsVector,
}

impl Vector {
    #[must_use]
    pub fn new() -> Self {
        Vector {
            value: RpdsVector::new(),
        }
    }

    fn normalize_index(&self, index: isize) -> PyResult<usize> {
        if index == 0 {
            return Ok(0);
        }

        let length = isize::try_from(self.value.len())?;

        let mut index = index;
        if index < 0 {
            index += length;
        }

        if index < 0 {
            return Err(PyErr::new::<exceptions::IndexError, _>(format!(
                "Index out of range: {}",
                index
            )));
        }
        Ok(usize::try_from(index)?)
    }
}

#[pymethods]
impl Vector {
    pub fn set(&self, index: isize, py_object: PyObject) -> PyResult<Self> {
        let index = self.normalize_index(index)?;

        let object = Object::new(py_object);
        let new_value = if index == self.value.len() {
            Some(self.value.push_back(object))
        } else {
            self.value.set(index, object)
        };

        match new_value {
            Some(value) => Ok(Self { value }),
            None => Err(PyErr::new::<exceptions::IndexError, _>(format!(
                "Index out of range: {}",
                index
            ))),
        }
    }

    pub fn append(&mut self, py_object: PyObject) -> PyResult<Self> {
        let new_self = Self {
            value: self.value.push_back(Object::new(py_object)),
        };
        Ok(new_self)
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn extend(&mut self, iterator: PyObject) -> PyResult<Self> {
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

    pub fn get(&self, index: isize) -> PyResult<PyObject> {
        let index = self.normalize_index(index)?;

        if index >= self.value.len() {
            return Err(PyErr::new::<exceptions::IndexError, _>(format!(
                "Index out of range: {}",
                index
            )));
        }

        extract_py_object(self.value.get(index))
    }

    pub fn index(&self, py_object: PyObject) -> PyResult<usize> {
        let object = Object::new(py_object);

        for (index, element) in self.value.iter().enumerate() {
            let element = element.clone();
            if object == element {
                return Ok(index);
            }
        }

        Err(PyErr::new::<exceptions::ValueError, _>(
            "Element not in vector!",
        ))
    }

    pub fn count(&self, py_object: PyObject) -> PyResult<usize> {
        let object = Object::new(py_object);

        let mut count = 0;
        for element in self.value.iter() {
            let element = element.clone();
            if object == element {
                count += 1;
            }
        }

        Ok(count)
    }

    pub fn remove(&self, py_object: PyObject) -> PyResult<Self> {
        let object = Object::new(py_object);

        let mut vector = Vector::new();
        let mut removed_once = false;
        for element in self.value.iter() {
            let element = element.clone();
            if object != element || removed_once {
                let element = extract_py_object(Some(element.borrow()))?;
                vector = vector.append(element)?;
            } else {
                removed_once = true;
            }
        }

        if vector.value.len() == self.value.len() {
            return Err(PyErr::new::<exceptions::ValueError, _>(
                "Element not in vector!",
            ));
        }
        Ok(vector)
    }

    #[args(args = "*")]
    pub fn mset(&self, args: &PyTuple) -> PyResult<Vector> {
        let mut vector = Self {
            value: self.value.clone(),
        };

        let mut arg_index = 0;
        loop {
            if arg_index >= args.len() {
                break;
            }
            if arg_index + 1 >= args.len() {
                return Err(PyErr::new::<exceptions::TypeError, _>(
                    "Not enough arguments!",
                ));
            }
            let index = args.get_item(arg_index).extract::<isize>()?;
            let element = args.get_item(arg_index + 1).extract::<PyObject>()?;

            vector = vector.set(index, element)?;
            arg_index += 2;
        }
        Ok(vector)
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
        /* pyo3 normalizes the index,
           therefore if negative index is encountered,
           it's actually out of bounds
        */
        if index < 0 {
            let original_index = index - isize::try_from(self.value.len())?;
            return Err(PyErr::new::<exceptions::IndexError, _>(format!(
                "Index out of range: {}",
                original_index
            )));
        }
        self.get(index)
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

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pvector([")?;

        let length = self.value.len();
        for (index, element) in self.value.iter().enumerate() {
            write!(f, "{}", element)?;
            if index != length - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "])")
    }
}

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
