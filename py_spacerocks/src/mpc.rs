use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use flate2::read::GzDecoder;
use reqwest;
use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use pyo3::types::PyDict;
use pyo3::PyObject;

use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct MPCData {
    pub H: Option<f64>, 
    pub G: Option<f64>,
    pub Epoch: f64, 
    pub M: f64,
    pub Peri: f64,
    pub Node: f64,
    pub i: f64,
    pub e: f64,
    pub a: f64,
    pub Principal_desig: String,
}

#[pyclass]
pub struct MPC {
    path: PathBuf,
}

#[pymethods]
impl MPC {
    #[new]
    pub fn new(path: String) -> Self {
        MPC {
            path: PathBuf::from(path),
        }
    }

    /// Fetch and optionally process MPC data
    pub fn fetch_data(&self, catalog: String, save_to_df: bool) -> PyResult<PyObject> {
        let url = format!(
            "https://www.minorplanetcenter.net/Extended_Files/{}.json.gz",
            catalog
        );
    
        let zip_path = self.path.join(format!("{}.json.gz", catalog));
        if !zip_path.exists() {
            println!("Downloading from URL: {}", url);
            let response = reqwest::blocking::get(&url)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let content = response.bytes()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            fs::write(&zip_path, &content)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        }
    
        // Parse JSON into a DataFrame
        let reader = GzDecoder::new(fs::File::open(&zip_path)?);
        let data: Vec<MPCData> = serde_json::from_reader(reader)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
        if save_to_df {
            Python::with_gil(|py| {
                let pd = py.import("pandas")?;
                let dict = PyDict::new(py);
                dict.set_item("H", data.iter().map(|d| d.H).collect::<Vec<_>>())?;
                dict.set_item("G", data.iter().map(|d| d.G).collect::<Vec<_>>())?;
                dict.set_item("Epoch", data.iter().map(|d| d.Epoch).collect::<Vec<_>>())?;
                dict.set_item("M", data.iter().map(|d| d.M).collect::<Vec<_>>())?;
                dict.set_item("Peri", data.iter().map(|d| d.Peri).collect::<Vec<_>>())?;
                dict.set_item("Node", data.iter().map(|d| d.Node).collect::<Vec<_>>())?;
                dict.set_item("i", data.iter().map(|d| d.i).collect::<Vec<_>>())?;
                dict.set_item("e", data.iter().map(|d| d.e).collect::<Vec<_>>())?;
                dict.set_item("a", data.iter().map(|d| d.a).collect::<Vec<_>>())?;
                dict.set_item("Principal_desig", data.iter().map(|d| &d.Principal_desig).collect::<Vec<_>>())?;
                
                let df = pd.call_method1("DataFrame", (dict,))?;
                Ok(df.into_py(py)) 
            })
        } else {
            Python::with_gil(|py| Ok(py.None()))
        }
    }
}