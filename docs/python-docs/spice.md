# SPICE Module Documentation

### Table of Contents
1. [Overview](#overview)
2. [SpiceKernel Class](#spicekernel-class)
3. [Configuration](#configuration)
4. [Examples](#examples)
5. [SpaceRock Integration](#spacerock-integration)

## Overview

The SPICE module provides integration with NASA's SPICE toolkit for spacecraft and planetary ephemerides calculations. This module manages SPICE kernel loading, downloading, and configuration.

## SpiceKernel Class

```python
class SpiceKernel:
    def __init__(self, config: Optional[str] = None):
        """
        Initialize a SpiceKernel instance.
        
        Arguments:
            config: Optional path to a config.toml file. 
                   If provided, loads and manages kernels according to configuration.
                   If None, creates an empty kernel for manual loading.
        """
```

### Basic Methods

```python
def load(self, path: str) -> None:
    """Load a SPICE kernel file from the specified path."""
    
def unload(self) -> None:
    """Unload all kernels and clear loaded files list."""
    
@property
def loaded_kernels(self) -> List[str]:
    """Return list of currently loaded kernel file paths."""
```

## Configuration

### Config File Structure (config.toml)
```toml
# Required kernels - will be loaded on initialization
default_kernels = [
    { name = "latest_leapseconds.tls", kernel_type = "lsk" }, 
    { name = "de440s.bsp", kernel_type = "spk/planets" },
    { name = "earth_1962_240827_2124_combined.bpc", kernel_type = "pck" } 
]

# Optional settings (defaults shown)
auto_download = true          # Download missing kernels automatically
kernel_paths = ["~/.spacerocks/kernels"]  # Where to look for existing kernels
download_dir = "~/.spacerocks/kernels"    # Where to store downloaded kernels
```

### Config Behavior
- Searches for kernels in specified kernel_paths
- If kernel not found and auto_download=true:
  - Downloads from SPICE server
  - Stores in download_dir
- If kernel not found and auto_download=false:
  - Raises informative error about missing kernel

## Examples

### Basic Usage (Manual Loading)
```python
# Create kernel for manual loading
kernel = SpiceKernel()

# Load specific kernels
kernel.load("path/to/my/kernel.bsp")
print(kernel.loaded_kernels)

# Unload when done
kernel.unload()
```

### Config-based Usage (Automatic Management)
```python
# Initialize with config file
kernel = SpiceKernel(config="path/to/config.toml")

# Kernels are automatically loaded/downloaded based on config
# Output shows process:
#   Found kernel: de440s.bsp in /path/to/kernels
#   Downloading: latest_leapseconds.tls...
#   Found kernel: earth_combined.bpc in /other/path
```

## SpaceRock Integration

```python
@classmethod
def from_spice(cls, name: str, epoch: Time, reference_plane: str, origin: str) -> 'SpaceRock':
    """
    Create SpaceRock object using SPICE ephemerides.
    
    Arguments:
        name: Body name (must match SPICE naming)
        epoch: Time of state vector
        reference_plane: Reference frame for coordinates
        origin: Origin of coordinate system
    
    Returns:
        SpaceRock instance
    
    Example:
        epoch = Time(2750923.093, "utc", "jd")
        mars = SpaceRock.from_spice("MARS BARYCENTER", epoch, "ECLIPJ2000", "SSB")
    """
```

### Notes
- For planets, use barycenter names (e.g., "MARS BARYCENTER")
- Position values converted from km to AU
- Velocities converted from km/s to AU/day
- Times handled through astropy Time objects

