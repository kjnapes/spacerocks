use pyo3::prelude::*;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::types::PyDict;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use flate2::read::GzDecoder;
use std::fs;
use std::io::Read;
use arrow::array::{Float64Array, StringArray, Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use rayon::prelude::*;

use spacerocks::spacerock::SpaceRock;
use spacerocks::Time;
use crate::py_time::time::PyTime;

use serde_json;

use crate::RockCollection;
use anyhow::Error;
use spacerocks::constants::MPC_URL;

#[derive(Debug, Clone)]
pub enum StorageFormat {
    None,
    JsonGz,
    Feather,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    DataFrame,
    RockCollection,
}


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
    #[serde(rename = "Orbit_type")]
    pub orbit_type: Option<String>,
}

#[pyclass]
pub struct MPCHandler {
    path: PathBuf,
}

#[pymethods]
impl MPCHandler {
    #[new]
    pub fn new(path: String) -> Self {
        MPCHandler {
            path: PathBuf::from(path),
        }
    }

    /// Fetch and optionally process MPC data
    #[pyo3(signature = (catalog, orbit_type=None, storage_format=Some("feather".to_string()), output_format="dataframe"))]
    pub fn fetch_data(&self, catalog: String, orbit_type: Option<String>, storage_format: Option<String>, output_format: &str) -> PyResult<PyObject> {

        // Get the storage format
        let storage_type = match storage_format.as_deref() {
            Some("feather") => StorageFormat::Feather,
            Some("json") => StorageFormat::JsonGz,
            None => StorageFormat::None,
            Some(other) => return Err(PyErr::new::<PyValueError, _>(
                format!("Invalid storage format: {}. Must be 'feather', 'json', or None", other)
            )),
        };


        // Get output format
        let out_format = match output_format {
            "dataframe" => OutputFormat::DataFrame,
            "rocks" => OutputFormat::RockCollection,
            _ => return Err(PyErr::new::<PyValueError, _>(
                "Output format must be 'dataframe' or 'rocks'"
            )),
        };

        // Get the data
        let mut data = self.get_data(&catalog, &storage_type)?;

        // Filter by orbit_type if specified
        if let Some(orbit_filter) = orbit_type {
            data.retain(|d| {
                d.orbit_type.as_ref().map_or(false, |ot| ot == &orbit_filter)
            });
        }

        // Convert to requested output format
        match out_format {
            OutputFormat::DataFrame => self.to_dataframe(data),
            OutputFormat::RockCollection => self.to_rock_collection(data),
        }
    }

    #[staticmethod]
    pub fn create_rock_collection(mpc_path: String, catalog: String, download_data: bool, orbit_type: Option<String>,) -> PyResult<RockCollection> {
        let handler = MPCHandler::new(mpc_path);

        // Create the necessary directories
        fs::create_dir_all(&handler.path)?;

        // Only download if requested or if data does not exist
        if download_data {
            println!("Downloading new MPC data...");
            handler.fetch_data(
                catalog.clone(),
                orbit_type.clone(),
                Some("feather".to_string()),
                "rocks"
            )?;
        }

        // Convert PyObject to RockCollection
        let py_obj = handler.fetch_data(
            catalog,
            orbit_type,
            Some("feather".to_string()),
            "rocks"
        )?;

        Python::with_gil(|py| {
            let rocks_collection = py_obj.extract::<RockCollection>(py)?;
            Ok(rocks_collection)
        })
    }
}

impl MPCHandler {
    fn get_data(&self, catalog: &str, storage_type: &StorageFormat) -> PyResult<Vec<MPCData>> {
        let url = format!(
            "{}/{}.json.gz",
            MPC_URL,
            catalog
        );

        match storage_type {
            StorageFormat::None => {
                // Direct read from URL
                println!("Downloading data from {}", url);
                let response = reqwest::blocking::get(&url)
                    .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
                let reader = GzDecoder::new(response);
                serde_json::from_reader(reader)
                    .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))
            },
            StorageFormat::JsonGz => {
                let zip_path = self.path.join(format!("{}.json.gz", catalog));
                if !zip_path.exists() {
                    println!("Downloading data from {}", url);
                    println!("Saving to {}", zip_path.display());
                    let response = reqwest::blocking::get(&url)
                        .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
                    let content = response.bytes()
                        .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
                    fs::create_dir_all(&self.path)?;
                    fs::write(&zip_path, content)?;
                } else {
                    println!("Using existing file: {}", zip_path.display());
                }
                let reader = GzDecoder::new(fs::File::open(&zip_path)?);
                serde_json::from_reader(reader)
                    .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))
            },
            StorageFormat::Feather => {
                let feather_path = self.path.join(format!("{}.feather", catalog));
                if !feather_path.exists() {
                    println!("Downloading data from {}", url);
                    println!("Converting and saving to {}", feather_path.display());
                    // First get JSON data
                    let response = reqwest::blocking::get(&url)
                        .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
                    let reader = GzDecoder::new(response);
                    let data: Vec<MPCData> = serde_json::from_reader(reader)
                        .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
                    
                    // Save as feather
                    fs::create_dir_all(&self.path)?;
                    self.save_as_feather(&data, &feather_path)?;
                    Ok(data)
                } else {
                    println!("Using existing file: {}", feather_path.display());
                    self.read_from_feather(&feather_path)
                }
            }
        }
    }

    fn to_dataframe(&self, data: Vec<MPCData>) -> PyResult<PyObject> {
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
            dict.set_item("orbit_type", data.iter().map(|d| &d.orbit_type).collect::<Vec<_>>())?;
            
            let df = pd.call_method1("DataFrame", (dict,))?;
            Ok(df.into_py(py))
        })
    }

    // fn to_rock_collection(&self, data: Vec<MPCData>) -> PyResult<PyObject> {
    //     let rocks: Result<Vec<SpaceRock>, Box<dyn std::error::Error>> = data.par_iter().map(|d| {
    //         let mut rock = SpaceRock::from_kepler(
    //             &d.Principal_desig,
    //             d.a,
    //             d.e,
    //             d.i,
    //             d.Peri,
    //             d.Node,
    //             d.M,
    //             Time::new(d.Epoch, "utc", "jd")?,
    //             "J2000",
    //             "SSB",
    //         )?;

    //         if let Some(h) = d.H {
    //             rock.set_absolute_magnitude(h);
    //         }

    //         if let Some(g) = d.G {
    //             rock.set_gslope(g);
    //         }

    //         Ok(rock)
    //     }).collect();

    //     match rocks {
    //         Ok(rocks) => Python::with_gil(|py| {
    //             Ok(RockCollection { rocks }.into_py(py))
    //         }),
    //         Err(e) => Err(PyErr::new::<PyRuntimeError, _>(e.to_string()))
    //     }
    // }

    fn to_rock_collection(&self, data: Vec<MPCData>) -> PyResult<PyObject> {
        let rocks: Vec<Result<SpaceRock, String>> = 
            data.par_iter().map(|d| {
                match SpaceRock::from_kepler(
                    &d.Principal_desig,
                    d.a,
                    d.e,
                    d.i,
                    d.Peri,
                    d.Node,
                    d.M,
                    Time::new(d.Epoch, "utc", "jd").map_err(|e| e.to_string())?,
                    "J2000",
                    "SSB",
                ) {
                    Ok(mut rock) => {
                        if let Some(h) = d.H {
                            rock.set_absolute_magnitude(h);
                        }
                        if let Some(g) = d.G {
                            rock.set_gslope(g);
                        }
                        Ok(rock)
                    },
                    Err(e) => Err(e.to_string())
                }
            }).collect();
    
        // Process results
        let rocks: Result<Vec<SpaceRock>, String> = rocks.into_iter()
            .collect();
    
        match rocks {
            Ok(rocks) => Python::with_gil(|py| {
                #[allow(deprecated)]
                Ok(RockCollection { rocks }.into_py(py))
            }),
            Err(e) => Err(PyErr::new::<PyRuntimeError, _>(e))
        }
    }

    fn save_as_feather(&self, data: &[MPCData], path: &Path) -> PyResult<()> {
        let schema = Schema::new(vec![
            Field::new("H", DataType::Float64, true),
            Field::new("G", DataType::Float64, true),
            Field::new("Epoch", DataType::Float64, true),  // Changed to nullable
            Field::new("M", DataType::Float64, true),      // Changed to nullable
            Field::new("Peri", DataType::Float64, true),   // Changed to nullable
            Field::new("Node", DataType::Float64, true),   // Changed to nullable
            Field::new("i", DataType::Float64, true),      // Changed to nullable
            Field::new("e", DataType::Float64, true),      // Changed to nullable
            Field::new("a", DataType::Float64, true),      // Changed to nullable
            Field::new("Principal_desig", DataType::Utf8, true), // Changed to nullable
            Field::new("orbit_type", DataType::Utf8, true)
        ]);

        // Create arrays
        let h_array = Float64Array::from(data.iter().map(|d| d.H).collect::<Vec<_>>());
        let g_array = Float64Array::from(data.iter().map(|d| d.G).collect::<Vec<_>>());
        let epoch_array = Float64Array::from(data.iter().map(|d| d.Epoch).collect::<Vec<_>>());
        let m_array = Float64Array::from(data.iter().map(|d| d.M).collect::<Vec<_>>());
        let peri_array = Float64Array::from(data.iter().map(|d| d.Peri).collect::<Vec<_>>());
        let node_array = Float64Array::from(data.iter().map(|d| d.Node).collect::<Vec<_>>());
        let inc_array = Float64Array::from(data.iter().map(|d| d.i).collect::<Vec<_>>());
        let e_array = Float64Array::from(data.iter().map(|d| d.e).collect::<Vec<_>>());
        let a_array = Float64Array::from(data.iter().map(|d| d.a).collect::<Vec<_>>());
        let principal_desig_array = StringArray::from(
            data.iter().map(|d| d.Principal_desig.as_str()).collect::<Vec<_>>()
        );
        let orbit_type_array = StringArray::from(
            data.iter().map(|d| d.orbit_type.as_deref()).collect::<Vec<_>>()
        );

        // Create RecordBatch
        let batch = RecordBatch::try_new(
            std::sync::Arc::new(schema),
            vec![
                std::sync::Arc::new(h_array),
                std::sync::Arc::new(g_array),
                std::sync::Arc::new(epoch_array),
                std::sync::Arc::new(m_array),
                std::sync::Arc::new(peri_array),
                std::sync::Arc::new(node_array),
                std::sync::Arc::new(inc_array),
                std::sync::Arc::new(e_array),
                std::sync::Arc::new(a_array),
                std::sync::Arc::new(principal_desig_array),
                std::sync::Arc::new(orbit_type_array),
            ],
        ).map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;

        // Write to feather file
        let file = fs::File::create(path)?;
        let mut writer = arrow::ipc::writer::FileWriter::try_new(file, &batch.schema())
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
        writer.write(&batch)
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
        writer.finish()
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
        
        Ok(())
    }

    // fn read_from_feather(&self, path: &Path) -> PyResult<Vec<MPCData>> {
    //     let file = fs::File::open(path)?;
    //     let reader = arrow::ipc::reader::FileReader::try_new(file, None)
    //         .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;

    //     let mut data = Vec::new();

    //     for batch in reader {
    //         let batch = batch.map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
    //         data.extend(self.record_batch_to_mpc_data(&batch)?);
    //     }

    //     Ok(data)
    // }

    fn read_from_feather(&self, path: &Path) -> PyResult<Vec<MPCData>> {
        let file = fs::File::open(path)?;
        let reader = arrow::ipc::reader::FileReader::try_new(file, None)
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
    
        let mut data = Vec::new();
    
        // Using rayon's parallel iterator for batch processing
        let batches: Vec<_> = reader.collect();
        let processed: Result<Vec<_>, _> = batches.par_iter()
            .map(|batch_result| {
                let batch = batch_result.as_ref()
                    .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;
                self.record_batch_to_mpc_data(batch)
            })
            .collect();
    
        match processed {
            Ok(batch_data) => {
                data.extend(batch_data.into_iter().flatten());
                Ok(data)
            },
            Err(e) => Err(e),
        }
    }

    fn record_batch_to_mpc_data(&self, batch: &RecordBatch) -> PyResult<Vec<MPCData>> {
        let mut data = Vec::with_capacity(batch.num_rows());

        for row in 0..batch.num_rows() {
            let h = batch
                .column_by_name("H")
                .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
                .map(|arr| arr.value(row));

            let g = batch
                .column_by_name("G")
                .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
                .map(|arr| arr.value(row));

            let principal_desig = batch
                .column_by_name("Principal_desig")
                .and_then(|col| col.as_any().downcast_ref::<StringArray>())
                .map(|arr| arr.value(row))
                .unwrap_or_default();

            let orbit_type = batch
                .column_by_name("orbit_type")
                .and_then(|col| col.as_any().downcast_ref::<StringArray>())
                .and_then(|arr| if arr.is_null(row) { None } else { Some(arr.value(row).to_string()) });

            // Get required numeric values
            let get_f64 = |name: &str| -> f64 {
                batch
                    .column_by_name(name)
                    .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
                    .map(|arr| arr.value(row))
                    .unwrap_or_default()
            };

            let mpc_data = MPCData {
                H: h,
                G: g,
                Principal_desig: principal_desig.to_string(),
                orbit_type,
                Epoch: get_f64("Epoch"),
                M: get_f64("M"),
                Peri: get_f64("Peri"),
                Node: get_f64("Node"),
                i: get_f64("i"),
                e: get_f64("e"),
                a: get_f64("a"),
            };

            data.push(mpc_data);
        }

        Ok(data)
    }
}















//         let url = format!(
//             "https://www.minorplanetcenter.net/Extended_Files/{}.json.gz",
//             catalog
//         );
    
//         let zip_path = self.path.join(format!("{}.json.gz", catalog));
//         if !zip_path.exists() {
//             println!("Downloading from URL: {}", url);
//             let response = reqwest::blocking::get(&url)
//                 .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
//             let content = response.bytes()
//                 .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
//             fs::write(&zip_path, &content)
//                 .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
//         }
    
//         // Parse JSON into a DataFrame
//         let mut reader = GzDecoder::new(fs::File::open(&zip_path)?);
//         let mut data: Vec<MPCData> = serde_json::from_reader(reader)
//             .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

//         // Filter by orbit_type if specified
//         if let Some(orbit_filter) = orbit_type {
//             data.retain(|d| {
//                 d.orbit_type.as_ref().map_or(false, |ot| ot == &orbit_filter)
//             });
//         }
    
//         if save_to_df {
//             Python::with_gil(|py| {
//                 let pd = py.import("pandas")?;
//                 let dict = PyDict::new(py);
//                 dict.set_item("H", data.iter().map(|d| d.H).collect::<Vec<_>>())?;
//                 dict.set_item("G", data.iter().map(|d| d.G).collect::<Vec<_>>())?;
//                 dict.set_item("Epoch", data.iter().map(|d| d.Epoch).collect::<Vec<_>>())?;
//                 dict.set_item("M", data.iter().map(|d| d.M).collect::<Vec<_>>())?;
//                 dict.set_item("Peri", data.iter().map(|d| d.Peri).collect::<Vec<_>>())?;
//                 dict.set_item("Node", data.iter().map(|d| d.Node).collect::<Vec<_>>())?;
//                 dict.set_item("i", data.iter().map(|d| d.i).collect::<Vec<_>>())?;
//                 dict.set_item("e", data.iter().map(|d| d.e).collect::<Vec<_>>())?;
//                 dict.set_item("a", data.iter().map(|d| d.a).collect::<Vec<_>>())?;
//                 dict.set_item("Principal_desig", data.iter().map(|d| &d.Principal_desig).collect::<Vec<_>>())?;
//                 dict.set_item("orbit_type", data.iter().map(|d| &d.orbit_type).collect::<Vec<_>>())?;
                
//                 let df = pd.call_method1("DataFrame", (dict,))?;
//                 Ok(df.into_py(py)) 
//             })
//         } else {
//             Python::with_gil(|py| Ok(py.None()))
//         }
//     }
// }