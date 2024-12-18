use pyo3::prelude::*;
use pyo3::types::PyType;

use spacerocks::SpaceRock;

use nalgebra::Vector3;

use crate::py_time::time::PyTime;
use crate::py_coordinates::origin::PyOrigin;
use crate::py_observing::observer::{PyObserver};
use crate::py_observing::observation::{PyObservation};


#[pyclass]
#[pyo3(name = "SpaceRock")]
pub struct PySpaceRock {
    pub inner: SpaceRock,
}

#[pymethods]
impl PySpaceRock {

    #[classmethod]
    #[pyo3(signature = (name, epoch, reference_plane="ECLIPJ2000", origin="SSB"))]
    fn from_horizons(_cls: Py<PyType>, name: &str, epoch: PyRef<PyTime>, reference_plane: &str, origin: &str) -> PyResult<Self> {
        let rock = SpaceRock::from_horizons(name, &epoch.inner, reference_plane, origin);
        if rock.is_err() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to create SpaceRock from Horizons for name: {}", name)));
        }
        Ok(PySpaceRock { inner: rock.unwrap() })
    }

    #[classmethod]
    #[pyo3(signature = (name, epoch, reference_plane="ECLIPJ2000", origin="SSB"))]
    fn from_spice(_cls: Py<PyType>, name: &str, epoch: PyRef<PyTime>, reference_plane: &str, origin: &str) -> PyResult<Self> {
        let rock = SpaceRock::from_spice(name, &epoch.inner, reference_plane, origin);
        if rock.is_err() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to create SpaceRock from Spice for name: {}", name)));
        }
        Ok(PySpaceRock { inner: rock.unwrap() })
    }

    #[classmethod]
    fn from_xyz(_cls: Py<PyType>, name: &str, x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64, epoch: PyRef<PyTime>, reference_plane: &str, origin: &str) -> PyResult<Self> {
        let rock = SpaceRock::from_xyz(name, x, y, z, vx, vy, vz, epoch.inner.clone(), reference_plane, origin);
        if rock.is_err() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to create SpaceRock from XYZ for name: {}", name)));
        }
        Ok(PySpaceRock { inner: rock.unwrap() })
    }


    fn observe(&mut self, observer: &PyObserver) -> PyResult<PyObservation> {
        // if observer.inner.frame != ReferencePlane::J2000 {
        //     return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Observer frame is not J2000. Cannot observe rocks.")));
        // }

        match self.inner.observe(&observer.inner) {
            Ok(obs) => Ok(PyObservation { inner: obs }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to observe rock: {}", e))),
        }
    }

    
    fn __repr__(&self) -> String {
        format!("SpaceRock: {}", self.inner.name)
    }

    #[getter]
    fn epoch(&self) -> PyTime {
        PyTime { inner: self.inner.epoch.clone() }
    }

    #[getter]
    fn reference_plane(&self) -> String {
        self.inner.reference_plane.to_string()
    }

    #[getter]
    fn origin(&self) -> PyOrigin {  
        PyOrigin { inner: self.inner.origin.clone() }
    }

    #[getter]
    fn r(&self) -> f64 {
        self.inner.r()
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name.to_string()
    }

    #[setter]
    fn set_name(&mut self, name: &str) -> PyResult<()> {
        self.inner.name = name.to_string().into();
        Ok(())
    }

    #[getter]
    fn position(&self) -> (f64, f64, f64) {
        (self.inner.position.x, self.inner.position.y, self.inner.position.z)
    }

    #[setter]
    fn set_position(&mut self, pos: (f64, f64, f64)) -> PyResult<()> {
        self.inner.position = Vector3::new(pos.0, pos.1, pos.2);
        Ok(())
    }

    #[getter]
    fn velocity(&self) -> (f64, f64, f64) {
        (self.inner.velocity.x, self.inner.velocity.y, self.inner.velocity.z)
    }

    #[setter]
    fn set_velocity(&mut self, vel: (f64, f64, f64)) -> PyResult<()> {
        self.inner.velocity = Vector3::new(vel.0, vel.1, vel.2);
        Ok(())
    }

    #[getter]
    fn x(&self) -> f64 {
        self.inner.position.x
    }

    #[setter]
    fn set_x(&mut self, x: f64) -> PyResult<()> {
        self.inner.position.x = x;
        Ok(())
    }

    #[getter]
    fn y(&self) -> f64 {
        self.inner.position.y
    }

    #[setter]
    fn set_y(&mut self, y: f64) -> PyResult<()> {
        self.inner.position.y = y;
        Ok(())
    }

    #[getter]
    fn z(&self) -> f64 {
        self.inner.position.z
    }

    #[setter]
    fn set_z(&mut self, z: f64) -> PyResult<()> {
        self.inner.position.z = z;
        Ok(())
    }

    #[getter]
    fn vx(&self) -> f64 {
        self.inner.velocity.x
    }

    #[setter]
    fn set_vx(&mut self, vx: f64) -> PyResult<()> {
        self.inner.velocity.x = vx;
        Ok(())
    }

    #[getter]
    fn vy(&self) -> f64 {
        self.inner.velocity.y
    }

    #[setter]
    fn set_vy(&mut self, vy: f64) -> PyResult<()> {
        self.inner.velocity.y = vy;
        Ok(())
    }

    #[getter]
    fn vz(&self) -> f64 {
        self.inner.velocity.z
    }

    #[setter]
    fn set_vz(&mut self, vz: f64) -> PyResult<()> {
        self.inner.velocity.z = vz;
        Ok(())
    }

    #[getter]
    fn mass(&self) -> f64 {
        self.inner.mass()
    }

    #[setter]
    fn set_mass(&mut self, mass: f64) -> PyResult<()> {
        self.inner.set_mass(mass);
        Ok(())
    }

    #[getter]
    fn evec(&self) -> (f64, f64, f64) {
        let e = self.inner.evec();
        (e.x, e.y, e.z)
    }

    #[getter]
    fn mu(&self) -> f64 {
        self.inner.origin.mu()
    }

    pub fn a(&self) -> f64 {
        self.inner.a()
    }

}

