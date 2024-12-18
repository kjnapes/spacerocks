# Using the `Time` Class in Python

This document provides a guide to using the Python wrapper for the `Time` struct, exposing its functionality via a Python class. This class allows manipulation of astronomical and time-based calculations using Python, leveraging the underlying Rust implementation.

## Overview

The Python `Time` class supports:
- Creating time objects with specific epochs, timescales, and formats.
- Converting between different timescales.
- Parsing time from strings.
- Adding and subtracting days from time objects.
- Accessing various representations of time.

## Creating a `Time` Object

### Constructor

The `Time` class can be instantiated with an epoch, timescale, and format.

**Arguments:**
- `epoch`: A float representing the epoch (e.g., Julian Date).
- `timescale`: A string representing the timescale (e.g., "UTC", "TDB", "TT", "TAI").
- `format`: A string representing the format (e.g., "JD", "MJD").

All arguments are case-insensitive.

**Example:**
```python
from spacerocks import Time

# Create a Time object
 t = Time(2451545.0, "UTC", "jd")
```

### Using `now`

You can create a `Time` object representing the current time in UTC.
```python
 t = Time.now()
```

### Parsing from a Fuzzy String

Use the `from_fuzzy_str` method to parse a time string like "2451545.0 UTC JD" or "now".

**Example:**
```python
 t = Time.from_fuzzy_str("2451545.0 UTC JD")
```

---
## Inferring Time Format

### `infer_time_format`

You can create a `Time` object by inferring the format ("JD" or "MJD") based on the provided epoch.

**Arguments:**
- `epoch`: A float representing the epoch.
- `timescale`: An optional string representing the timescale (default is "UTC").

**Example:**
```python
# Infer format from epoch
 t = Time.infer_time_format(2451545.0, "UTC")
print(t.format)  # "JD"

# Infer format with no timescale provided
 t = Time.infer_time_format(50000.0, None)
print(t.format)  # "MJD"
```


## Timescale Conversions

### Immutable Conversions

The `Time` class supports creating new time objects with a converted timescale.

#### Convert to UTC
```python
utc_time = t.utc()
```

#### Convert to TDB
```python
tdb_time = t.tdb()
```

#### Convert to TT
```python
tt_time = t.tt()
```

#### Convert to TAI
```python
tai_time = t.tai()
```

### In-Place Conversions

Convert the timescale of a `Time` object in place:

#### Convert to UTC
```python
t.to_utc()
```

#### Convert to TDB
```python
t.to_tdb()
```

#### Convert to TT
```python
t.to_tt()
```

#### Convert to TAI
```python
t.to_tai()
```

#### Change Timescale Dynamically
You can change the timescale dynamically using the `change_timescale` method.

**Example:**
```python
t.change_timescale("UTC")
```

---

## Time Representations

### Julian Date (JD)
Retrieve the epoch as a Julian Date.

**Example:**
```python
jd = t.jd()
```

### Modified Julian Date (MJD)
Retrieve the epoch as a Modified Julian Date.

**Example:**
```python
mjd = t.mjd()
```

### ISO 8601 Format
Retrieve the time as an ISO 8601 formatted string.

**Example:**
```python
iso = t.iso()
```

### Calendar Date
Retrieve a human-readable calendar date.

**Example:**
```python
calendar_date = t.calendar()
```

---

## Arithmetic Operations

### Adding Days
Add a specified number of days to a `Time` object.

**Example:**
```python
new_time = t + 10.0
```

### Subtracting Days
Subtract a specified number of days from a `Time` object.

**Example:**
```python
new_time = t - 5.0
```

---

## Attributes

### Epoch
Retrieve the epoch value of the `Time` object.

**Example:**
```python
print(t.epoch)  # 2451545.0
```

### Timescale
Retrieve the timescale of the `Time` object.

**Example:**
```python
print(t.timescale)  # "UTC"
```

### Format
Retrieve the format of the `Time` object.

**Example:**
```python
print(t.format)  # "JD"
```

---

## Error Handling

### Invalid Timescale or Format
If an invalid `timescale` or `format` is provided, an error is raised with a suggestion for correction.

**Example:**
```python
try:
    t = Time(2451545.0, "utcx", "JD")
except ValueError as e:
    print(e)  # ValueError: Invalid timescale: 'utcx'. Did you mean 'utc'?. Needs to be 'utc', 'tdb', 'tt', or 'tai'.
```

---

## Special Methods

### String Representation
The `__repr__` method provides a string representation of the `Time` object.

**Example:**
```python
print(t)  # Time: 2451545.0 UTC JD
```

---

## Conclusion

The Python `Time` class simplifies time-based calculations and conversions, providing a seamless interface for astronomical and time-based operations.

