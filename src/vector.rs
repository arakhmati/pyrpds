use pyo3::class::basic::CompareOp;
use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};
use pyo3::{PyAny, PyCell};
use rpds::Vector;

#[pyclass(name = Vector)]
pub struct PyVector {
    vector: Vector<PyObject>,
}

#[pymethods]
impl PyVector {
    #[new]
    fn new() -> Self {
        PyVector {
            vector: Vector::new(),
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

#[pyproto]
impl PySequenceProtocol for PyVector {
    fn __len__(&self) -> PyResult<usize> {
        let len = self.vector.len();
        Ok(len)
    }
}

#[pyproto]
impl PyObjectProtocol for PyVector {
    fn __hash__(&self) -> PyResult<isize> {
        Ok(0)
    }

    fn __richcmp__(&self, other_as_any: &PyAny, op: CompareOp) -> PyResult<bool> {
        let other_as_cell = other_as_any.downcast::<PyCell<PyVector>>()?;
        let other = other_as_cell.borrow();

        match op {
            CompareOp::Eq => Ok(self.vector == other.vector),
            CompareOp::Ne => Ok(self.vector != other.vector),
            _ => panic!("Invalid CompareOp"),
        }
    }
}
