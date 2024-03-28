use pyo3::prelude::*;

use spacerocks::observing::Observer;
use spacerocks::spacerock::{CoordinateFrame, Origin}; 

use crate::py_time::time::PyTime;
use crate::py_spacerock::origin::PyOrigin;

use numpy::{PyArray1, IntoPyArray};

#[pyclass]
#[pyo3(name = "Observer")]
#[derive(Clone)]
pub struct PyObserver {
    pub inner: Observer,
}

#[pymethods]
impl PyObserver {

    // fn __repr__(&self) -> String {
    //     format!("Observer: {} at position: {:?}", self.inner.name, self.inner.position)
    // }

    #[getter]
    fn position(&self, py: Python) -> Py<PyArray1<f64>> {
        // self.inner.position.clone().to_vec().into_pyarray(py).to_owned()
        let pos = vec![self.inner.position.x, self.inner.position.y, self.inner.position.z];
        pos.into_pyarray(py).to_owned()
    }

    #[getter]
    fn velocity(&self, py: Python) -> Py<PyArray1<f64>> {
        // self.inner.velocity.clone().to_vec().into_pyarray(py).to_owned()
        let vel = vec![self.inner.velocity.x, self.inner.velocity.y, self.inner.velocity.z];
        vel.into_pyarray(py).to_owned()
    }

    #[getter]
    fn origin(&self) -> PyOrigin {
        PyOrigin { inner: self.inner.origin.clone() }
    }

    fn change_frame(&mut self, new_frame: &str) {
        let frame = CoordinateFrame::from_str(new_frame).unwrap();
        self.inner.change_frame(&frame);
    }

    #[getter]
    fn lat(&self) -> Option<f64> {
        self.inner.lat
    }

    #[getter]
    fn lon(&self) -> Option<f64> {
        self.inner.lon
    }

    #[getter]
    fn rho(&self) -> Option<f64> {
        self.inner.rho
    }

    #[getter]
    fn frame(&self) -> String {
        self.inner.frame.to_string()
    }

    #[getter]
    fn epoch(&self) -> PyTime {
        PyTime { inner: self.inner.epoch.clone() }
    }
    
}