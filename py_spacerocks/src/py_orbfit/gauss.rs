use pyo3::prelude::*;

use crate::py_observing::observation::PyObservation;
use crate::PySpaceRock;

use spacerocks::orbfit::gauss;
use spacerocks::observing::Observation;



#[pyfunction]
#[pyo3(name = "gauss")]
pub fn gauss_py(_py: Python, o1: &PyObservation, o2: &PyObservation, o3: &PyObservation, min_distance: f64) -> PyResult<Vec<PySpaceRock>> {
    
    // let o1: Observation = o1.inner.clone();
    // let o2: Observation = o2.inner.clone();
    // let o3: Observation = o3.inner.clone();

    // let rocks = gauss(&o1, &o2, &o3, min_distance);
    let rocks = gauss(&o1.inner, &o2.inner, &o3.inner, min_distance);
    if rocks.is_none() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("No solutions found"))
    }
    
    let rocks = rocks.expect("No solutions found");
    let py_rocks: Vec<PySpaceRock> = rocks.iter().map(|rock| PySpaceRock { inner: rock.clone() }).collect();
    Ok(py_rocks)
}

// #[pyfunction]
// #[pyo3(name = "gauss2")]
// pub fn gauss2_py(py: Python<'_>, input: Vec<PyRef<PyObservation>>) -> PyResult<()> {
//     for obs in input {
//         println!("{:?}", obs.inner);
//     }
//     Ok(())
// }
