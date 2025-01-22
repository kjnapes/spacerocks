<h1 style="border-bottom: 5px solid white;">OrbitFit Module</h1>

### Table of Contents
1. [Overview](#overview)
2. [Primary Functions](#primary-functions)
3. [Methods](#methods)
4. [Examples](#examples)
5. [Notes](#notes)

<h2 style="border-bottom: 3px solid white;">Overview</h2>

The OrbitFit module provides a function for determining orbital elements from observational data. It implements Gauss's method for initial orbit determination using three observations of a celestial body.

<h2 style="border-bottom: 3px solid white;">Primary Functions</h2>

The module implements Gauss's method, which:
- Takes three observations of a celestial body
- Solves for the object's distance at each observation
- Determines a unique orbit that fits the observations
- Accounts for light-time correction in the solution

<h2 style="border-bottom: 3px solid white;">Methods</h2>

### Orbit Determination

**`gauss()`**
```python
def gauss(o1: Observation, o2: Observation, o3: Observation, 
          min_distance: float) -> List[SpaceRock]
```
**Arguments:**
- `o1`: First observation
- `o2`: Second observation
- `o3`: Third observation
- `min_distance`: Minimum allowed distance

**Returns:**
- List of possible SpaceRock objects fitting the observations

**Raises:**
- `ValueError`: If no valid solutions are found

*Example:*
```python
rocks = gauss(obs1, obs2, obs3, min_distance=0.1)
```

<h2 style="border-bottom: 3px solid white;">Examples</h2>

### Basic Usage
```python
from spacerocks.orbfit import gauss
from spacerocks.observing import Observer
from spacerocks.time import Time

# Create or load three observations
time1 = Time.now()
time2 = time1 + 150
time3 = time2 + 150

observer1 = Observatory.from_obscode('w84').at(time1)
observer2 = Observatory.from_obscode('w84').at(time2)
observer3 = Observatory.from_obscode('w84').at(time3)

obs1 = Observation.from_astrometry(epoch, ra1, dec1, observer1)
obs2 = Observation.from_astrometry(epoch, ra2, dec2, observer2)
obs3 = Observation.from_astrometry(epoch, ra3, dec3, observer3)



# Find possible orbits
solutions = gauss(obs1, obs2, obs3, min_distance=0.1)

# Examine solutions
for rock in solutions:
    print(f"Semi-major axis: {rock.a()} AU")
    print(f"Eccentricity: {rock.e()}")
```

### Error Handling
```python
try:
    solutions = gauss(obs1, obs2, obs3, min_distance=0.1)
    if not solutions:
        print("No physically meaningful solutions found")
except ValueError as e:
    print(f"Failed to find orbit: {e}")
```

<h2 style="border-bottom: 3px solid white;">Notes</h2>

- Observations should be well-spaced in time 
- The method accounts for:
  - Earth's motion around the solar system barycenter
  - Light-time corrections
  - Multiple possible solutions
- Observations must include:
  - Time of observation
  - Observer's position
  - Direction to object (RA/Dec)
- The min_distance parameter helps filter non-physical solutions
- Solution uniqueness improves with better-spaced observations
- Solutions are returned ordered by likelihood
- Empty list indicates no physical solutions found
- All distances are in AU
- All angles are in radians