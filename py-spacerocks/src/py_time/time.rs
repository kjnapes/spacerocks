use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::exceptions::PyValueError;

use spacerocks::time::Time;
use spacerocks::time::TimeScale;

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
    fn now(_cls: Py<PyType>) -> PyResult<Self> {
        Ok(PyTime { inner: Time::now() })
    }

    #[classmethod]
    fn from_fuzzy_str(_cls: Py<PyType>, s: &str) -> PyResult<Self> {
        match Time::from_fuzzy_str(s) {
            Ok(time) => Ok(PyTime { inner: time }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[classmethod]
    fn infer_time_format(_cls: Py<PyType>, epoch: f64, timescale: Option<&str>) -> PyResult<Self> {
        match Time::infer_time_format(epoch, timescale) {
            Ok(time) => Ok(PyTime { inner: time }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    // Methods that return new objects with converted timescale

    fn utc(&self) -> PyTime {
        PyTime { inner: self.inner.utc() }
    }

    fn tdb(&self) -> PyTime {
        PyTime { inner: self.inner.tdb() }
    }
    
    fn tt(&self) -> PyTime {
        PyTime { inner: self.inner.tt() }
    }
    
    fn tai(&self) -> PyTime {
        PyTime { inner: self.inner.tai() }
    }

    // Timescale conversion methods

    fn to_utc(&mut self) -> PyResult<()> {
        self.inner.to_utc();
        Ok(())
    }

    fn to_tdb(&mut self) -> PyResult<()> {
        self.inner.to_tdb();
        Ok(())
    }
    
    fn to_tt(&mut self) -> PyResult<()> {
        self.inner.to_tt();
        Ok(())
    }
    
    fn to_tai(&mut self) -> PyResult<()> {
        self.inner.to_tai();
        Ok(())
    }


    pub fn change_timescale(&mut self, timescale: &str) -> PyResult<()> {
        match timescale.to_lowercase().as_str() {
            "utc" => {
                self.to_utc()?;
            },
            "tdb" => {
                self.to_tdb()?;
            },
            "tt" => {
                self.to_tt()?;
            },
            "tai" => {
                self.to_tai()?;
            },
            _ => return Err(PyValueError::new_err(format!("Invalid timescale: {}", timescale))),
        }
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


