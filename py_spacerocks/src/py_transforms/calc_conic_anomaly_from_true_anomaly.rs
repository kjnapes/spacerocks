use pyo3::prelude::*;

use spacerocks::transforms::calc_conic_anomaly_from_true_anomaly;

#[pyfunction]
#[pyo3(name = "calc_conic_anomaly_from_true_anomaly")]
pub fn calc_conic_anomaly_from_true_anomaly_py(e: f64, true_anomaly: f64) -> PyResult<f64> {
    match calc_conic_anomaly_from_true_anomaly(e, true_anomaly) {
        Ok(result) => Ok(result),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e))),
    }
}