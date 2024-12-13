use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod time;


// use pyo3::prelude::*;

// pub fn make_time_submodule(py: Python, m: &PyModule) -> PyResult<()> {
//     // Add the `time` submodule
//     let submodule = PyModule::new(py, "time")?;

//     submodule.add_class::<time::PyTime>()?;

//     m.add_submodule(submodule)?;
//     py.import("sys")?
//         .getattr("modules")?
//         .set_item("spacerocks.time", submodule)?;
//     submodule.setattr("__name__", "spacerocks.time")?;
//     Ok(())
// }

pub fn make_time_submodule(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Create a submodule named "time"
    let submodule = PyModule::new(m.py(), "time")?;
    
    // Register your time::PyTime class with the submodule
    submodule.add_class::<time::PyTime>()?;

    // Add the submodule to the parent module (requires PyO3 ≥ 0.14)
    m.add_submodule(&submodule)?;

    // For a fully importable submodule in Python, register it in sys.modules
    py.import("sys")?
        .getattr("modules")?
        .set_item("spacerocks.time", submodule.clone())?;

    // Update the submodule’s __name__ to reflect the fully qualified name
    submodule.setattr("__name__", "spacerocks.time")?;

    Ok(())
}
