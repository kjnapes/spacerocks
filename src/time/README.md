# Working with `Time` Objects in Rust

This document provides a detailed guide on how to use the `Time` struct in Rust for managing and manipulating astronomical and time-based calculations.

## Overview

The `Time` struct represents a point in time with the following attributes:

- **epoch**: The numeric representation of time, either as Julian Date (JD) or Modified Julian Date (MJD).
- **timescale**: The time reference frame (UTC, TDB, TT, TAI).
- **format**: The representation of time (JD, MJD).

The `Time` struct supports:
- Varius methods for initialization.
- Timescale conversions.
- Arithmetic operations.

---

## Struct Definition

```rust
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Time {
    pub epoch: f64,
    pub timescale: TimeScale,
    pub format: TimeFormat,
}
```

---

## Creating a `Time` Object

### `Time::new`

Creates a new `Time` object.

**Arguments:**
- `epoch`: Epoch of the time (JD or MJD).
- `timescale`: Timescale (e.g., `"UTC"`, `"TDB"`).
- `format`: Format (e.g., `"JD"`, `"MJD"`).

**Example:**
```rust
let t = Time::new(2451545.0, "utc", "jd").unwrap();
```

If an invalid `timescale` or `format` is provided, an error is returned with a suggestion for the correct value. The `timescale` and `format` values are case-insensitive.

---

### `Time::now`

Creates a `Time` object representing the current time in UTC and JD format.

**Example:**
```rust
let t = Time::now();
```

---

### `Time::from_fuzzy_str`

Parses a `Time` object from a string of the format `"epoch timescale format"`. The string can also be `"now"` to represent the current time.

**Example:**
```rust
let t = Time::from_fuzzy_str("2451545.0 UTC JD").unwrap();
```

---

### `Time::infer_time_format`

Creates a `Time` object by inferring the format (`JD` or `MJD`) based on the provided epoch.

**Example:**
```rust
let t = Time::infer_time_format(2451545.0, Some("UTC")).unwrap();
```

---

## Timescale Conversions

The `Time` struct supports conversion between different timescales (`UTC`, `TDB`, `TT`, and `TAI`).

### Immutable Conversions

#### `Time::utc`
Converts the `Time` object to UTC.

**Example:**
```rust
let utc_time = tdb_time.utc();
```

#### `Time::tdb`
Converts the `Time` object to TDB.

**Example:**
```rust
let tdb_time = utc_time.tdb();
```

#### `Time::tt`
Converts the `Time` object to TT.

**Example:**
```rust
let tt_time = tdb_time.tt();
```

#### `Time::tai`
Converts the `Time` object to TAI.

**Example:**
```rust
let tai_time = utc_time.tai();
```

### In-Place Conversions

#### `Time::to_utc`
Modifies the `Time` object in place to UTC.

**Example:**
```rust
let mut time = Time::new(2451545.0, "TDB", "JD").unwrap();
time.to_utc();
```

#### `Time::to_tdb`
Modifies the `Time` object in place to TDB.

**Example:**
```rust
let mut time = Time::new(2451545.0, "UTC", "JD").unwrap();
time.to_tdb();
```

---

## Time Representations

### `Time::jd`
Returns the epoch as a Julian Date (JD).

**Example:**
```rust
let jd = time.jd();
```

### `Time::mjd`
Returns the epoch as a Modified Julian Date (MJD).

**Example:**
```rust
let mjd = time.mjd();
```

### `Time::iso`
Converts the epoch to an ISO 8601 formatted string.

**Example:**
```rust
let iso = time.iso();
```

### `Time::calendar`
Converts the epoch to a human-readable calendar date.

**Example:**
```rust
let calendar_date = time.calendar();
```

---

## Arithmetic Operations


### Addition and Subtraction with Scalars
You can add or subtract a scalar value of days to or from a `Time` object; 

**Example:**
```rust
let mut time = Time::new(2451545.0, "UTC", "JD").unwrap();
time += 10.0; // Adds 10 days to the epoch
time -= 5.0;  // Subtracts 5 days from the epoch
```

### Operations Between Time Objects
You can also directly compare the epochs of two `Time` objects manually.

**Example:**
```rust
let diff = time1.epoch - time2.epoch; // Difference in epochs
```

---

## Error Handling

### Invalid Timescale or Format
When an invalid `timescale` or `format` is provided, a suggestion is given based on string similarity.

**Example:**
```rust
let result = Time::new(2451545.0, "UTCX", "JD");
assert!(result.is_err()); 

// Invalid timescale: 'utcx'. Did you mean 'utc'?. Needs to be 'utc', 'tdb', 'tt', or 'tai'.
```

---

## Parsing Strings with Fuzzy Matching

The `Time` struct uses the **Damerau-Levenshtein** algorithm to suggest corrections for invalid `timescale` or `format` inputs.

---

## Conclusion

The `Time` struct provides a robust and user-friendly interface for working with time calculations and astronomical data.

