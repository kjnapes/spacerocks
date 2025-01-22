<h1 style="border-bottom: 5px solid white;">Transforms Module</h1>

### Table of Contents
1. [Overview](#overview)
2. [Primary Functions](#primary-functions)
3. [Methods](#methods)
4. [Examples](#examples)
5. [Notes](#notes)

<h2 style="border-bottom: 3px solid white;">Overview</h2>

The Transforms module provides functions for orbital mechanics calculations, focusing on conversions between different types of orbital anomalies and state vectors. All angular units are in radians, distances in astronomical units (AU), and time in days.

<h2 style="border-bottom: 3px solid white;">Primary Functions</h2>

The module provides functions for converting between different representations of orbital position:
- Mean anomaly (M)
- Conic anomaly: Generalization of eccentric anomaly for all orbit types
  - For elliptical orbits: Eccentric anomaly (E)
  - For hyperbolic orbits: Hyperbolic anomaly (H)
  - For parabolic orbits: Parabolic anomaly (B)
  - For circular orbits: Equal to true anomaly
- True anomaly (ν): Actual angle of object from periapsis

Note that "conic anomaly" is used throughout as a unified term for what is traditionally called "eccentric anomaly" in elliptical orbits, allowing the same functions to work across all orbit types.

<h2 style="border-bottom: 3px solid white;">Methods</h2>

### Anomaly Conversions
---

**`calc_conic_anomaly_from_mean_anomaly()`**
```python
def calc_conic_anomaly_from_mean_anomaly(e: float, mean_anomaly: float) -> float
```
**Arguments:**
- `e`: Orbital eccentricity
- `mean_anomaly`: Mean anomaly in radians

**Returns:**
- Conic anomaly in radians

*Example:*
```python
E = calc_conic_anomaly_from_mean_anomaly(0.5, 1.0)
```

**`calc_conic_anomaly_from_true_anomaly()`**
```python
def calc_conic_anomaly_from_true_anomaly(e: float, true_anomaly: float) -> float
```
**Arguments:**
- `e`: Orbital eccentricity
- `true_anomaly`: True anomaly in radians

**Returns:**
- Conic anomaly in radians

*Example:*
```python
E = calc_conic_anomaly_from_true_anomaly(0.5, 1.0)
```

**`calc_mean_anomaly_from_conic_anomaly()`**
```python
def calc_mean_anomaly_from_conic_anomaly(e: float, conic_anomaly: float) -> float
```
**Arguments:**
- `e`: Orbital eccentricity
- `conic_anomaly`: Conic anomaly in radians

**Returns:**
- Mean anomaly in radians

*Example:*
```python
M = calc_mean_anomaly_from_conic_anomaly(0.5, 1.0)
```

**`calc_true_anomaly_from_conic_anomaly()`**
```python
def calc_true_anomaly_from_conic_anomaly(e: float, conic_anomaly: float) -> float
```
**Arguments:**
- `e`: Orbital eccentricity
- `conic_anomaly`: Conic anomaly in radians

**Returns:**
- True anomaly in radians

*Example:*
```python
nu = calc_true_anomaly_from_conic_anomaly(0.5, 1.0)
```

**`calc_true_anomaly_from_mean_anomaly()`**
```python
def calc_true_anomaly_from_mean_anomaly(e: float, mean_anomaly: float) -> float
```
**Arguments:**
- `e`: Orbital eccentricity
- `mean_anomaly`: Mean anomaly in radians

**Returns:**
- True anomaly in radians

*Example:*
```python
nu = calc_true_anomaly_from_mean_anomaly(0.5, 1.0)
```

### Universal Kepler Solver
--- 

**`solve_for_universal_anomaly()`**
```python
def solve_for_universal_anomaly(r: float, vr: float, alpha: float, mu: float, 
                              dt: float, tol: float, max_iter: int) -> float
```
**Arguments:**
- `r`: Initial radius in AU
- `vr`: Radial velocity in AU/day
- `alpha`: Reciprocal of semi-major axis (-2E/μ)
- `mu`: Gravitational parameter in AU³/day²
- `dt`: Time interval in days
- `tol`: Solution tolerance
- `max_iter`: Maximum number of iterations

**Returns:**
- Universal anomaly value

*Example:*
```python
chi = solve_for_universal_anomaly(1.0, 0.1, -1.0, 1.0, 1.0, 1e-12, 1000)
```

***Note:*** This is used within the `analytic_propagate()` method of SpaceRock

### Stumpff Functions
---

***Note:*** These are used internally for calculating the universal anomaly. You can read more about them (and much else relating to orbital mechanics!) [here](https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/universal-variables.html).

**`stumpff_c()`**
```python
def stumpff_c(z: float) -> float
```
**Arguments:**
- `z`: Input argument

**Returns:**
- Value of Stumpff C function

*Example:*
```python
c = stumpff_c(0.5)
```

**`stumpff_s()`**
```python
def stumpff_s(z: float) -> float
```
**Arguments:**
- `z`: Input argument

**Returns:**
- Value of Stumpff S function

*Example:*
```python
s = stumpff_s(0.5)
```

<h2 style="border-bottom: 3px solid white;">Examples</h2>

### Anomaly Conversions
```python
from spacerocks.transforms import *

# Convert from mean to true anomaly for an elliptical orbit
e = 0.7  # eccentricity
M = 0.8  # mean anomaly

# First get eccentric anomaly
E = calc_conic_anomaly_from_mean_anomaly(e, M)

# Then convert to true anomaly
nu = calc_true_anomaly_from_conic_anomaly(e, E)
```

### Universal Kepler Problem
```python
from spacerocks.transforms import solve_for_universal_anomaly

# Solve for position after time interval
r = 1.0    # initial radius (AU)
vr = 0.1   # radial velocity (AU/day)
alpha = -1.0  # -2E/μ
mu = 1.0    # gravitational parameter
dt = 1.0    # time interval (days)

chi = solve_for_universal_anomaly(r, vr, alpha, mu, dt, 1e-12, 1000)
```

<h2 style="border-bottom: 3px solid white;">Notes</h2>

- All angles are in radians
- All distances are in astronomical units (AU)
- All times are in days
- Velocities are in AU/day
- Functions handle different orbit types:
  - Circular (e ≈ 0)
  - Elliptical (0 < e < 1)
  - Parabolic (e ≈ 1)
  - Hyperbolic (e > 1)
- Invalid inputs (e.g., negative eccentricity) raise ValueError
- Universal Kepler solver uses Stumpff functions for efficiency
- For elliptical orbits, conic anomaly is the eccentric anomaly
- For hyperbolic orbits, conic anomaly is the hyperbolic anomaly