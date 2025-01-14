# Errors 

The `spacerocks` library defines custom error enums to handle specific failure cases encountered during a variety computations. These error types provide detailed information about the nature of the error, making debugging and error handling more straightforward.

---

## **`OrbitError`**
Errors related to orbital computations.

### Variants:
- **`NegativeEccentricity(f64)`**  
  Raised when an orbit's eccentricity is negative.  
  **Example Message:**  
  `"Eccentricity cannot be negative: -0.5"`

- **`ConvergenceFailure(f64, f64)`**  
  Raised when solving Kepler's equation fails to converge.  
  **Parameters:**  
  - `f64`: The eccentricity of the orbit.  
  - `f64`: The mean anomaly.  
  **Example Message:**  
  `"Failed to converge for eccentricity 0.9 and mean anomaly 3.14"`

---

## **`OriginError`**
Errors related to invalid origins.

### Variants:
- **`InvalidOrigin(String)`**  
  Raised when an invalid origin is specified.  
  **Example Message:**  
  `"Invalid origin: UnknownOrigin"`

---

## **`ReferencePlaneError`**
Errors related to invalid reference planes.

### Variants:
- **`InvalidReferencePlane(String)`**  
  Raised when an invalid reference plane is specified.  
  **Example Message:**  
  `"Invalid origin: UnknownPlane"`

---

## **`SimulationError`**
Errors encountered during simulation of celestial systems.

### Variants:
- **`OriginMismatch(Origin, Origin, String)`**  
  Raised when the origin of a particle does not match the simulation origin and is not found in the perturbers.  
  **Example Message:**  
  `"The origin of the particle Earth (Sun) did not match the simulation origin (SSB), and was not found in perturbers."`

- **`EpochMismatch(Time, Time, String)`**  
  Raised when the epoch of a particle does not match the simulation epoch.  
  **Example Message:**  
  `"The epoch of particle Earth (2451545.0) did not match the simulation epoch (2451544.5)."`

- **`ParticleNotFound(String)`**  
  Raised when a particle is not found in the simulation.  
  **Example Message:**  
  `"The particle Earth was not found in the simulation."`

---

## **`TimeError`**
Errors related to time computations.

### Variants:
- **`InvalidTimeScale(String)`**  
  Raised when an invalid timescale is specified.  
  **Example Message:**  
  `"Invalid timescale: 'xyz'. Needs to be 'utc', 'tdb', 'tt', or 'tai'."`

- **`InvalidTimeFormat(String)`**  
  Raised when an invalid time format is specified.  
  **Example Message:**  
  `"Invalid time format: 'unknown'. Needs to be 'jd' or 'mjd'."`

---

