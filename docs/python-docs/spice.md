<h1 style="border-bottom: 5px solid white;">SPICE Module</h1>

### Table of Contents
1. [Overview](#overview)
2. [Primary Classes](#primary-classes)
3. [Methods](#constructor-methods)
4. [Configuration](#configuration)
5. [Examples](#examples)
6. [Notes](#notes)

<h2 style="border-bottom: 3px solid white;">Overview</h2>

The SPICE module provides integration with NASA's SPICE toolkit for spacecraft and planetary ephemerides calculations. It handles kernel management, including automatic downloading and configuration. 

<h2 style="border-bottom: 3px solid white;">Primary Classes</h2>

### SpiceKernel
```python
class SpiceKernel:
    """
    Manages SPICE kernel loading and configuration.
    
    Example:
        kernel = SpiceKernel.defaults()
        kernel.load("de440s.bsp")
    """
```

<h2 style="border-bottom: 3px solid white;">Methods</h2>


### Constructor Methods
---

**`new()`**

**Returns:**
- New SpiceKernel instance

*Example:*
```python
kernel = SpiceKernel()
```

**`defaults()`**
```python
@classmethod
def defaults(cls, download: bool = True) -> SpiceKernel
```
**Arguments:**
- `download`: Whether to automatically download missing kernels

**Returns:**
- SpiceKernel instance with default configuration loaded

*Example:*
```python
kernel = SpiceKernel.defaults(download=True)
```

**`from_config()`**
```python
@classmethod
def from_config(cls, path: str) -> SpiceKernel
```
**Arguments:**
- `path`: Path to configuration file

**Returns:**
- SpiceKernel instance configured according to specified file

*Example:*
```python
kernel = SpiceKernel.from_config("config.toml")
```

### Operation Methods
---

**`load()`**
```python
def load(self, path: str) -> None
```
**Arguments:**
- `path`: Path to SPICE kernel file to load

*Example:*
```python
kernel.load("path/to/kernel.bsp")
```

**`unload()`**
```python
def unload(self) -> None
```
Unloads all currently loaded kernels.

*Example:*
```python
kernel.unload()
```

**`loaded_kernels()`**
```python
@property
def loaded_kernels(self) -> List[str]
```
**Returns:**
- List of paths to currently loaded kernel files

*Example:*
```python
paths = kernel.loaded_kernels
```

<h2 style="border-bottom: 3px solid white;">Configuration</h2>


### Config File Format (config.toml)
---
```toml
# Required kernels that will be loaded on initialization
default_kernels = [
    { name = "latest_leapseconds.tls", kernel_type = "lsk" },
    { name = "de440s.bsp", kernel_type = "spk/planets" },
    { name = "earth_1962_240827_2124_combined.bpc", kernel_type = "pck" }
]

# Optional settings with defaults shown
auto_download = true                      # Download missing kernels
kernel_paths = ["~/.spacerocks/kernels"]  # Search paths for kernels
download_dir = "~/.spacerocks/kernels"    # Download location
```

<h2 style="border-bottom: 3px solid white;">Examples</h2>


### Manual Kernel Management
```python
from spacerocks.spice import SpiceKernel

# Create empty kernel manager
kernel = SpiceKernel()

# Manually load specific kernels
kernel.load("de440s.bsp")
kernel.load("latest_leapseconds.tls")

# Check loaded kernels
print(kernel.loaded_kernels)

# Unload when finished
kernel.unload()
```

### Automatic Management
```python
from spacerocks.spice import SpiceKernel

# Use default configuration
kernel = SpiceKernel.defaults()

# Kernels are automatically loaded
# Output shows process:
#   Found kernel: de440s.bsp in /path/to/kernels
#   Downloading: latest_leapseconds.tls...
#   Found kernel: earth_combined.bpc in /path

# Or use custom configuration
kernel = SpiceKernel.from_config("myconfig.toml")
```

<h2 style="border-bottom: 3px solid white;">Notes</h2>

- Kernels must be loaded before using SPICE-dependent features
- Default configuration provides essential kernels for most use cases
- Downloaded kernels are cached for future use
- Kernel search paths are checked before downloading
- Configuration paths use `~` for home directory expansion
- All kernels are automatically unloaded when the object is destroyed