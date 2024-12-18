# `SpaceRock`

### Table of Contents
1. [Overview](#overview)
2. [Attributes](#attributes)
3. [Instantiation Methods](#instantiation-methods)
4. [Operation Methods](#operation-methods)
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

### Operation Methods

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

#### `set_absolute_magnitude`
Set the absolute magnitude (H) of a SpaceRock object.
```python
rock.set_absolute_magnitude(absolute_magnitude: float) -> None
```

#### `set_albedo`
Set the albedo of a SpaceRock object.
```python
rock.set_albedo(albedo: float) -> None
```

#### `set_gslope`
Set the G-slope of a SpaceRock object.
```python
rock.set_gslope(gslope: float) -> None
```

#### `set_mass`
Set the mass of a SpaceRock object in units of solar masses.
```python
rock.set_mass(mass: float) -> None
```

#### `set_radius`
Set the radius of a SpaceRock object in km.
```python
rock.set_radius(radius: float) -> None
```

#### `set_x`
Set the x-coordinate of the SpaceRock in au.
```python
rock.set_x(x: float) -> None
```

#### `set_y`
Set the y-coordinate of the SpaceRock in au.
```python
rock.set_y(y: float) -> None
```

#### `set_z`
Set the z-coordinate of the SpaceRock in au.
```python
rock.set_z(z: float) -> None
```

#### `set_vx`
Set the x-component of the velocity of the SpaceRock in au/day.
```python
rock.set_vx(vx: float) -> None
```

#### `set_vy`
Set the y-component of the velocity of the SpaceRock in au/day.
```python
rock.set_vy(vy: float) -> None
```

#### `set_vz`
Set the z-component of the velocity of the SpaceRock in au/day.
```python
rock.set_vz(vz: float) -> None
```


---
### Getter Methods

#### `x`
Get the x-coordinate of the SpaceRock in au.
```python
rock.x() -> float
```

#### `y`
Get the y-coordinate of the SpaceRock in au.
```python
rock.y() -> float
```

#### `z`
Get the z-coordinate of the SpaceRock in au.
```python
rock.z() -> float
```

#### `vx`
Get the x-component of the velocity of the SpaceRock in au/day.
```python
rock.vx() -> float
```

#### `vy`
Get the y-component of the velocity of the SpaceRock in au/day.
```python
rock.vy() -> float
```

#### `vz`
Get the z-component of the velocity of the SpaceRock in au/day.
```python
rock.vz() -> float
```

#### `mass`
Get the mass of the SpaceRock in solar masses.
```python
rock.mass() -> float
```

#### `radius`
Get the radius of the SpaceRock in km.
```python
rock.radius() -> float
```

#### `absolute_magnitude`
Get the absolute magnitude (H) of the SpaceRock.
```python
rock.absolute_magnitude() -> float
```

#### `albedo`
Get the albedo of the SpaceRock.
```python
rock.albedo() -> float
```

#### `gslope`
Get the G-slope of the SpaceRock.
```python
rock.gslope() -> float
```

---

### Examples
```python
from spacerocks import SpaceRock
from spacerocks.time import Time

epoch = Time.now()

arrokoth = SpaceRock.from_horizons("Arrokoth", epoch, "ECLIPJ2000", "ssb")
```