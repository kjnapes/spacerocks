# Time Module (Python)

### Table of Contents
1. [Overview](#overview)
2. [Time Class](#time-class)
3. [Constructor Methods](#constructor-methods)
4. [Time Conversions](#time-conversions)
5. [Format Methods](#format-methods)
6. [Arithmetic Operations](#arithmetic-operations)
7. [Display](#display)
8. [Notes](#notes)

## Overview

The Time module provides a Python interface for handling astronomical time calculations. It serves as a wrapper around the Rust implementation, supporting multiple time scales (UTC, TDB, TT, TAI) and formats (JD, MJD), with conversion utilities and arithmetic operations.

## Time Class

### Time Definition
```python
class Time:
    def __init__(self, epoch: float, timescale: str, format: str):
        self.epoch = epoch
        self.timescale = timescale
        self.format = format
```

The `Time` class represents a moment in time with an epoch value in a specific timescale and format.

## Constructor Methods

#### __init__()
```python
def __init__(self, epoch: float, timescale: str, format: str)
```

Creates a new Time instance with specified epoch, timescale, and format.

**Arguments:**
- `epoch`: The epoch value (JD or MJD)
- `timescale`: The timescale ("UTC", "TDB", "TT", "TAI")
- `format`: The time format ("JD", "MJD")

**Example:**
```python
# Create a time in UTC using Julian Date
t1 = Time(2451545.0, "UTC", "JD")

# Create a time in TDB using Modified Julian Date
t2 = Time(51544.5, "TDB", "MJD")

# Invalid timescale raises error with suggestion
try:
    t3 = Time(2451545.0, "GPS", "JD")
except ValueError as e:
    print(e)  # Invalid timescale: 'GPS'. Did you mean 'tai'?
```

#### now()
```python
@classmethod
def now(cls) -> 'Time'
```

Creates a new Time instance representing the current time.

**Returns:**
- A new `Time` instance in UTC and JD format

**Example:**
```python
current_time = Time.now()
print(f"Current JD: {current_time.jd()}")
```

#### from_fuzzy_str()
```python
@classmethod
def from_fuzzy_str(cls, s: str) -> 'Time'
```

Creates a Time instance from a string specification.

**Arguments:**
- `s`: String in format "epoch timescale format" or "now"

**Example:**
```python
# Create from explicit string
t1 = Time.from_fuzzy_str("2451545.0 UTC JD")

# Create using current time
t2 = Time.from_fuzzy_str("now")

# Create using MJD
t3 = Time.from_fuzzy_str("51544.5 TDB MJD")
```

#### infer_time_format()
```python
@classmethod
def infer_time_format(cls, epoch: float, timescale: Optional[str] = None) -> 'Time'
```

Creates a Time instance by inferring the format from the epoch value.

**Arguments:**
- `epoch`: The epoch value
- `timescale`: Optional timescale (defaults to "UTC")

**Example:**
```python
# Large epoch infers JD format
t1 = Time.infer_time_format(2451545.0, "UTC")
assert t1.format == "JD"

# Small epoch infers MJD format
t2 = Time.infer_time_format(51544.5)
assert t2.format == "MJD"
```

## Time Conversions

### Creating New Time Objects

#### utc()
```python
def utc(self) -> 'Time'
```

Creates a new Time object converted to UTC timescale.

**Example:**
```python
tdb_time = Time(2456205.5, "TDB", "JD")
utc_time = tdb_time.utc()
assert tdb_time.epoch != utc_time.epoch  # Epochs differ due to timescale conversion
```

Similar methods exist for other timescales: `tdb()`, `tt()`, and `tai()`.

### In-Place Conversions

#### to_utc()
```python
def to_utc(self) -> 'Time'
```

Converts the time object to UTC timescale in place.

**Example:**
```python
time = Time(2456205.5, "TDB", "JD")
time.to_utc()
assert time.timescale == "UTC"

# Chain operations
time.to_utc().to_tdb().to_tt()
```

Similar methods exist for other timescales: `to_tdb()`, `to_tt()`, and `to_tai()`.

## Format Methods

#### jd()
```python
def jd(self) -> float
```

Returns the epoch as a Julian Date.

**Example:**
```python
time = Time(51544.5, "UTC", "MJD")
jd = time.jd()  # Returns 2451545.0
```

#### mjd()
```python
def mjd(self) -> float
```

Returns the epoch as a Modified Julian Date.

**Example:**
```python
time = Time(2451545.0, "UTC", "JD")
mjd = time.mjd()  # Returns 51544.5
```

#### calendar()
```python
def calendar(self) -> str
```

Returns a human-readable calendar date.

**Example:**
```python
time = Time(2451545.0, "UTC", "JD")
date = time.calendar()  # Returns "01 Jan 2000"
```

#### iso()
```python
def iso(self) -> str
```

Returns the time in ISO 8601 format.

**Example:**
```python
time = Time(2451545.0, "UTC", "JD")
iso = time.iso()  # Returns "2000-01-01T12:00:00.000Z"
```

## Arithmetic Operations

```python
# Add days to time
time = Time(2451545.0, "UTC", "JD")
tomorrow = time + 1.0  # Adds one day
assert tomorrow.epoch == 2451546.0

# Subtract days from time
yesterday = time - 1.0  # Subtracts one day
assert yesterday.epoch == 2451544.0

# Time differences not supported in Python implementation
```

## Display

### String Representation (__repr__)
```python
def __repr__(self) -> str
```

Provides a string representation of the Time object when displayed in the Python interpreter.

**Example:**
```python
>>> time = Time(2451545.0, "UTC", "JD")
>>> time
Time: 2451545.0 UTC JD

>>> t2 = Time.now()
>>> t2
Time: 2460252.5 UTC JD

>>> print(time)  # Same representation when printing
Time: 2451545.0 UTC JD
```

## Notes

1. **Timescale Handling**
   - All timescale conversions maintain precision
   - Case-insensitive timescale strings
   - Invalid timescale strings receive helpful suggestions

2. **Format Conversions**
   - JD/MJD conversions handle offset automatically
   - Calendar format provides human-readable dates
   - ISO format follows the 8601 standard

3. **Error Handling**
   - Invalid inputs raise ValueError with suggestions
   - Helpful error messages for common mistakes
   - Type checking for numeric inputs

4. **Python-Specific Features**
   - Properties for easy attribute access
   - Method chaining supported for conversions
   - Full string representation via `__repr__`