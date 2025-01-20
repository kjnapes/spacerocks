# `ReferencePlane`

To specify the position of a celestial body, both an origin and a reference plane are needed. A reference plane specifies the orientation of the coordinate system used for calculations of the position and motion of the body. `spacerocks` provides the `ReferencePlane` enum to represent commonly used reference planes in astronomy and celestial mechanics.

Each `ReferencePlane` is associated with a pre-defined rotation matrix that transforms coordinates into the desired frame of reference. This allows for seamless and efficient conversions between different celestial coordinate systems.

The following reference planes are available in `spacerocks`:

- **J2000**: Equatorial coordinates based on the J2000 epoch.
- **ECLIPJ2000** (default): Ecliptic coordinates based on the J2000 epoch.
- **INVARIABLE**: The invariable plane of the solar system, representing the angular momentum vector of the entire solar system.
- **GALACTIC**: Galactic coordinates, based on the orientation of the Milky Way galaxy.
- **FK4**: An older equatorial reference frame based on the B1950 epoch.

#### Examples
```python
from spacerocks.coordinates import ReferencePlane

# Use predefined reference planes
j2000 = ReferencePlane.J2000
ecliptic = ReferencePlane.ECLIPJ2000  # This is the default reference plane
galactic = ReferencePlane.GALACTIC

# Convert a string to a ReferencePlane
custom_plane = ReferencePlane.from_str("INVARIABLE")
print(custom_plane)  # Output: INVARIABLE

# Get the rotation matrix for a reference plane
rotation_matrix = ReferencePlane.GALACTIC.get_rotation_matrix()
```

#### Technical Details
`ReferencePlane` is implemented as a Rust enum. Each variant corresponds to a specific celestial reference frame and has an associated rotation matrix for coordinate transformations. The enum also provides utility methods for:

- **String Conversion**: Convert between strings and `ReferencePlane` values using `from_str` and `as_str`.
- **Rotation Matrix Access**: Obtain the rotation matrix for each reference plane using `get_rotation_matrix`.

