<h1 style="border-bottom: 5px solid white;">RockCollection Module</h1>


### Table of Contents
1. [Overview](#overview)
2. [Primary Classes](#primary-classes)
3. [Methods](#methods)
4. [Properties](#properties)
5. [Examples](#examples)
6. [Notes](#notes)

<h2 style="border-bottom: 3px solid white;">Overview</h2>


The RockCollection module provides tools for managing and manipulating groups of SpaceRock objects. It supports batch operations like observing multiple objects, filtering collections, and fetching orbital data from external sources such as the Minor Planet Center (MPC).

<h2 style="border-bottom: 3px solid white;">Primary Classes</h2>


### RockCollection
```python
class RockCollection:
    """
    Manages collections of SpaceRock objects with batch operations support.
    
    Example:
        collection = RockCollection()
        collection.add(rock)
        observations = collection.observe(observer)
    """
```

<h2 style="border-bottom: 3px solid white;">Methods</h2>


### Constructor Methods
---
**`new()`**

**Returns:**
- New RockCollection instance

*Example:*
```python
collection = RockCollection()
```

**`from_mpc()`**
```python
@classmethod
def from_mpc(cls, mpc_path: str, catalog: str, 
             download_data: bool = True) -> RockCollection
```

**Arguments:**
- `mpc_path`: Directory for MPC data storage
- `catalog`: Name of MPC catalog (e.g., 'mpcorb_extended')
- `download_data`: Whether to download missing data from MPC
- `orbit_type`: Specify only certain types of objects, i.e. "Atira"

**Returns:**
- RockCollection populated with MPC data

*Example:*
```python
trojan_collection = RockCollection.from_mpc(
    mpc_path="/path/to/store/data",
    catalog="mpcorb_extended",
    download_data=True, 
    orbit_type="Jupiter Trojan"
)
```

### Operation Methods
---

**`add()`**
```python
def add(self, rock: SpaceRock) -> None
```

**Arguments:**
- `rock`: SpaceRock object to add

*Example:*
```python
collection = RockCollection()
rock = SpaceRock.from_horizons("Ceres", epoch, "ECLIPJ2000", "SSB")
collection.add(rock)
```

**`observe()`**
```python
def observe(self, observer: Observer) -> List[Observation]
```

**Arguments:**
- `observer`: Observer object representing viewing location

**Returns:**
- List of Observation objects

*Example:*
```python
observatory = Observatory.from_obscode("F51")
observer = observatory.at(epoch)

observations = collection.observe(observer)
```

**`filter()`**


```python
def filter(self, indices: List[bool]) -> RockCollection
```

**Arguments**
- indices: List of boolean values. Must have the same length as the number of rocks.

**Returns**
- A new RockCollection containing only selected rocks

*Example*
```python
collection_under_5_au = collection.filter(collection.a() < 5)
```

**`change_reference_plane()`**
```python
def change_reference_plane(self, reference_plane: str) -> None
```

**Arguments:**
- `reference_plane`: New reference plane identifier

*Example:*
```python
collection.change_reference_plane("ECLIPJ2000")
```

<h2 style="border-bottom: 3px solid white;">Properties</h2>


### Position and Velocity Properties
| Property | Type | Description |
|----------|------|-------------|
| `x`, `y`, `z` | `numpy.ndarray` | Position components in AU |
| `vx`, `vy`, `vz` | `numpy.ndarray` | Velocity components in AU/day |
| `epoch` | `List[Time]` | Epochs of all objects |
| `rocks` | `List[SpaceRock]` | All SpaceRock objects |

<h2 style="border-bottom: 3px solid white;">Examples</h2>


### Basic Usage
```python
from spacerocks import RockCollection, SpaceRock
from spacerocks.time import Time

# Create collection and add objects
collection = RockCollection()
epoch = Time.now()

ceres = SpaceRock.from_horizons("Ceres", epoch, "ECLIPJ2000", "SSB")
vesta = SpaceRock.from_horizons("Vesta", epoch, "ECLIPJ2000", "SSB")

collection.add(ceres)
collection.add(vesta)

print(f"Collection size: {len(collection.rocks)}")
```

### Observing Multiple Objects
```python
from spacerocks.observing import Observatory

# Set up observer
observatory = Observatory.from_obscode('F51')
observer = observatory.at(epoch)

# Get observations for all objects
observations = collection.observe(observer)

# Process observations
for rock, obs in zip(collection.rocks, observations):
    print(f"{rock.name}:")
    print(f"  RA: {obs.ra}")
    print(f"  Dec: {obs.dec}")
    print(f"  Magnitude: {obs.mag}")
```
### Filtering Rocks
```python
# Filter rocks with eccentricity greater than 0.1
mask = [rock.e() > 0.05 for rock in collection]
filtered_collection = collection.filter(mask)

print(f"Filtered collection size: {len(filtered_collection.rocks)}")
```

### Loading from MPC
```python
# Create collection from MPC data
collection = RockCollection.from_mpc(
    mpc_path="~/mpc_data",
    catalog="mpcorb_extended",
    download_data=True
)

# Change reference plane for all objects
collection.change_reference_plane("J2000")
```

<h2 style="border-bottom: 3px solid white;">Notes</h2>


- Collections maintain the order of added objects
- All position values are in AU
- All velocity values are in AU/day
- MPC data is stored locally when downloaded
- Compatible with all SpaceRock instantiation methods
- Empty collections can be created with RockCollection()

