#![macro_use]

#[macro_export]
macro_rules! py_object_protocol {
    ($struct_:ty) => {
        #[pyproto]
        impl PyObjectProtocol for $struct_ {
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_possible_wrap)]
            fn __hash__(&self) -> PyResult<isize> {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                self.hash(&mut hasher);
                Ok(hasher.finish() as isize)
            }

            fn __richcmp__(
                &self,
                other: &PyAny,
                op: pyo3::class::basic::CompareOp,
            ) -> pyo3::PyResult<bool> {
                let other = other.downcast::<PyCell<$struct_>>();
                if other.is_err() {
                    return match op {
                        pyo3::class::basic::CompareOp::Eq => Ok(false),
                        pyo3::class::basic::CompareOp::Ne => Ok(true),
                        _ => Err(PyErr::new::<exceptions::TypeError, _>(
                            "Invalid comparison operator!".to_string(),
                        )),
                    };
                }
                let other = other?.borrow();

                match op {
                    pyo3::class::basic::CompareOp::Eq => Ok(self.value == other.value),
                    pyo3::class::basic::CompareOp::Ne => Ok(self.value != other.value),
                    _ => Err(PyErr::new::<exceptions::TypeError, _>(
                        "Invalid comparison operator!".to_string(),
                    )),
                }
            }

            fn __repr__(&self) -> pyo3::PyResult<String> {
                Ok(format!("{}", self))
            }
        }
    };
}
