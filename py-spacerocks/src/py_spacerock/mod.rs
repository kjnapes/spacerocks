use pyo3::prelude::*;

pub mod spacerock;
pub mod rockcollection;
pub mod origin;
pub mod reference_plane;

pub fn make_spacerock_submodule(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add the `spacerock` submodule
    let submodule = PyModule::new(py, "spacerock")?;

    submodule.add_class::<spacerock::PySpaceRock>()?;
    // submodule.add_class::<rockcollection::RockCollection>()?;
    // submodule.add_class::<origin::PyOrigin>()?;
    // submodule.add_class::<coordinate_frame::PyReferencePlane>()?;


    m.add_submodule(&submodule)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("spacerocks.spacerock", submodule.clone())?;
    submodule.setattr("__name__", "spacerocks.spacerock")?;
    Ok(())
}