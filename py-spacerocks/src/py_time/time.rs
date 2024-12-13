use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::exceptions::PyValueError;

use spacerocks::time::Time;

#[pyclass]
#[pyo3(name = "Time")]
#[derive(Clone)]
pub struct PyTime {
    pub inner: Time,
}

#[pymethods]
impl PyTime {

    #[new]
    fn new(epoch: f64, timescale: &str, format: &str) -> PyResult<Self> {
        match Time::new(epoch, timescale, format) {
            Ok(time) => Ok(PyTime { inner: time }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[classmethod]
    fn now(cls: Py<PyType>) -> PyResult<Self> {
        Ok(PyTime { inner: Time::now() })
    }

    #[classmethod]
    fn from_fuzzy_str(cls: Py<PyType>, s: &str) -> PyResult<Self> {
        match Time::from_fuzzy_str(s) {
            Ok(time) => Ok(PyTime { inner: time }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[classmethod]
    fn infer_time_format(cls: Py<PyType>, epoch: f64, timescale: Option<&str>) -> PyResult<Self> {
        match Time::infer_time_format(epoch, timescale) {
            Ok(time) => Ok(PyTime { inner: time }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    // Timescale conversion methods

    fn utc(&mut self) -> PyResult<()> {
        self.inner.utc();
        Ok(())
    }

    fn tdb(&mut self) -> PyResult<()> {
        self.inner.tdb();
        Ok(())
    }
    
    fn tt(&mut self) -> PyResult<()> {
        self.inner.tt();
        Ok(())
    }
    
    fn tai(&mut self) -> PyResult<()> {
        self.inner.tai();
        Ok(())
    }

     // Format getters

     fn jd(&self) -> f64 {
        self.inner.jd()
    }

    fn mjd(&self) -> f64 {
        self.inner.mjd()
    }

    fn calendar(&self) -> String {
        self.inner.calendar()
    }

    // Attribute getters

    #[getter]
    fn epoch(&self) -> f64 {
        self.inner.epoch
    }

    #[getter]
    fn timescale(&self) -> &str {
        // &self.inner.timescale
        match self.inner.timescale {
            spacerocks::time::TimeScale::TDB => "TDB",
            spacerocks::time::TimeScale::UTC => "UTC",
            spacerocks::time::TimeScale::TT => "TT",
            spacerocks::time::TimeScale::TAI => "TAI",
        }
    }

    #[getter]
    fn format(&self) -> &str {
        // &self.inner.format
        match self.inner.format {
            spacerocks::time::TimeFormat::JD => "JD",
            spacerocks::time::TimeFormat::MJD => "MJD",
        }
    }

    // define __add__ and __sub__ here
    fn __add__(&self, dt: f64) -> PyTime {
        PyTime { inner: self.inner.clone() + dt }
    }

    fn __sub__(&self, dt: f64) -> PyTime {
        PyTime { inner: self.inner.clone() - dt }
    }

    fn __repr__(&self) -> String {
        format!("Time: {} {:?} {:?}", self.inner.epoch, self.inner.timescale, self.inner.format)
    }

    
}


