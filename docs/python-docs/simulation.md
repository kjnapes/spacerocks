# `Simulation`

### Table of Contents
1. [Overview](#overview)
2. [Instantiation Methods](#instantiation-methods)
3. [Operation Methods](#operation-calculation-methods)
4. [Examples](#examples)


### Overview

```python
from spacerocks.nbody import Simulation, Integrator

sim = Simulation()

sim.set_epoch(epoch)
sim.set_reference_plane("ECLIPJ2000")
sim.set_origin('ssb')
sim.set_integrator(Integrator.ias15(timestep=20.0))
```

---
### Instantiation Methods

#### `giants`
Create a new simulation object.
```python
Simulation.giants() -> Simulation
```

#### `planets`
Create a new simulation object.
```python
Simulation.planets() -> Simulation
```

#### `horizons`
Create a new simulation object.
```python
Simulation.horizons() -> Simulation
```

#### `new`
Create a new simulation object.
```python
Simulation.new() -> Simulation
```
