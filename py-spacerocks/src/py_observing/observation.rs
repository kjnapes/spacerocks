use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::exceptions::PyValueError;

use spacerocks::Observation;

use crate::py_observing::observer::PyObserver;
use crate::py_time::time::PyTime;
// use crate::py_spacerock::origin::PyOrigin;

#[pyclass]
#[pyo3(name = "Observation")]
pub struct PyObservation {
    pub inner: Observation,
}

#[pymethods]
impl PyObservation {


    #[getter]
    fn ra(&self) -> f64 {
        self.inner.ra()
    }

    #[getter]
    fn dec(&self) -> f64 {
        self.inner.dec()
    }

    #[getter]
    fn ra_rate(&self) -> Option<f64> {
        self.inner.ra_rate()
    }

    #[getter]
    fn dec_rate(&self) -> Option<f64> {
        self.inner.dec_rate()
    }

    #[getter]
    fn range(&self) -> Option<f64> {
        self.inner.range()
    }

    #[getter]
    fn range_rate(&self) -> Option<f64> {
        self.inner.range_rate()
    }

    #[getter]
    fn epoch(&self) -> PyTime {
        PyTime { inner: self.inner.epoch.clone() }
    }

}