# SPICE Module (Python)

### Table of Contents
1. [Overview](#overview)
2. [SpiceKernel Management](#spicekernel-management)
3. [SpaceRock Integration](#spacerock-integration)
4. [Notes](#notes)

## Overview

The SPICE module provides integration with NASA's SPICE toolkit for spacecraft and planetary ephemerides calculations. It provides a Python interface for managing SPICE kernels and creating SpaceRock objects from SPICE data.

## SpiceKernel Management

### SpiceKernel Class
```python
class SpiceKernel:
    def __init__(self):
        self.loaded_files = []
```

The `SpiceKernel` class maintains a list of loaded SPICE kernel files and provides methods for kernel management.

### Kernal Initialization


**Behavior:**
- Creates an empty list to track loaded files

**Example:**
```python
kernel = SpiceKernel()
```

### Methods

#### load()
```python
def load(self, path: str) -> None
```

Loads a SPICE kernel file from the specified path.

**Arguments:**
- `path`: Path to the kernel file to load

**Returns:**
- None if successful
- Raises `ValueError` if the kernel was already loaded

**Example:**
```python
kernel = SpiceKernel()
kernel.load("path/to/kernel.bsp")
```
Loads a SPICE kernel file from the specified path. If the file is already loaded, raises a ValueError.

Key features:
- Prevents duplicate loading of the same kernel
- Maintains a list of loaded kernels
- Raises ValueError if kernel is already loaded

#### unload()
```python
kernel.unload()
```
Unloads all kernels and clears the loaded files list.

#### __repr__()
```python
def __repr__(self) -> str
```
Returns a string representation of the SpiceKernel instance, showing all loaded kernel files.

**Example output:**
```python
>>> kernel = SpiceKernel()
>>> kernel.load("path/to/kernel1.bsp")
>>> kernel.load("path/to/kernel2.bsp")
>>> kernel
SpiceKernel
 - path/to/kernel1.bsp
 - path/to/kernel2.bsp
```

#### loaded_files (property)
```python
@property
def loaded_files(self) -> List[str]
```
Returns a list of currently loaded kernel file paths.

**Example:**
```python
>>> kernel = SpiceKernel()
>>> kernel.load("path/to/kernel1.bsp")
>>> kernel.load("path/to/kernel2.bsp")
>>> kernel.loaded_files
['path/to/kernel1.bsp', 'path/to/kernel2.bsp']
```

## SpaceRock Integration

### Creating SpaceRock Objects from SPICE

#### from_spice()
```python
@classmethod
def from_spice(
    cls,
    name: str,
    epoch: Time,
    reference_plane: str,
    origin: str
) -> 'SpaceRock'
```

Creates a new SpaceRock object using SPICE ephemerides data.

**Arguments:**
- `name`: Name of the celestial body (must match SPICE naming)
- `epoch`: Time of the state vector
- `reference_plane`: Reference frame for coordinates
- `origin`: Origin of the coordinate system

**Returns:**
- SpaceRock instance if successful
- Raises Exception if there was an error accessing SPICE data

**Example:**
```python
epoch = Time(2750923.093, "utc", "jd")
mars = SpaceRock.from_spice("MARS BARYCENTER", epoch, "ECLIPJ2000", "SSB")
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
   - Times are passed to SPICE as `Time` objects
   - SPICE conversion to ET (Ephemeris Time) is handled in the `from_spice()` function

3. **Unit Conversions `from_spice()` SpaceRock Method**
   - Position values are converted from km to AU
   - Velocities are converted from km/s to AU/day

4. **Planet ID's within SPICE**
   - When attempting to generate a SpaceRock with the from_spice method for a planet within the Solar System, we must specify that we are using the barycenter of that planetary system (i.e: 'mars barycenter', 'jupiter barycenter', etc)