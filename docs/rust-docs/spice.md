# SPICE Module

### Table of Contents
1. [Overview](#overview)
2. [SpiceKernel Management](#spicekernel-management)
3. [SpaceRock Integration](#spacerock-integration)
4. [Notes](#notes)

## Overview

The SPICE module provides integration with NASA's SPICE toolkit for spacecraft and planetary ephemerides calculations. It provides a Rust interface for managing SPICE kernels and creating SpaceRock objects from SPICE data.

## SpiceKernel Management

### SpiceKernel Structure
```rust
pub struct SpiceKernel {
    pub loaded_files: Vec<String>,
}
```

The `SpiceKernel` struct maintains a list of loaded SPICE kernel files and provides methods for kernel management.

### Methods

#### new()
```rust
fn new() -> SpiceKernel
```

Creates a new empty SpiceKernel instance.

**Returns:**
- A new `SpiceKernel` with an empty list of loaded files

**Example:**
```rust
let kernel = SpiceKernel::new();
```
Creates a new empty SpiceKernel instance.

#### load()
```rust
fn load(&mut self, path: &str) -> Result<(), String>
```

Loads a SPICE kernel file from the specified path.

**Arguments:**
- `path`: Path to the kernel file to load

**Returns:**
- `Ok(())` if the kernel was loaded successfully
- `Err(String)` if the kernel was already loaded

**Example:**
```rust
let mut kernel = SpiceKernel::new();
kernel.load("path/to/kernel.bsp")?;
```
Loads a SPICE kernel file from the specified path. If the file is already loaded, returns an error.

Key features:
- Prevents duplicate loading of the same kernel
- Maintains a list of loaded kernels
- Returns error if kernel is already loaded

#### unload()
```rust
kernel.unload();
```
Unloads all kernels and clears the loaded files list.

#### display()
```rust
kernel.display();
```
Prints the list of currently loaded kernel files.

## SpaceRock Integration

### Creating SpaceRock Objects from SPICE

#### from_spice()
```rust
fn from_spice(
    name: &str,
    epoch: &Time,
    reference_plane: &str,
    origin: &str
) -> Result<Self, Box<dyn std::error::Error>>
```

Creates a new SpaceRock object using SPICE ephemerides data.

**Arguments:**
- `name`: Name of the celestial body (must match SPICE naming)
- `epoch`: Time of the state vector
- `reference_plane`: Reference frame for coordinates
- `origin`: Origin of the coordinate system

**Returns:**
- `Ok(SpaceRock)` if the object was created successfully
- `Err(Box<dyn std::error::Error>)` if there was an error accessing SPICE data

**Example:**
```rust
let epoch = Time::new(2750923.093, "utc", "jd")?;
let mars = SpaceRock::from_spice("MARS", &epoch, "ECLIPJ2000", "SUN")?;
```

Creates a new SpaceRock object using SPICE ephemerides data.

Parameters:
- `name`: Name of the celestial body (must match SPICE naming)
- `epoch`: Time of the state vector
- `reference_plane`: Reference frame for coordinates
- `origin`: Origin of the coordinate system

Key features:
- Converts time to SPICE ET (Ephemeris Time)
- Performs unit conversions (km to AU, km/s to AU/day)
- Sets mass if available in MASSES constant
- Handles coordinate frame transformations



## Notes

1. **Kernel Management**
   - Use `unload()` to clear all kernels and free resources
   - Duplicate kernel loading is prevented automatically

2. **Time Handling**
   - Times are passed to SPICE as UTC JD values
   - SPICE conversion to ET (Ephemeris Time) is handled in the `from_spice()` function

3. **Unit Conversions `from_spice()`**
   - Position values are converted from km to AU
   - Velocities are converted from km/s to AU/day