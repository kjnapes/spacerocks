use pyo3::prelude::*;
use pyo3::exceptions::PyIndexError;
use pyo3::{PyErr, Python, PyResult};

use rayon::prelude::*;

use spacerocks::spacerock::SpaceRock;
use spacerocks::Time;

use crate::py_time::time::PyTime;
use crate::PySpaceRock;
use crate::py_observing::observer::PyObserver;
use crate::py_observing::observation::PyObservation;

use numpy::{PyArray1, IntoPyArray};

use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use nalgebra::Vector3;
use serde_json;
use arrow::array::{Float64Array, StringArray};
use crate::mpc::MPCData;


// use numpy::{PyArray1, IntoPyArray, PyArray};

// pub fn create_mixed_array<T: pyo3::IntoPyObject>(data: Vec<Option<T>>, py: Python) -> pyo3::Bound<'_, PyArray<pyo3::Py<PyAny>, numpy::ndarray::Dim<[usize; 1]>>> {
//     let numpy_array: Vec<_> = data.into_iter()
//             .map(|opt| match opt {
//                     Some(value) => value.to_object(py),
//                     None => py.None(),
//                 }
//             ).collect();
//     numpy_array.into_pyarray(py).to_owned()
// }

#[pyclass]
pub struct RockCollection {
    pub rocks: Vec<SpaceRock>,
}

#[pymethods]
impl RockCollection {
    
    #[new]
    pub fn new() -> Self {
        RockCollection { rocks: Vec::new() }
    }

    #[staticmethod]
    pub fn from_mpc(
        mpc_path: String,
        catalog: String, 
        download_data: bool, 
    ) -> PyResult<Self> {
        let path = PathBuf::from(&mpc_path);
        let handler = MPCHandler::new(path.clone());

        // Create a directory if it doesn't exist
        fs::create_dir_all(&path)?;

        let feather_path = handler.mpc_path.join(format!("{}.feather", catalog));

        if download_data || !feather_path.exists() {
            handler.fetch_data(&catalog).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
            })?;
        }

        handler.read_data(&catalog).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
        })
    }

    

    // #[classmethod]
    // pub fn random(_cls: &PyType, n: usize) -> Self {
    //     let rocks: Vec<SpaceRock> = (0..n).into_par_iter().map(|_| SpaceRock::random()).collect();
    //     let mut name_hash_map = HashMap::new();
    //     for (i, rock) in rocks.iter().enumerate() {
    //         name_hash_map.insert(rock.name.to_string(), i);
    //     }
    //     RockCollection { rocks: rocks, name_hash_map: name_hash_map }
    // }


    pub fn add(&mut self, rock: PyRef<PySpaceRock>) {
        self.rocks.push(rock.inner.clone());
    }

    fn __getitem__(&self, index: usize) -> PyResult<PySpaceRock> {
        if index < self.rocks.len() {
            Ok(PySpaceRock { inner: self.rocks[index].clone() })
        } else {
            Err(PyIndexError::new_err("Index out of range!"))
        }
    }

    // function to filter rocks by a boolean array, and then return a new RockCollection of clones of the rocks that are True
    // pub fn filter(&self, mask: &PyArray1<bool>) -> PyResult<RockCollection> {
    //     let mask = unsafe {mask.as_array()};
    //     let mut new_rocks = Vec::new();
    //     let mut new_name_hash_map = HashMap::new();
    //     for (i, rock) in self.rocks.iter().enumerate() {
    //         if mask[i] {
    //             new_rocks.push(rock.clone());
    //             new_name_hash_map.insert(rock.name.to_string(), new_rocks.len()-1);
    //         }
    //     }
    //     Ok(RockCollection { rocks: new_rocks, name_hash_map: new_name_hash_map })
    // }


    // pub fn calculate_orbit(&mut self) {
    //     self.rocks.par_iter_mut().for_each(|rock| rock.calculate_orbit());
    // }

    pub fn observe(&mut self, observer: PyRef<PyObserver>) -> PyResult<Vec<PyObservation>> {
        let o = observer.inner.clone();

        if o.reference_plane() != "J2000" {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Observer frame is not J2000. Cannot observe rocks.")));
        }

        let observations: Vec<_> = self.rocks.par_iter_mut().map(|rock| rock.observe(&o).unwrap()).collect();   
        let py_observations: Vec<_> = observations.into_iter().map(|obs| PyObservation { inner: obs }).collect();
        Ok(py_observations)
           
    }

    // pub fn analytic_propagate(&mut self, t: PyRef<PyTime>) {
    //     let ep = &t.inner;
    //     self.rocks.par_iter_mut().for_each(|rock| rock.analytic_propagate(ep));
    // }

    pub fn change_reference_plane(&mut self, reference_plane: &str) {
        self.rocks.par_iter_mut().for_each(|rock| rock.change_reference_plane(reference_plane).expect("Failed to change frame"));
    }

    #[getter]
    pub fn reference_plane(&self) -> Vec<String> {
        let reference_planes = self.rocks.par_iter().map(|rock| rock.reference_plane.to_string()).collect::<Vec<String>>();
        reference_planes
    }

    
    fn __len__(&self) -> usize {
        self.rocks.len()
    }

    pub fn len(&self) -> usize {
        self.rocks.len()
    }

    pub fn __repr__(&self) -> String {
        format!("RockCollection: {} rocks", self.rocks.len())
    }

    #[getter]
    pub fn x(&self, py: Python) -> Py<PyArray1<f64>> {
        let x: Vec<f64> = self.rocks.par_iter().map(|rock| rock.position[0]).collect();
        x.into_pyarray(py).to_owned().into()
    }

    #[getter]
    pub fn y(&self, py: Python) -> Py<PyArray1<f64>> {
        let y: Vec<f64> = self.rocks.par_iter().map(|rock| rock.position[1]).collect();
        y.into_pyarray(py).to_owned().into()
    }

    #[getter]
    pub fn z(&self, py: Python) -> Py<PyArray1<f64>> {
        let z: Vec<f64> = self.rocks.par_iter().map(|rock| rock.position[2]).collect();
        z.into_pyarray(py).to_owned().into()
    }

    #[getter]
    pub fn vx(&self, py: Python) -> Py<PyArray1<f64>> {
        let vx: Vec<f64> = self.rocks.par_iter().map(|rock| rock.velocity[0]).collect();
        vx.into_pyarray(py).to_owned().into()
    }

    #[getter]
    pub fn vy(&self, py: Python) -> Py<PyArray1<f64>> {
        let vy: Vec<f64> = self.rocks.par_iter().map(|rock| rock.velocity[1]).collect();
        vy.into_pyarray(py).to_owned().into()
    }

    #[getter]
    pub fn vz(&self, py: Python) -> Py<PyArray1<f64>> {
        let vz: Vec<f64> = self.rocks.par_iter().map(|rock| rock.velocity[2]).collect();
        vz.into_pyarray(py).to_owned().into()
    }

    // #[getter]
    pub fn r(&self, py: Python) -> Py<PyArray1<f64>> {
        let r: Vec<f64> = self.rocks.par_iter().map(|rock| rock.r()).collect();
        r.into_pyarray(py).to_owned().into()
    }

    // #[getter]
    // pub fn name(&self) -> Vec<String> {
    //     self.rocks.par_iter().map(|rock| rock.name.clone()).collect()
    // }
    // #[getter] 
    // pub fn name(&self, py: Python) -> PyResult<Py<PyArray1<PyObject>>> {
    //     let names: Vec<Option<String>> = self.rocks.par_iter().map(|rock| Some((*rock.name).clone())).collect();
    //     create_mixed_array(names, py)
    // }

    pub fn a(&self, py: Python) -> Py<PyArray1<f64>> {
        let a_values: Vec<f64> = self.rocks.par_iter().map(|rock| rock.a()).collect();
        a_values.into_pyarray(py).to_owned().into()
    }

    pub fn q(&self, py: Python) -> Py<PyArray1<f64>> {
        let q_values: Vec<f64> = self.rocks.par_iter().map(|rock| rock.q()).collect();
        q_values.into_pyarray(py).to_owned().into()
    }

    pub fn e(&self, py: Python) -> Py<PyArray1<f64>> {
        let e_values: Vec<f64> = self.rocks.par_iter().map(|rock| rock.e()).collect();
        e_values.into_pyarray(py).to_owned().into()
    }

    pub fn inc(&self, py: Python) -> Py<PyArray1<f64>> {
        let inc_values: Vec<f64> = self.rocks.par_iter().map(|rock| rock.inc()).collect();
        inc_values.into_pyarray(py).to_owned().into()
    }

    pub fn node(&self, py: Python) -> Py<PyArray1<f64>> {
        let node_values: Vec<f64> = self.rocks.par_iter().map(|rock| rock.node()).collect();
        node_values.into_pyarray(py).to_owned().into()
    }

    pub fn arg(&self, py: Python) -> Py<PyArray1<f64>> {
        let arg_values: Vec<f64> = self.rocks.par_iter().map(|rock| rock.arg()).collect();
        arg_values.into_pyarray(py).to_owned().into()
    }

    pub fn true_anomaly(&self, py: Python) -> Py<PyArray1<f64>> {
        let true_anomaly_values: Vec<f64> = self.rocks.par_iter().map(|rock| rock.true_anomaly()).collect();
        true_anomaly_values.into_pyarray(py).to_owned().into()
    }

    pub fn mean_anomaly(&self, py: Python) -> Py<PyArray1<f64>> {
        let mean_anomaly_values: Vec<f64> = self.rocks.par_iter().map(|rock| rock.mean_anomaly()).collect();
        mean_anomaly_values.into_pyarray(py).to_owned().into()
    }

    pub fn conic_anomaly(&self, py: Python) -> Py<PyArray1<f64>> {
        let conic_anomaly_values: Vec<f64> = self.rocks.par_iter().map(|rock| rock.conic_anomaly()).collect();
        conic_anomaly_values.into_pyarray(py).to_owned().into()
    }


    #[getter]
    pub fn epoch(&self) -> Vec<PyTime> {
        self.rocks.par_iter().map(|rock| PyTime { inner: rock.epoch.clone() }).collect()
    }

}



struct MPCHandler {
    mpc_path: PathBuf,
}

impl MPCHandler {
    pub fn new(mpc_path: PathBuf) -> Self {
        Self { mpc_path } 
    }

    pub fn mpc_path(&self) -> &Path {
        &self.mpc_path
    }

    pub fn fetch_data(&self, catalog: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://www.minorplanetcenter.net/Extended_Files/{}.json.gz", 
            catalog
        );

        println!("Downloading from URL: {}", url);

        let response = reqwest::blocking::get(&url)?;
        let content = response.bytes()?;
        
        // Save the zipped JSON file
        let zip_path = self.mpc_path.join(format!("{}.json.gz", catalog));
        fs::write(&zip_path, content)?;

        // Read and parse the JSON data
        let reader = flate2::read::GzDecoder::new(fs::File::open(&zip_path)?);
        let data: Vec<MPCData> = serde_json::from_reader(reader)?;


        // Save as feather
        self.save_as_feather(&data, &self.mpc_path.join(format!("{}.feather", catalog)))?;

        // Remove the zipped JSON file
        // fs::remove_file(&zip_path)?;


        Ok(())

    }

    fn read_data(&self, catalog: &str) -> Result<RockCollection, Box<dyn std::error::Error>> {
        let feather_path = self.mpc_path.join(format!("{}.feather", catalog));
        
        if !feather_path.exists() {
            return Err("Feather file not found. Call fetch_data first.".into());
        }

        let file = fs::File::open(&feather_path)?;
        let reader = arrow::ipc::reader::FileReader::try_new(file, None)?;

        let mut rocks = Vec::new();

        for batch in reader {
            let batch = batch?;
            rocks.extend(self.record_batch_to_spacerocks(&batch)?);
        }

        Ok(RockCollection { rocks })
    }

    fn record_batch_to_spacerocks(
        &self, 
        batch: &arrow::record_batch::RecordBatch
    ) -> Result<Vec<SpaceRock>, Box<dyn std::error::Error>> {


        let mut rocks = Vec::with_capacity(batch.num_rows());

        for row in 0..batch.num_rows() {
            let h = batch
            .column_by_name("H")
            .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
            .map(|arr| arr.value(row));

            let g = batch
            .column_by_name("G")
            .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
            .map(|arr| arr.value(row));

            let name = batch
            .column_by_name("Principal_desig")
            .and_then(|col| col.as_any().downcast_ref::<StringArray>())
            .map(|arr| arr.value(row))
            .unwrap_or_default();

            let epoch = batch
            .column_by_name("Epoch")
            .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
            .map(|arr| arr.value(row))
            .unwrap_or_default();

            let m = batch
            .column_by_name("M")
            .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
            .map(|arr| arr.value(row))
            .unwrap_or_default();


            let arg = batch
            .column_by_name("Peri")
            .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
            .map(|arr| arr.value(row))
            .unwrap_or_default();

            let node = batch
            .column_by_name("Node")
            .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
            .map(|arr| arr.value(row))
            .unwrap_or_default();

            let inc = batch
            .column_by_name("i")
            .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
            .map(|arr| arr.value(row))
            .unwrap_or_default();

            let e = batch
            .column_by_name("e")
            .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
            .map(|arr| arr.value(row))
            .unwrap_or_default();

            let a = batch
            .column_by_name("a")
            .and_then(|col| col.as_any().downcast_ref::<Float64Array>())
            .map(|arr| arr.value(row))
            .unwrap_or_default();

            // Create SpaceRock from Orbital Elements
            let mut rock = SpaceRock::from_kepler(
                &name,
                a,
                e,
                inc,
                arg,
                node,
                m,
                Time::new(epoch, "utc", "jd")?,
                "J2000",
                "SSB",
            )?;

            // Set properties
            if let Some(h) = h {
                rock.set_absolute_magnitude(h);
            }
            if let Some(g) = g {
                rock.set_gslope(g);
            }

            rocks.push(rock);

        }

        Ok(rocks)
    }

    fn save_as_feather(&self, data: &[MPCData], path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use arrow::array::{Float64Array, StringArray};
        use arrow::datatypes::{DataType, Field, Schema};
        use arrow::record_batch::RecordBatch;
        
        // Define schema
        let schema = Schema::new(vec![
            Field::new("H", DataType::Float64, true),
            Field::new("G", DataType::Float64, true),
            Field::new("Epoch", DataType::Float64, false),
            Field::new("M", DataType::Float64, false),
            Field::new("Peri", DataType::Float64, false),
            Field::new("Node", DataType::Float64, false),
            Field::new("i", DataType::Float64, false),
            Field::new("e", DataType::Float64, false),
            Field::new("a", DataType::Float64, false),
            Field::new("Principal_desig", DataType::Utf8, false),
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

        // Create record batch
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
            ],
        )?;

        // Write to feather file
        let file = fs::File::create(path)?;
        let mut writer = arrow::ipc::writer::FileWriter::try_new(file, &batch.schema())?;
        writer.write(&batch)?;
        writer.finish()?;
        
        Ok(())
    }
}
    