use pyo3::prelude::*;

pub mod calc_mean_anomaly_from_conic_anomaly;
pub use self::calc_mean_anomaly_from_conic_anomaly::calc_mean_anomaly_from_conic_anomaly_py;

pub mod calc_conic_anomaly_from_mean_anomaly;
pub use self::calc_conic_anomaly_from_mean_anomaly::calc_conic_anomaly_from_mean_anomaly_py;

pub mod calc_conic_anomaly_from_true_anomaly;
pub use self::calc_conic_anomaly_from_true_anomaly::calc_conic_anomaly_from_true_anomaly_py;

pub mod calc_true_anomaly_from_mean_anomaly;
pub use self::calc_true_anomaly_from_mean_anomaly::calc_true_anomaly_from_mean_anomaly_py;

pub mod stumpff;
pub use self::stumpff::{stumpff_c_py, stumpff_s_py};

pub mod universal_kepler_solver;


// pub mod calc_true_anomaly_from_conic_anomaly;
// pub use self::calc_true_anomaly_from_conic_anomaly::calc_true_anomaly_from_conic_anomaly_py;

pub fn make_transforms_submodule(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add the `transforms` submodule
    let submodule = PyModule::new(m.py(), "transforms")?;


    submodule.add_function(wrap_pyfunction!(calc_mean_anomaly_from_conic_anomaly_py, submodule.clone())?)?;
    submodule.add_function(wrap_pyfunction!(calc_conic_anomaly_from_mean_anomaly_py, submodule.clone())?)?;
    submodule.add_function(wrap_pyfunction!(calc_conic_anomaly_from_true_anomaly_py, submodule.clone())?)?;
    submodule.add_function(wrap_pyfunction!(calc_true_anomaly_from_mean_anomaly_py, submodule.clone())?)?;

    submodule.add_function(wrap_pyfunction!(stumpff_c_py, submodule.clone())?)?;
    submodule.add_function(wrap_pyfunction!(stumpff_s_py, submodule.clone())?)?;
    submodule.add_function(wrap_pyfunction!(universal_kepler_solver::solve_for_universal_anomaly_py, submodule.clone())?)?;


    m.add_submodule(&submodule)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("spacerocks.transforms", submodule.clone())?;
    submodule.setattr("__name__", "spacerocks.transforms")?;
    Ok(())
}

    

// pub mod calc_E_from_M;
// pub use self::calc_E_from_M::calc_E_from_M_py;

// pub mod calc_E_from_f;
// pub use self::calc_E_from_f::calc_E_from_f_py;

// pub mod calc_M_from_E;
// pub use self::calc_M_from_E::calc_M_from_E_py;

// pub mod calc_f_from_E;
// pub use self::calc_f_from_E::calc_f_from_E_py;

// pub fn make_transforms_submodule(py: Python, m: &PyModule) -> PyResult<()> {
//     // Add the `transforms` submodule
//     let submodule = PyModule::new(py, "transforms")?;

//     submodule.add_function(wrap_pyfunction!(calc_E_from_M_py, submodule)?)?;
//     submodule.add_function(wrap_pyfunction!(calc_E_from_f_py, submodule)?)?;
//     submodule.add_function(wrap_pyfunction!(calc_M_from_E_py, submodule)?)?;
//     submodule.add_function(wrap_pyfunction!(calc_f_from_E_py, submodule)?)?;

//     m.add_submodule(submodule)?;
//     py.import("sys")?
//         .getattr("modules")?
//         .set_item("spacerocks.transforms", submodule)?;
//     submodule.setattr("__name__", "spacerocks.transforms")?;
//     Ok(())
// }