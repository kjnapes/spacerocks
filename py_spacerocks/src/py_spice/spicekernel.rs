use pyo3::prelude::*;
use spacerocks::spice::SpiceKernel;
use pyo3::exceptions::PyValueError;

#[pyclass]
#[pyo3(name = "SpiceKernel")]
pub struct PySpiceKernel {
    pub inner: SpiceKernel,
}

#[pymethods]
impl PySpiceKernel {
    #[new]
    #[pyo3(signature = (config = None))]
    fn new(config: Option<String>) -> PyResult<Self> {
        match config {
            Some(path) => {
                SpiceKernel::from_config(&path)
                    .map(|kernel| PySpiceKernel { inner: kernel })
                    .map_err(|e| PyValueError::new_err(e))
            },
            None => {
                Ok(PySpiceKernel { 
                    inner: SpiceKernel::new()
                })
            }
        }
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