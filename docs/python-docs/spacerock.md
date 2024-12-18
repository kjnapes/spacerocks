# `SpaceRock`

### Table of Contents
1. [Overview](#overview)
2. [Attributes](#attributes)
3. [Instantiation Methods](#instantiation-methods)
4. [Operation Methods](#operation-calculation-methods)
5. [Setter Methods](#setter-methods)
6. [Getter Methods](#getter-methods)
7. [Examples](#examples)


### Overview

---

### Attributes
| Attribute | Type | Description |
| --- | --- | --- |
| `name` | `str` | Name of the SpaceRock. |
| `position` | `np.ndarray` | Position of the SpaceRock in au. |
| `velocity` | `np.ndarray` | Velocity of the SpaceRock in au/day. |
| `epoch` | `Time` | Epoch of the SpaceRock. |
| `origin` | `Origin` | Origin of the SpaceRock. |
| `reference plane` | `ReferencePlane` | Reference plane of the SpaceRock. |
| `properties` | `Properties` | Properties of the SpaceRock. |

---
### Instantiation Methods

#### `from_horizons`
Create a SpaceRock object from JPL Horizons data.
```python
SpaceRock.from_horizons(name: str, epoch: Time, reference_plane: str, origin: str) -> SpaceRock
```

#### `from_spice`
Create a SpaceRock object from a SPICE kernel.
```python
SpaceRock.from_spice(name: str, epoch: Time, reference_plane: str, origin: str) -> SpaceRock
```

#### `from_xyz`
Create a SpaceRock object from position and velocity data.
```python
SpaceRock.from_xyz(name: str, position: np.ndarray, velocity: np.ndarray, epoch: Time, origin: str, reference_plane: str) -> SpaceRock
```

#### `from_spherical`
Create a SpaceRock object using the coordinate basis described in Napier and Holman (2024).
```python
SpaceRock.from_spherical(name: str, r: float, theta: float, phi: float, vr: float, vo: float, psi: float, epoch: Time, origin: str, reference_plane: str) -> SpaceRock
```

---

### Operation and Calculation Methods

#### `observe`
Calculate the ephemeris of the SpaceRock object, given an observer.
```python
rock.observe(observer: Observer) -> Observation
```

#### `change_reference_plane`
Change the reference plane of the SpaceRock object.
```python
rock.change_reference_plane(reference_plane: str) -> None
```

#### `change_origin`
Change the origin of the SpaceRock object.
```python
rock.change_origin(origin: str) -> None
```

---

### Setter Methods

These methods allow you to modify a SpaceRock object after it has been created. The general usage pattern is 
```python
rock.set_property(value)
```
where `property` is the name of the property you want to set, and `value` is the value you want to set it to. A list of the setter methods is given below.

| Method | Returns | Description |
| --- | --- | --- |
| `set_absolute_magnitude` | `None` | Set the absolute magnitude (H) of a SpaceRock object. |
| `set_albedo` | `None` | Set the albedo of a SpaceRock object. |
| `set_gslope` | `None` | Set the G-slope of a SpaceRock object. |
| `set_mass` | `None` | Set the mass of a SpaceRock object in units of solar masses. |
| `set_radius` | `None` | Set the radius of a SpaceRock object in km. |
| `set_x` | `None` | Set the x-coordinate of the SpaceRock in au. |
| `set_y` | `None` | Set the y-coordinate of the SpaceRock in au. |
| `set_z` | `None` | Set the z-coordinate of the SpaceRock in au. |
| `set_vx` | `None` | Set the x-component of the velocity of the SpaceRock in au/day. |
| `set_vy` | `None` | Set the y-component of the velocity of the SpaceRock in au/day. |
| `set_vz` | `None` | Set the z-component of the velocity of the SpaceRock in au/day. |


---
### Getter Methods

These methods allow you to access the properties of a SpaceRock object. The general usage pattern is 
```python
rock.get_property()
```
where `property` is the name of the property you want to access. A list of the getter methods is given below.

| Method | Returns | Description |
| --- | --- | --- |
| `absolute_magnitude` | `float` or `None` | Get the absolute magnitude (H) of the SpaceRock. |
| `albedo` | `float` or `None` | Get the albedo of the SpaceRock. |
| `gslope` | `float` or `None` | Get the G-slope of the SpaceRock. |
| `mass` | `float` or `None` | Get the mass of the SpaceRock in units of solar masses. |
| `radius` | `float` or `None` | Get the radius of the SpaceRock in km. |
| `x` | `float` | Get the x-coordinate of the SpaceRock in au. |
| `y` | `float` | Get the y-coordinate of the SpaceRock in au. |
| `z` | `float` | Get the z-coordinate of the SpaceRock in au. |
| `vx` | `float` | Get the x-component of the velocity of the SpaceRock in au/day. |
| `vy` | `float` | Get the y-component of the velocity of the SpaceRock in au/day. |
| `vz` | `float` | Get the z-component of the velocity of the SpaceRock in au/day. |

---

### Examples
```python
from spacerocks import SpaceRock
from spacerocks.time import Time

epoch = Time.now()

arrokoth = SpaceRock.from_horizons("Arrokoth", epoch, "ECLIPJ2000", "ssb")
```