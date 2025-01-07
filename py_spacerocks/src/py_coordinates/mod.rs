use pyo3::prelude::*;

pub mod origin;
pub mod reference_plane;

use crate::py_coordinates::origin::PyOrigin;
use crate::py_coordinates::reference_plane::PyReferencePlane;


pub fn make_coordinates_submodule(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new(py, "coordinates")?;

    submodule.add_class::<PyOrigin>()?;
    submodule.add_class::<PyReferencePlane>()?;

    m.add_submodule(&submodule)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("spacerocks.coordinates", submodule.clone())?;
    submodule.setattr("__name__", "spacerocks.coordinates")?;
    Ok(())
}