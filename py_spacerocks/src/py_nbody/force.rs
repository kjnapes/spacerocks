use pyo3::prelude::*;
use pyo3::types::PyType;

use spacerocks::nbody::forces::{Force, NewtonianGravity, SolarGR};

#[pyclass]
#[pyo3(name = "Force")]
pub struct PyForce {
    pub inner: Box<dyn Force + Send + Sync>,
}

#[pymethods]
impl PyForce {

    #[classmethod]
    pub fn newtonian_gravity(_cls: Py<PyType>) -> PyResult<Self> {
        Ok(PyForce { inner: Box::new(NewtonianGravity) })
    }

    #[classmethod]
    pub fn solar_gr(_cls: Py<PyType>) -> PyResult<Self> {
        Ok(PyForce { inner: Box::new(SolarGR) })
    }


}