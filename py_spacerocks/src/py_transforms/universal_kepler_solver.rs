use pyo3::prelude::*;

use spacerocks::transforms::universal_kepler_solver::solve_for_universal_anomaly;


#[pyfunction]
#[pyo3(name = "solve_for_universal_anomaly")]
pub fn solve_for_universal_anomaly_py(r: f64, vr: f64, alpha: f64, mu: f64, dt: f64, tol: f64, max_iter: usize) -> PyResult<f64> {
    match solve_for_universal_anomaly(r, vr, alpha, mu, dt, tol, max_iter) {
        Ok(result) => Ok(result),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e))),
    }
}



// pub fn solve_for_universal_anomaly(r0: f64, vr0: f64, alpha: f64, mu: f64, dt: f64, tol: f64, max_iter: usize) -> f64 {
