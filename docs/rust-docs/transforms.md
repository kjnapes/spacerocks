# Orbital Transform Functions

### Table of Contents

1. [Overview](#overview)
2. [Constants and Types](#constants-and-types)
3. [Transform Methods](#transform-methods)
4. [Examples](#examples)

## Overview

The `transforms` module provides a collection of functions for orbital mechanics calculations, specifically focusing on conversions between different types of orbital anomalies and state vectors. These transformations are essential for accurate orbital predictions and calculations. Unless otherwise stated, all angular units are in radians, distances are in astronomical units (AU), and time is in days.

---

## Constants and Types

### Orbit Types

The module uses an `OrbitType` enum to classify different orbits based on their eccentricity:

```rust
pub enum OrbitType {
    Hyperbolic,   // e > 1
    Parabolic,    // e ≈ 1
    Elliptical,   // 0 < e < 1
    Circular,     // e ≈ 0
    Radial        // Special case
}
```

The orbit type is determined using eccentricity thresholds:

```rust
OrbitType::from_eccentricity(e: f64, threshold: f64) -> Result<OrbitType, OrbitError>
```

### Orbit Errors

The module defines an error type for handling invalid inputs or computational issues:

```rust
#[derive(Debug)]
pub enum OrbitError {
    NegativeEccentricity(f64),
}

impl std::fmt::Display for OrbitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrbitError::NegativeEccentricity(e) => write!(f, "Eccentricity cannot be negative: {}", e),
        }
    }
}

impl std::error::Error for OrbitError {}
```

---

## Transform Methods

### Anomaly Conversions

#### Mean Anomaly to Conic Anomaly

```rust
pub fn calc_conic_anomaly_from_mean_anomaly(
    e: f64,
    mean_anomaly: f64
) -> Result<f64, OrbitError>
```

Converts mean anomaly to the appropriate conic anomaly based on orbit type.

##### Input/Output Specification

| Input          | Type                      | Description                         |
| -------------- | ------------------------- | ----------------------------------- |
| `e`            | `f64`                     | Orbital eccentricity                |
| `mean_anomaly` | `f64`                     | Mean anomaly in radians             |
| **Returns**    | `Result<f64, OrbitError>` | Conic anomaly in radians (if valid) |

##### Errors

- `OrbitError::NegativeEccentricity`: Returned if the eccentricity `e` is negative.

##### Implementation Details

1. **Circular Orbits (****`e ≈ 0`****)**: Returns `mean_anomaly` directly.
2. **Elliptical Orbits (****`0 < e < 1`****)**: Uses Newton-Raphson iteration with third-order corrections for fast convergence.
3. **Parabolic Orbits (****`e ≈ 1`****)**: Applies an analytical formula.
4. **Hyperbolic Orbits (****`e > 1`****)**: Uses an iterative method to solve Kepler's equation.

##### Thresholds and Tolerances

- **Threshold for circularity (****`e < threshold`****)**: A small value, typically `1e-10`, determines if the orbit is effectively circular.
- **Convergence tolerance**: For iterative solutions, a tolerance of `1e-15` ensures accurate results without excessive computation.

##### Example

```rust
use spacerocks::transforms;

let e = 0.5;  // Elliptical orbit
let mean_anomaly = 0.5;  // radians
match transforms::calc_conic_anomaly_from_mean_anomaly(e, mean_anomaly) {
    Ok(eccentric_anomaly) => println!("Eccentric Anomaly: {:.6}", eccentric_anomaly),
    Err(e) => println!("Error: {}", e),
}
```

#### True Anomaly to Conic Anomaly

```rust
pub fn calc_conic_anomaly_from_true_anomaly(
    e: f64,
    true_anomaly: f64
) -> Result<f64, Box<dyn std::error::Error>>
```

Converts true anomaly to conic anomaly using orbit-specific formulas.

##### Input/Output Specification

| Input          | Type                                      | Description                         |
| -------------- | ----------------------------------------- | ----------------------------------- |
| `e`            | `f64`                                     | Orbital eccentricity                |
| `true_anomaly` | `f64`                                     | True anomaly in radians             |
| **Returns**    | `Result<f64, Box<dyn std::error::Error>>` | Conic anomaly in radians (if valid) |

##### Errors

- `OrbitError::NegativeEccentricity`: Returned if the eccentricity `e` is negative.

##### Implementation Details

1. **Circular Orbits (****`e ≈ 0`****)**: Returns `true_anomaly` directly.
2. **Elliptical Orbits (****`0 < e < 1`****)**: Uses trigonometric transformations.
3. **Parabolic Orbits (****`e ≈ 1`****)**: Applies the tangent half-angle formula.
4. **Hyperbolic Orbits (****`e > 1`****)**: Uses hyperbolic tangent transformations.

##### Example

```rust
use spacerocks::transforms;

let e = 0.3;  // Elliptical orbit
let true_anomaly = 1.2;  // radians
match transforms::calc_conic_anomaly_from_true_anomaly(e, true_anomaly) {
    Ok(eccentric_anomaly) => println!("Eccentric Anomaly: {:.6}", eccentric_anomaly),
    Err(e) => println!("Error: {}", e),
}
```

#### Conic Anomaly to Mean Anomaly

```rust
pub fn calc_mean_anomaly_from_conic_anomaly(
    e: f64,
    conic_anomaly: f64
) -> Result<f64, OrbitError>
```

Converts conic anomaly to mean anomaly using Kepler's equations.

##### Input/Output Specification

| Input           | Type                      | Description                        |
| --------------- | ------------------------- | ---------------------------------- |
| `e`             | `f64`                     | Orbital eccentricity               |
| `conic_anomaly` | `f64`                     | Conic anomaly in radians           |
| **Returns**     | `Result<f64, OrbitError>` | Mean anomaly in radians (if valid) |

##### Errors

- `OrbitError::NegativeEccentricity`: Returned if the eccentricity `e` is negative.

##### Implementation Details

- For circular orbits: `M = E`
- For elliptical orbits: `M = E - e sin(E)`
- For parabolic orbits: `M = B - B^3/3`
- For hyperbolic orbits: `M = e sinh(H) - H`

##### Example

```rust
use spacerocks::transforms;

let e = 0.5;  // Elliptical orbit
let conic_anomaly = 0.5;  // radians
match transforms::calc_mean_anomaly_from_conic_anomaly(e, conic_anomaly) {
    Ok(mean_anomaly) => println!("Mean Anomaly: {:.6}", mean_anomaly),
    Err(e) => println!("Error: {}", e),
}
```

---

## Examples

### Error Handling with Invalid Eccentricity

```rust
use spacerocks::transforms;

let e = -0.1;  // Invalid negative eccentricity
match transforms::calc_conic_anomaly_from_mean_anomaly(e, 0.0) {
    Ok(_) => println!("Valid calculation"),
    Err(e) => println!("Error: {}", e),
}
```

### Converting Mean Anomaly to True Anomaly

```rust
use spacerocks::transforms;

let e = 0.7;  // Elliptical orbit
let mean_anomaly = 0.8;  // radians
match transforms::calc_conic_anomaly_from_mean_anomaly(e, mean_anomaly) {
    Ok(eccentric_anomaly) => {
        let true_anomaly = transforms::calc_conic_anomaly_from_true_anomaly(e, eccentric_anomaly).unwrap();
        println!("True Anomaly: {:.6}", true_anomaly);
    }
    Err(e) => println!("Error: {}", e),
}
```

### State Vector to Keplerian Elements

```rust
use spacerocks::transforms;
use nalgebra::Vector3;

let position = Vector3::new(0.000047, 0.0, 0.0);  // AU
let velocity = Vector3::new(0.0, 0.000213, 0.0);  // AU/day
let mu = 2.959122082855911e-4;  // Gravitational parameter for the Sun in AU^3/day^2

```
