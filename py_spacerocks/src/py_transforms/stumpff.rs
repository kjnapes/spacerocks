use pyo3::prelude::*;

use spacerocks::transforms::stumpff::{stumpff_c, stumpff_s};

// #[pyfunction]
// #[pyo3(name = "calc_true_anomaly_from_mean_anomaly")]
// pub fn calc_true_anomaly_from_mean_anomaly_py(e: f64, mean_anomaly: f64) -> PyResult<f64> {
//     match calc_true_anomaly_from_mean_anomaly(e, mean_anomaly) {
//         Ok(result) => Ok(result),
//         Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e))),
//     }
// }

#[pyfunction]
#[pyo3(name = "stumpff_c")]
pub fn stumpff_c_py(z: f64) -> f64 {
    stumpff_c(z)
}

#[pyfunction]
#[pyo3(name = "stumpff_s")]
pub fn stumpff_s_py(z: f64) -> f64 {
    stumpff_s(z)
}