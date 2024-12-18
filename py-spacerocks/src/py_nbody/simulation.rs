use pyo3::prelude::*;
use pyo3::types::PyType;

use spacerocks::nbody::Simulation;
use spacerocks::coordinates::{ReferencePlane, Origin};

use crate::PySpaceRock;
// use crate::py_spacerock::rockcollection::RockCollection;
use crate::py_time::time::PyTime;
use crate::py_nbody::integrator::PyIntegrator;
use crate::py_nbody::force::PyForce;
use crate::py_coordinates::origin::PyOrigin;

#[pyclass]
#[pyo3(name = "Simulation")]
pub struct PySimulation {
    pub inner: Simulation,
}

#[pymethods]
impl PySimulation {

    #[new]
    pub fn new() -> Self {
        PySimulation { inner: Simulation::new() }
    }

    #[classmethod]
    pub fn giants(_cls: Py<PyType>, epoch: &PyTime, reference_plane: &str, origin: &str) -> PyResult<Self> {
        let ep = &epoch.inner;
        let sim = Simulation::giants(ep, reference_plane, origin);
        Ok(PySimulation { inner: sim.unwrap() })
    }

    #[classmethod]
    pub fn planets(_cls: Py<PyType>, epoch: &PyTime, reference_plane: &str, origin: &str) -> PyResult<Self> {
        let ep = &epoch.inner;
        let sim = Simulation::planets(ep, reference_plane, origin);
        Ok(PySimulation { inner: sim.unwrap() })
    }

    #[classmethod]
    pub fn horizons(_cls: Py<PyType>, epoch: &PyTime, reference_plane: &str, origin: &str) -> PyResult<Self> {
        let ep = &epoch.inner;
        let sim = Simulation::horizons(ep, reference_plane, origin);
        Ok(PySimulation { inner: sim.unwrap() })
    }

    // #[classmethod]
    // pub fn planets(_cls: &PyType, epoch: &PyTime, frame: &str, origin: &PyOrigin) -> PyResult<Self> {
    //     let ep = &epoch.inner;
    //     let or = &origin.inner;

    //     let frame = CoordinateFrame::from_str(frame).unwrap();
    //     let sim = Simulation::planets(ep, &frame, or);
    //     Ok(PySimulation { inner: sim.unwrap() })
    // }

    // #[classmethod]
    // pub fn horizons(_cls: &PyType, epoch: &PyTime, frame: &str, origin: &PyOrigin) -> PyResult<Self> {
    //     let ep = &epoch.inner;
    //     let or = &origin.inner;

    //     let frame = CoordinateFrame::from_str(frame).unwrap();
    //     let sim = Simulation::horizons(ep, &frame, or);
    //     Ok(PySimulation { inner: sim.unwrap() })
    // }

    pub fn add(&mut self, rock: &PySpaceRock) -> PyResult<()> {
        match self.inner.add(rock.inner.clone()) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }
    }

    pub fn remove(&mut self, name: &str) -> PyResult<()> {
        match self.inner.remove(name) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }
    }

    pub fn integrate(&mut self, epoch: &PyTime) {
        self.inner.integrate(&epoch.inner.clone());
    }
    
    pub fn step(&mut self) {
        self.inner.step();
    }

    pub fn move_to_center_of_mass(&mut self) -> PyResult<()> {
        match self.inner.move_to_center_of_mass() {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }
    }

    pub fn change_origin(&mut self, origin: &str) -> PyResult<()> {
        match self.inner.change_origin(origin) {
            Ok(_) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }
    }

    pub fn set_epoch(&mut self, epoch: &PyTime) {
        self.inner.epoch = epoch.inner.clone();
    }

    pub fn set_reference_plane(&mut self, reference_plane: &str) {
        let reference_plane = ReferencePlane::from_str(reference_plane).unwrap();
        self.inner.reference_plane = reference_plane;
    }

    pub fn set_origin(&mut self, origin: &str) -> PyResult<()> {
        match Origin::from_str(origin) {
            Ok(o) => {
                self.inner.origin = o;
                Ok(())
            },
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }
    }

    pub fn set_integrator(&mut self, integrator: PyRef<PyIntegrator>) {
        self.inner.integrator = integrator.inner.clone();
    }

    pub fn add_force(&mut self, force: PyRef<PyForce>) {
        self.inner.add_force(force.inner.clone());
    }

    pub fn energy(&self) -> f64 {
        self.inner.energy()
    }

    pub fn get_particle(&self, name: &str) -> PyResult<PySpaceRock> {
        let rock = self.inner.get_particle(name);
        match rock {
            Ok(r) => Ok(PySpaceRock { inner: r.clone() }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }
       
    }


    // #[getter]
    // pub fn particles(&self) -> RockCollection {
    //     RockCollection { rocks: self.inner.particles.clone(), name_hash_map: self.inner.particle_index_map.clone() }
    // }


    #[getter]
    pub fn epoch(&self) -> PyTime {
        PyTime { inner: self.inner.epoch.clone() }
    }

    #[getter]
    pub fn reference_plane(&self) -> String {
        self.inner.reference_plane.to_string()
    }


    #[getter]
    pub fn timestep(&self) -> f64 {
        self.inner.integrator.timestep()
    }

    #[getter]
    pub fn origin(&self) -> PyOrigin {
        let o = self.inner.origin.clone();
        PyOrigin { inner: o }
    }

    // #[getter]
    // pub fn integrator(&self) -> PyIntegrator {
    //     PyIntegrator { inner: self.inner.integrator.clone() }
    // }

}