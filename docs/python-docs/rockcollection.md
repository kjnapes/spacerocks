# `RockCollection`

### Table of Contents
1. [Overview](#overview)
2. [Attributes](#attributes)
3. [Instantiation Methods](#instantiation-methods)
4. [Operation Methods](#operation-methods)
5. [Getter Methods](#getter-methods)

---

### Overview
The `RockCollection` class is used to manage and manipulate a collection of `SpaceRock` objects. This includes operations such as observing, filtering, and fetching orbital data from external sources like the Minor Planet Center (MPC).

---

### Attributes

| Attribute | Type | Description |
| --- | --- | --- |
| `rocks` | `List[SpaceRock]` | A list of `SpaceRock` objects contained in the collection. |

---

### Instantiation Methods

#### `new`
Create an empty `RockCollection`.
```python
collection = RockCollection()
```

#### `from_mpc`
Create a `RockCollection` by fetching data from the Minor Planet Center (MPC).
```python
collection = RockCollection.from_mpc(mpc_path: str, catalog: str, download_data: bool)
```
**Parameters**:
- `mpc_path` (`str`): Directory where MPC data will be stored.
- `catalog` (`str`): Name of the MPC catalog to fetch (e.g., `mpcorb_extended`).
- `download_data` (`bool`): If `True`, downloads the data if not already present.

**Returns**:  
A `RockCollection` instance populated with data from the specified MPC catalog.

---

### Operation Methods

#### `add`
Add a `SpaceRock` to the collection.
```python
collection.add(rock: SpaceRock)
```
**Parameters**:
- `rock` (`SpaceRock`): The `SpaceRock` object to add.

**Example**:
```python
from spacerocks import RockCollection, SpaceRock

# Create an empty collection
collection = RockCollection()

# Create a SpaceRock and add it to the collection
rock = SpaceRock.from_xyz("Custom Rock", [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], epoch="J2000", origin="sun", reference_plane="ECLIPJ2000")

collection.add(rock)

print(len(collection))  # Output: 1
```

#### `observe`
Observe all `SpaceRock` objects in the collection from the perspective of an observer.
```python
observations = collection.observe(observer: Observer)
```
**Parameters**:
- `observer` (`Observer`): The observing entity.

**Returns**:  
A list of `Observation` objects representing the observations of each `SpaceRock`.

**Example**:
```python
from spacerocks import RockCollection
from spacerocks.time import Time
from spacerocks.observing import Observatory


# Create a RockCollection
collection = RockCollection()

t = Time.now()
rock1 = SpaceRock.from_horizons("Arrokoth", t, reference_plane="J2000", origin='SSB')
rock2 =  SpaceRock.from_horizons("Ceres", t, reference_plane="J2000", origin='SSB')

collection.add(rock1)
collection.add(rock2)

# Create an observer
obscode = 'W84'
w84 = Observatory.from_obscode(obscode)
observer = w84.at(t, reference_plane="J2000", origin="ssb")

# Observe the rocks
observations = collection.observe(observer)
ra_vals = observations.ra
dec_vals = observations.dec


print(f"Number of observations: {len(observations)}")
```

#### `change_reference_plane`
Change the reference plane of all `SpaceRock` objects in the collection.
```python
collection.change_reference_plane(reference_plane: str)
```
**Parameters**:
- `reference_plane` (`str`): The new reference plane (e.g., `"J2000"`).


---

### Getter Methods

#### `x`, `y`, `z`
Get the spatial coordinates of all `SpaceRock` objects in the collection.
```python
x_coords = collection.x
y_coords = collection.y
z_coords = collection.z
```

#### `vx`, `vy`, `vz`
Get the components of velocity for all `SpaceRock` objects.
```python
vx_coords = collection.vx
vy_coords = collection.vy
vz_coords = collection.vz
```

#### `epoch`
Get the epoch of each `SpaceRock` in the collection.
```python
epochs = collection.epoch
```
---

