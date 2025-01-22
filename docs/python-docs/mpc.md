<h1 style="border-bottom: 5px solid white;">MPC Module</h1>

### Table of Contents
1. [Overview](#overview)
2. [Primary Classes](#primary-classes)
3. [Methods](#methods)
4. [Examples](#examples)
5. [Notes](#notes)

<h2 style="border-bottom: 3px solid white;">Overview</h2>

The MPC module provides functionality for fetching and managing data from the Minor Planet Center. It supports downloading orbital elements, physical parameters, and converting data into different formats.

<h2 style="border-bottom: 3px solid white;">Primary Classes</h2>

### MPCHandler
```python
class MPCHandler:
    """
    Core class for handling MPC data downloads and conversions.
    
    Example:
        handler = MPCHandler()
        data = handler.fetch_data("mpcorb_extended", orbit_type="MBA")
    """
```

<h2 style="border-bottom: 3px solid white;">Methods</h2>

### Constructor Methods
---

**`new()`**

**Arguments:**
- `path`: Optional path for storing MPC data (defaults to ~/.spacerocks/mpc)

**Returns:**
- New MPCHandler instance

*Example:*
```python
handler = MPCHandler()  # Use default path
handler = MPCHandler("/custom/path")  # Use custom path
```

### Data Methods
---

**`fetch_data()`**
```python
def fetch_data(self, catalog: str, orbit_type: Optional[str] = None, 
               storage_format: Optional[str] = "feather", 
               output_format: str = "dataframe") -> Union[pd.DataFrame, RockCollection]
```
**Arguments:**
- `catalog`: Name of the MPC catalog to fetch (e.g., "MPCORB")
- `orbit_type`: Optional filter for specific orbit types (e.g., "MBA", "NEA")
- `storage_format`: Format to store data ("feather", "json", or None)
- `output_format`: Format to return data ("dataframe" or "rocks")

**Returns:**
- Either a pandas DataFrame or RockCollection depending on output_format

*Example:*
```python
# Get asteroid data as DataFrame
mba_df = handler.fetch_data("mpcorb_extended", orbit_type="MBA")

# Get as RockCollection
nea_rockcollection = handler.fetch_data("nea_extended", output_format="rocks")
```

**`create_rock_collection()`**
```python
@classmethod
def create_rock_collection(cls, mpc_path: PathBuf, catalog: str, 
                         download_data: bool, 
                         orbit_type: Optional[str] = None) -> RockCollection
```
**Arguments:**
- `mpc_path`: Path for storing MPC data
- `catalog`: Name of the MPC catalog
- `download_data`: Whether to download new data
- `orbit_type`: Optional filter for specific orbit types

**Returns:**
- RockCollection containing the requested objects

*Example:*
```python
rocks = MPCHandler.create_rock_collection(
    "path/to/data",
    "mpcorb_extended",
    download_data=True,
    orbit_type="MBA"
)
```

<h2 style="border-bottom: 3px solid white;">Examples</h2>

### Basic Usage
```python
from spacerocks.mpc import MPCHandler

# Create handler
handler = MPCHandler()

# Get main-belt asteroids as DataFrame
amor_df = handler.fetch_data(
    catalog="nea_extended",
    orbit_type="Apollo",
    storage_format="feather"
)

# Print summary
print(f"Found {len(amor_df)} Amor orbit-type near earth asteroids")
print(f"Mean semi-major axis: {amor_df['a'].mean():.2f} AU")
```

### Creating SpaceRock Objects
```python
# Get data as RockCollection
trojans = handler.fetch_data(
    catalog="mpcorb_extended",
    orbit_type="Jupiter Trojans",
    output_format="rocks"
)

# Work with individual objects
for rock in rocks:
    if rock.a() < 5.0: 
        print(f"{rock.name}: {rock.a():.2f} AU")
```

<h2 style="border-bottom: 3px solid white;">Notes</h2>

- Downloads are cached in ~/.spacerocks/mpc by default
- Feather format provides faster I/O than JSON
- Available orbit types include:
    - Atira
    - Aten
    - Apollo
    - Amor
    - Hungaria
    - MBA
    - Phocaea
    - Hilda
    - Jupiter Trojan
    - Distant Object
- RockCollection output automatically includes H magnitudes and G slopes
- Data is automatically downloaded if not found in cache
- Large datasets are processed in parallel for better performance
- All orbital elements are in standard MPC units (AU, degrees)