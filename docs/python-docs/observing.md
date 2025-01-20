# Observing 

### Table of Contents
1. [Overview](#overview)
2. [Observatory Class](#observatory-class)
3. [Observer Class](#observer-class)
4. [Observation Class](#observation-class)
5. [Examples](#examples)

## Overview

The SpaceRocks observing module provides tools for working with astronomical observations, including handling observatory locations, observer positions, and various types of observational data. The module consists of three main classes that work together:
- `Observatory`: Represents physical observation locations
- `Observer`: Represents an observatory at a specific time
- `Observation`: Handles different types of astronomical measurements

## Observatory Class

The Observatory class manages observation locations, including both ground-based observatories and space telescopes.

```python
class Observatory:
    @classmethod
    def from_obscode(cls, obscode: str) -> Observatory:
        """
        Create an Observatory instance from a standard observatory code.
        
        Args:
            obscode: Standard observatory code (e.g., '500' for Geocenter)
            
        Returns:
            Observatory instance
        
        Raises:
            ValueError: If observatory code is not found
        """
    
    def at(self, epoch: Time, reference_plane: str = "J2000", 
           origin: str = "SSB") -> Observer:
        """
        Get Observer instance at specific time.
        
        Args:
            epoch: Time of observation
            reference_plane: Reference frame (default: "J2000")
            origin: Origin of coordinate system (default: "SSB")
            
        Returns:
            Observer instance at specified time
        """
    
    @property
    def lat(self) -> Optional[float]:
        """Latitude in radians (ground-based only)"""
    
    @property
    def lon(self) -> Optional[float]:
        """Longitude in radians (ground-based only)"""
    
    @property
    def rho(self) -> Optional[float]:
        """Distance from geocenter in Earth radii (ground-based only)"""
```

## Observer Class

The Observer class represents an observatory at a specific time, providing access to its position and velocity.

```python
class Observer:
    @property
    def position(self) -> numpy.ndarray:
        """3D position vector [x, y, z] in AU"""
    
    @property
    def velocity(self) -> numpy.ndarray:
        """3D velocity vector [vx, vy, vz] in AU/day"""
    
    @property
    def origin(self) -> str:
        """Origin of coordinate system"""
    
    @property
    def reference_plane(self) -> str:
        """Reference coordinate frame"""
    
    @property
    def epoch(self) -> Time:
        """Observation epoch"""
    
    @property
    def lat(self) -> Optional[float]:
        """Observer latitude in radians (if applicable)"""
    
    @property
    def lon(self) -> Optional[float]:
        """Observer longitude in radians (if applicable)"""
    
    @property
    def rho(self) -> Optional[float]:
        """Observer distance from geocenter (if applicable)"""
```

## Observation Class

The Observation class handles different types of astronomical observations and their measurements.

### Constructor Methods

```python
class Observation:
    @classmethod
    def from_astrometry(cls, epoch: Time, ra: float, dec: float, 
                       observer: Observer, mag: Optional[float] = None) -> Observation:
        """
        Create an astrometric observation.
        
        Args:
            epoch: Observation time
            ra: Right ascension in radians
            dec: Declination in radians
            observer: Observer instance
            mag: Optional visual magnitude
            
        Returns:
            Observation instance
        """
    
    @classmethod
    def from_streak(cls, epoch: Time, ra: float, dec: float, 
                   ra_rate: float, dec_rate: float, observer: Observer, 
                   mag: Optional[float] = None) -> Observation:
        """
        Create a streak observation.
        
        Args:
            epoch: Observation time
            ra: Right ascension in radians
            dec: Declination in radians
            ra_rate: RA rate in rad/s
            dec_rate: Dec rate in rad/s
            observer: Observer instance
            mag: Optional visual magnitude
            
        Returns:
            Observation instance
        """
```

### Properties

```python
class Observation:
    @property
    def ra(self) -> float:
        """Right ascension in radians"""
    
    @property
    def dec(self) -> float:
        """Declination in radians"""
    
    @property
    def ra_rate(self) -> Optional[float]:
        """RA rate in rad/s"""
    
    @property
    def dec_rate(self) -> Optional[float]:
        """Dec rate in rad/s"""
    
    @property
    def range(self) -> Optional[float]:
        """Range in AU"""
    
    @property
    def range_rate(self) -> Optional[float]:
        """Range rate in AU/day"""
    
    @property
    def mag(self) -> Optional[float]:
        """Visual magnitude"""
        
    @property
    def epoch(self) -> Time:
        """Observation epoch"""
        
    @property
    def observer(self) -> Observer:
        """Observer that made the observation"""
```

## Examples

### Basic SpaceRock Observation

```python
from spacerocks.observing import Observatory
from spacerocks.time import Time
from spacerocks import SpaceRock
from spacerocks.spice import SpiceKernel

# Initialize SPICE kernels
kernel = SpiceKernel()
kernel.load("path/to/latest_leapseconds.tls")
kernel.load("path/to/de440s.bsp")

# Create observatory and time
w84 = Observatory.from_obscode('F51')  # Warrumbungle Observatory
epoch = Time.now()

# Create and configure SpaceRock
arrokoth = SpaceRock.from_horizons(
    "Arrokoth", 
    epoch,
    reference_plane="J2000",
    origin='SSB'
)
arrokoth.set_absolute_magnitude(11.0)

# Get observer and generate observation
observer = w84.at(epoch)
obs = arrokoth.observe(observer)

# Access observation details
print(f"Magnitude: {obs.mag}")
print(f"RA: {obs.ra} radians")
print(f"Dec: {obs.dec} radians")
print(f"Range: {obs.range} AU")

if obs.ra_rate is not None:
    print(f"RA rate: {obs.ra_rate} rad/s")
if obs.dec_rate is not None:
    print(f"Dec rate: {obs.dec_rate} rad/s")
```

### Observing from Multiple Sites

```python
# Initialize SPICE and time
kernel = SpiceKernel()
kernel.load("path/to/latest_leapseconds.tls")
kernel.load("path/to/de440s.bsp")
epoch = Time.now()

# Set up multiple observatories
mauna_kea = Observatory.from_obscode('568')
siding_spring = Observatory.from_obscode('413')

# Create and configure asteroid
bennu = SpaceRock.from_horizons(
    "Bennu",
    epoch,
    reference_plane="J2000",
    origin='SSB'
)
bennu.set_absolute_magnitude(20.5)

# Observe from different locations
obs_mk = bennu.observe(mauna_kea.at(epoch))
obs_sso = bennu.observe(siding_spring.at(epoch))

# Compare observations
print("Mauna Kea:")
print(f"RA: {obs_mk.ra}, Dec: {obs_mk.dec}")
print(f"Range: {obs_mk.range} AU")
print(f"Magnitude: {obs_mk.mag}")

print("\nSiding Spring:")
print(f"RA: {obs_sso.ra}, Dec: {obs_sso.dec}")
print(f"Range: {obs_sso.range} AU")
print(f"Magnitude: {obs_sso.mag}")
```

### Time Series Example

```python
from spacerocks.observing import Observatory
from spacerocks.time import Time
from spacerocks import SpaceRock
from spacerocks.spice import SpiceKernel

# Initialize SPICE
kernel = SpiceKernel()
kernel.load("path/to/latest_leapseconds.tls")
kernel.load("path/to/de440s.bsp")

# Set up time range
epoch_start = Time.from_iso("2024-01-01T00:00:00")
epoch_end = Time.from_iso("2024-01-02T00:00:00")
intervals = 24  # One observation per hour

# Set up observatory
telescope = Observatory.from_obscode('F51')
time_step = (epoch_end.jd() - epoch_start.jd()) / intervals

# Method 1: Using Horizons Ephemerides
observations = []
current_epoch = epoch_start

for i in range(intervals + 1):
    # Get new ephemeris at each time point
    # This gives the most accurate positions by querying Horizons,
    # but requires internet connection and is slower
    target = SpaceRock.from_horizons(
        "Apophis", 
        current_epoch,
        reference_plane="J2000",
        origin='SSB'
    )
    target.set_absolute_magnitude(19.7)
    
    # Get observer and observation
    observer = telescope.at(current_epoch)
    obs = target.observe(observer)
    observations.append(obs)
    
    print(f"Time: {current_epoch.iso()}")
    print(f"RA: {obs.ra}")
    print(f"Dec: {obs.dec}")
    print(f"Magnitude: {obs.mag}")
    
    current_epoch = Time.from_jd(current_epoch.jd() + time_step)

# Method 2: Using Analytic Propagation
observations = []
current_epoch = epoch_start

# Get initial state from Horizons
target = SpaceRock.from_horizons(
    "Apophis", 
    epoch_start,
    reference_plane="J2000",
    origin='SSB'
)
target.set_absolute_magnitude(19.7)

for i in range(intervals + 1):
    # Propagate the orbit analytically
    # This is faster and works offline
    propagated_target = target.analytic_at(current_epoch)
    
    # Get observer and observation
    observer = telescope.at(current_epoch)
    obs = propagated_target.observe(observer)
    observations.append(obs)
    
    print(f"Time: {current_epoch.iso()}")
    print(f"RA: {obs.ra}")
    print(f"Dec: {obs.dec}")
    print(f"Magnitude: {obs.mag}")

    
    current_epoch = Time.from_jd(current_epoch.jd() + time_step)
```

### Unit Notes

- Angles (RA, Dec, latitude, longitude): radians
- Positions: AU (Astronomical Units)
- Velocities: AU/day for orbital motions, rad/s for apparent motions
- Times: Various formats through Time class
- Observatory codes: Standard MPC/IAU codes