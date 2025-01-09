# `Simulation`

### Table of Contents
1. [Overview](#overview)
2. [Instantiation Methods](#instantiation-methods)
3. [Operation Methods](#operation-calculation-methods)
4. [Examples](#examples)


### Overview

The `Simulation` class is the way to run n-body simulations in `spacerocks`. 
For example, the following instantiates a new simulation, sets the epoch, reference plane, origin, and integrator.


```python
from spacerocks.nbody import Simulation, Integrator
from spacerocks.time import Time

epoch = Time.now()
reference_plane = "ECLIPJ2000"
origin = "ssb"

sim = Simulation()

sim.set_epoch(epoch)
sim.set_reference_plane(reference_plane)
sim.set_origin(origin)
sim.set_integrator(Integrator.ias15(timestep=20.0))
```

Now we can add bodies to the simulation. Here we add the Sun, Earth, and a custom body.
```python
from spacerocks import SpaceRock
from spacerocks.spice import SpiceKernel

# need to load a spicekernel so we can get the positions of the planets
kernel = SpiceKernel()
kernel.load("de440s.bsp")
kernel.load("latest_leapseconds.tls")

# The masses of the planets are handled under the hood.
sun = SpaceRock.from_spice("sun", epoch=epoch, reference_plane=reference_plane, origin=origin)
earth = SpaceRock.from_spice("earth", epoch=epoch, reference_plane=reference_plane, origin=origin)

# Create a custom body and set its mass
px = SpaceRock.from_kepler(name="px", q=200, e=0.6, inc=0.2, arg=2.4, node=5.6, true_anomaly=2.1, epoch=epoch, reference_plane=reference_plane, origin=origin)
px.set_mass(1.0e-5)

sim.add(sun)
sim.add(earth)
sim.add(px)
```
Now we need to move the simulation to the center of mass so that it doesn't drift. 
```python
sim.move_to_center_of_mass()
```
Finally, we can integrate the simulation to some other epoch.
```python
sim.integrate(epoch + 10.0) # integrate to 10 days in the future
```
Under the hood, a `Simulation` object keeps a hash map with references to all the bodies in the simulation. We can access particles individually using the `get_particle` method.
```python
earth_at_epoch = sim.get_particle("earth")
```



---
### Instantiation Methods

The following instantiation methods help to abstract away some of the logic for common scenarios. 

#### `giants`
Create a new simulation object with the Sun, Jupiter, Saturn, Uranus, and Neptune.
```python
Simulation.giants(epoch: Time, reference_plane: str, origin: str) -> Simulation
```

#### `planets`
Create a new simulation object with the Sun, Mercury, Venus, Earth, Moon, Mars, Jupiter, Saturn, Uranus, Neptune, and Pluto.
```python
Simulation.planets(epoch: Time, reference_plane: str, origin: str) -> Simulation
```

#### `horizons`
Create a new simulation object with the full set of JPL Horizons perturbers.
```python
Simulation.horizons(epoch: Time, reference_plane: str, origin: str) -> Simulation
```
