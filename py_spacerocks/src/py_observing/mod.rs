use pyo3::prelude::*;

pub mod observatory;
pub mod observer;
pub mod observation;

pub fn make_observing_submodule(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new(py, "observing")?;

    submodule.add_class::<observatory::PyObservatory>()?;
    submodule.add_class::<observer::PyObserver>()?;
    submodule.add_class::<observation::PyObservation>()?;

    m.add_submodule(&submodule)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("spacerocks.observing", submodule.clone())?;
    submodule.setattr("__name__", "spacerocks.observing")?;
    Ok(())
}