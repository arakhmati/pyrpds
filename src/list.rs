use pyo3::class::basic::CompareOp;
use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pymethods, pyproto, PyObject, PyResult};
use pyo3::{PyAny, PyCell};
use rpds::List;

#[pyclass(name = List)]
pub struct PyList {
    list: List<PyObject>,
}

#[pymethods]
impl PyList {
    #[new]
    fn new() -> Self {
        PyList { list: List::new() }
    }

    fn push_front(&mut self, object: PyObject) -> PyResult<Self> {
        let py_list = Self {
            list: self.list.push_front(object),
        };
        Ok(py_list)
    }

    fn drop_first(&mut self) -> PyResult<Self> {
        let list = match self.list.drop_first() {
            Some(list) => list,
            None => panic!("drop_first failed!"),
        };
        let py_list = Self { list };
        Ok(py_list)
    }

    fn reverse(&self) -> PyResult<Self> {
        let reversed = Self {
            list: self.list.reverse(),
        };
        Ok(reversed)
    }

    fn first(&self) -> PyResult<Option<&PyObject>> {
        let first = self.list.first();
        Ok(first)
    }

    fn last(&self) -> PyResult<Option<&PyObject>> {
        let last = self.list.last();
        Ok(last)
    }
}

#[pyproto]
impl PySequenceProtocol for PyList {
    fn __len__(&self) -> PyResult<usize> {
        let len = self.list.len();
        Ok(len)
    }
}

#[pyproto]
impl PyObjectProtocol for PyList {
    fn __hash__(&self) -> PyResult<isize> {
        Ok(0)
    }

    fn __richcmp__(&self, other_as_any: &PyAny, op: CompareOp) -> PyResult<bool> {
        let other_as_cell = other_as_any.downcast::<PyCell<PyList>>()?;
        let other = other_as_cell.borrow();

        match op {
            CompareOp::Eq => Ok(self.list == other.list),
            CompareOp::Ne => Ok(self.list != other.list),
            _ => panic!("Invalid CompareOp"),
        }
    }
}
