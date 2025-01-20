use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::constants::SPICE_URL;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KernelSpec {
    pub name: String,
    pub kernel_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub default_kernels: Vec<KernelSpec>,
    
    #[serde(default)]
    pub kernel_paths: Vec<PathBuf>,
    
    #[serde(default = "default_download_setting")]
    pub auto_download: bool,
    
    #[serde(default = "default_download_dir")]
    pub download_dir: PathBuf,
}

fn default_download_setting() -> bool {
    true
}

fn default_download_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".spacerocks")
        .join("kernels")
}

pub struct SpiceKernel {
    loaded_files: Vec<String>,
    config: Option<Config>,
}

impl SpiceKernel {
    pub fn new() -> Self {
        SpiceKernel {
            loaded_files: vec![],
            config: None,
        }
    }

    pub fn from_config(path: &str) -> Result<Self, String> {
        println!("Loading configuration from {}", path);
        
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
            
        let config: Config = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))?;
            
        println!("\nConfiguration loaded:");
        println!("  Kernel paths: {:?}", config.kernel_paths);
        println!("  Download directory: {:?}", config.download_dir);
        println!("  Auto-download: {}", config.auto_download);
        
        let mut kernel = SpiceKernel {
            loaded_files: vec![],
            config: Some(config),
        };
        
        // Process default kernels
        kernel.load_default_kernels()?;
        
        Ok(kernel)
    }

    fn process_kernel(&mut self, kernel_spec: &KernelSpec) -> Result<(), String> {
        // If we have kernel paths, check them first
        if let Some(config) = &self.config {
            println!("\nProcessing kernel: {}", kernel_spec.name);
            // Check each path
            for path in &config.kernel_paths {
                let kernel_path = path.join(&kernel_spec.name);
                if kernel_path.exists() {
                    println!("✓ Found existing kernel at: {}", kernel_path.display());
                    return self.load(kernel_path.to_str().unwrap());
                }
            }
            
            // Not found in paths - try downloading
            if config.auto_download {
                println!("➜ Kernel not found locally, downloading...");
                fs::create_dir_all(&config.download_dir)
                    .map_err(|e| format!("Failed to create download directory: {}", e))?;
                    
                let path = self.download_kernel(&kernel_spec.kernel_type, &kernel_spec.name)?;
                return self.load(path.to_str().unwrap());
            }
            
            return Err(format!("✗ Kernel {} not found in any paths and auto_download is false", kernel_spec.name));
        }
        
        Err("No configuration provided".to_string())
    }
    
    // Main function just iterates over kernels
    fn load_default_kernels(&mut self) -> Result<(), String> {
        let kernels = self.config.as_ref()
            .map(|c| c.default_kernels.clone())
            .unwrap_or_default();
            
        for kernel in kernels {
            self.process_kernel(&kernel)?;
        }
        Ok(())
    }

    fn download_kernel(&self, kernel_type: &str, filename: &str) -> Result<PathBuf, String> {
        let config = self.config.as_ref().unwrap();
        let url = format!("{}/{}/{}", SPICE_URL, kernel_type, filename);
        let path = config.download_dir.join(filename);
        
        println!("    Downloading from {}", url);
        println!("    Saving to {}", path.display());
        
        let response = reqwest::blocking::get(&url)
            .map_err(|e| format!("Download failed: {}", e))?;
            
        if !response.status().is_success() {
            return Err(format!("Download failed with status: {}", response.status()));
        }
        
        let content = response.bytes()
            .map_err(|e| format!("Failed to read response: {}", e))?;
            
        let mut file = File::create(&path)
            .map_err(|e| format!("Failed to create file: {}", e))?;
            
        file.write_all(&content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
            
        println!("    Download complete");
        Ok(path)
    }
    
    pub fn load(&mut self, path: &str) -> Result<(), String> {
        if self.loaded_files.contains(&path.to_string()) {
            println!("Kernel already loaded: {}", path);
            return Ok(());
        }
        
        println!("Loading kernel: {}", path);
        spice::furnsh(path);
        self.loaded_files.push(path.to_string());
        Ok(())
    }

    pub fn unload(&mut self) {
        println!("Unloading all kernels");
        spice::kclear();
        self.loaded_files.clear();
    }
    
    pub fn loaded_kernels(&self) -> &[String] {
        &self.loaded_files
    }
}