use pyo3::prelude::*;
use spacerocks::spice::SpiceKernel;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyType;

#[pyclass]
#[pyo3(name = "SpiceKernel")]
pub struct PySpiceKernel {
    pub inner: SpiceKernel,
}

#[pymethods]
impl PySpiceKernel {
    #[new]
    fn new() -> Self {
        PySpiceKernel { 
            inner: SpiceKernel::new()
        }
    }

    #[classmethod]
    #[pyo3(signature = (download = true))]
    fn defaults(cls: Py<PyType>, download: bool) -> PyResult<Self> {
        SpiceKernel::defaults(download)
            .map(|kernel| PySpiceKernel { inner: kernel })
            .map_err(|e| PyValueError::new_err(e))
    }
    #[classmethod]
    fn from_config(cls: Py<PyType>, path: String) -> PyResult<Self> {
        SpiceKernel::from_config(&path)
            .map(|kernel| PySpiceKernel { inner: kernel })
            .map_err(|e| PyValueError::new_err(e))
    }

    fn load(&mut self, path: &str) -> PyResult<()> {
        self.inner.load(path)
            .map_err(|e| PyValueError::new_err(e))
    }
    
    fn unload(&mut self) {
        self.inner.unload();
    }
    
    #[getter]
    fn loaded_kernels(&self) -> Vec<String> {
        self.inner.loaded_kernels().to_vec()
    }
    
    fn __repr__(&self) -> String {
        let mut s = String::from("SpiceKernel:\n");
        for kernel in self.inner.loaded_kernels() {
            s.push_str(&format!("  - {}\n", kernel));
        }
        s
    }
}