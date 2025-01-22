<h1 style="border-bottom: 5px solid white;">Observing Module</h1>


### Table of Contents
1. [Overview](#overview)
2. [Primary Classes](#primary-classes)
3. [Methods](#methods)
4. [Properties](#properties)
5. [Examples](#examples)
6. [Notes](#notes)

<h2 style="border-bottom: 3px solid white;">Overview</h2>


The Observing module provides tools for working with astronomical observations. It handles observatory locations, observer positions, and various types of observational data through three main classes:
- `Observatory`: Represents physical observation locations
- `Observer`: Represents an observatory at a specific time
- `Observation`: Handles different types of astronomical measurements

<h2 style="border-bottom: 3px solid white;">Primary Classes</h2>

### Observatory
```python
class Observatory:
    """
    Manages observation locations including ground-based observatories and space telescopes.
    
    Example:
        maunakea = Observatory.from_obscode('568')
        observer = maunakea.at(epoch)
    """
```

### Observer
```python
class Observer:
    """
    Represents an observatory at a specific time, providing position and velocity information.
    
    Example:
        observer = observatory.at(epoch)
        position = observer.position  # [x, y, z] in AU
    """
```

### Observation
```python
class Observation:
    """
    Handles different types of astronomical observations and their measurements.
    
    Example:
        obs = Observation.from_astrometry(epoch, ra, dec, observer)
        print(f"RA: {obs.ra}, Dec: {obs.dec}")
    """
```

<h2 style="border-bottom: 3px solid white;">Methods</h2>

### Observatory Methods
---

**`from_obscode()`**
```python
@classmethod
def from_obscode(cls, obscode: str) -> Observatory
```

**Arguments:**
- `obscode`: Standard observatory code (e.g., '500' for Geocenter)

**Returns:**
- New Observatory instance

*Example:*
```python
siding_spring = Observatory.from_obscode('413')
```

**`at()`**
```python
def at(self, epoch: Time, reference_plane: str = "J2000", 
       origin: str = "SSB") -> Observer
```

**Arguments:**
- `epoch`: Time of observation
- `reference_plane`: Reference frame (default: "J2000")
- `origin`: Origin of coordinate system (default: "SSB")

**Returns:**
- Observer instance at specified time

*Example:*
```python
observer = observatory.at(epoch, reference_plane="ECLIPJ2000")
```

### Observation Methods
---


**`from_astrometry()`**
```python
@classmethod
def from_astrometry(cls, epoch: Time, ra: float, dec: float, 
                   observer: Observer, mag: Optional[float] = None) -> Observation
```

**Arguments:**
- `epoch`: Observation time
- `ra`: Right ascension in radians
- `dec`: Declination in radians
- `observer`: Observer instance
- `mag`: Optional visual magnitude

**Returns:**
- New Observation instance

*Example:*
```python
obs = Observation.from_astrometry(epoch, ra, dec, observer, mag=20.5)
```

**`from_streak()`**
```python
@classmethod
def from_streak(cls, epoch: Time, ra: float, dec: float, 
               ra_rate: float, dec_rate: float, observer: Observer, 
               mag: Optional[float] = None) -> Observation
```

**Arguments:**
- `epoch`: Observation time
- `ra`: Right ascension in radians
- `dec`: Declination in radians
- `ra_rate`: RA rate in rad/day
- `dec_rate`: Dec rate in rad/day
- `observer`: Observer instance
- `mag`: Optional visual magnitude

**Returns:**
- New Observation instance

*Example:*
```python
obs = Observation.from_streak(epoch, ra, dec, ra_rate, dec_rate, observer)
```

<h2 style="border-bottom: 3px solid white;">Properties</h2>

### Observatory Properties
---
| Property | Type | Description |
|----------|------|-------------|
| `lat` | `Optional[float]` | Latitude in radians (ground-based only) |
| `lon` | `Optional[float]` | Longitude in radians (ground-based only) |
| `rho` | `Optional[float]` | Distance from geocenter in Earth radii |

### Observer Properties
---
| Property | Type | Description |
|----------|------|-------------|
| `position` | `numpy.ndarray` | 3D position vector [x, y, z] in AU |
| `velocity` | `numpy.ndarray` | 3D velocity vector [vx, vy, vz] in AU/day |
| `epoch` | `Time` | Observation epoch |
| `reference_plane` | `ReferencePlane` | Reference coordinate frame |
| `origin` | `Origin` | Origin of coordinate system |

### Observation Properties
---
| Property | Type | Description |
|----------|------|-------------|
| `ra` | `float` | Right ascension in radians |
| `dec` | `float` | Declination in radians |
| `ra_rate` | `Optional[float]` | RA rate in rad/day |
| `dec_rate` | `Optional[float]` | Dec rate in rad/day |
| `range` | `Optional[float]` | Range in AU |
| `range_rate` | `Optional[float]` | Range rate in AU/day |
| `mag` | `Optional[float]` | Visual magnitude |

<h2 style="border-bottom: 3px solid white;">Examples</h2>

### Basic SpaceRock Observation
```python
from spacerocks.observing import Observatory
from spacerocks.time import Time
from spacerocks import SpaceRock
from spacerocks.spice import SpiceKernel

# Initialize SPICE kernels
kernel = SpiceKernel()
kernel.load("latest_leapseconds.tls")
kernel.load("de440s.bsp")

# Create observatory and time
telescope = Observatory.from_obscode('F51')
epoch = Time.now()

# Create and configure target
target = SpaceRock.from_horizons(
    "Apophis", 
    epoch,
    reference_plane="J2000",
    origin='SSB'
)
target.set_absolute_magnitude(19.7)

# Get observation
observer = telescope.at(epoch)
obs = target.observe(observer)

print(f"RA: {obs.ra}")
print(f"Dec: {obs.dec}")
print(f"Range: {obs.range} AU")
print(f"Magnitude: {obs.mag}")
```

### Multi-Site Time Series
```python
# Set up observatories
mauna_kea = Observatory.from_obscode('568')
siding_spring = Observatory.from_obscode('413')

# Observe from both sites
obs_mk = target.observe(mauna_kea.at(epoch))
obs_sso = target.observe(siding_spring.at(epoch))

```

<h2 style="border-bottom: 3px solid white;">Notes</h2>

- Angles (RA, Dec, latitude, longitude) are consistently in radians
- Positions are in Astronomical Units (AU)
- Velocities use AU/day for orbital motions and rad/day for apparent motions
- Times are handled through the Time class in various formats
- Observatory codes follow the standard MPC/IAU format
- All coordinate transformations preserve precision through the use of SPICE
- When using streak observations, rates must be in proper spherical coordinates