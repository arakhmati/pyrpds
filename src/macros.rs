#![macro_use]

#[macro_export]
macro_rules! py_object_protocol {
    ($struct_:ident) => {
        #[pyproto]
        impl PyObjectProtocol for $struct_ {
            fn __hash__(&self) -> PyResult<isize> {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                self.hash(&mut hasher);
                Ok(hasher.finish() as isize)
            }

            fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<bool> {
                let other = other.downcast::<PyCell<$struct_>>()?;
                let other = other.borrow();

                match op {
                    CompareOp::Eq => Ok(self.value == other.value),
                    CompareOp::Ne => Ok(self.value != other.value),
                    _ => panic!("Invalid CompareOp"),
                }
            }
        }
    };
}
