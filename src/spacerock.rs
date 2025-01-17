use crate::{Origin, ReferencePlane, Time, Properties, Observer, Observation};
use crate::constants::*;
use crate::correct_for_ltt;
use crate::OrbitType;

use crate::transforms::{calc_conic_anomaly_from_true_anomaly, calc_mean_anomaly_from_conic_anomaly};

use serde::{Serialize, Deserialize};
use nalgebra::Vector3;

use rand;
use rand::Rng;

use std::collections::HashMap;

// use uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpaceRock {

    pub name: String,
    pub epoch: Time,

    pub reference_plane: ReferencePlane,
    pub origin: Origin,

    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,

    pub properties: Option<Properties>,
}

/// A SpaceRock is a celestial object with a position and velocity in space. It can be instantiated from a spice kernel, 
/// random keplerian elements, cartesian coordinates, spherical coordinates, or the JPL Horizons API. It can be propagated
/// in time, and observed from an observer on Earth. It can also be transformed to the solar system barycenter or the heliocenter.
impl SpaceRock {

    /// Instantiate a SpaceRock from a spice kernel. A kernel must be loaded before calling this method.
    ///
    /// # Arguments
    /// * `name` - The name of the object
    /// * `epoch` - The epoch of the ephemeris
    /// * `reference_plane` - The coordinate reference_plane of the ephemeris
    /// * `origin` - The origin of the orbit
    ///
    /// # Returns
    /// * A SpaceRock object
    ///
    /// # Example
    /// ```
    /// use spacerocks::SpaceRock;
    /// use spacerocks::Time;
    ///
    /// let epoch = Time::now();
    /// let rock = SpaceRock::from_spice("Earth", &epoch, "J2000", "SSB");
    /// ```
    pub fn from_spice(name: &str, epoch: &Time, reference_plane: &str, origin: &str) -> Result<Self, Box<dyn std::error::Error>> {

        // check a priori if the name is in the list of loaded kernels

        let reference_plane = ReferencePlane::from_str(reference_plane)?;
        let origin = Origin::from_str(origin)?;

        // let mut ep = epoch.clone();
        let et = spice::str2et(&format!("JD{epoch} UTC", epoch=epoch.utc().jd()));
        let (state, _) = spice::spkezr(name, et, reference_plane.as_str(), "NONE", &origin.to_string());
        let position = Vector3::new(state[0], state[1], state[2]) * KM_TO_AU;
        let velocity = Vector3::new(state[3], state[4], state[5]) * KM_TO_AU * SECONDS_PER_DAY;

        let mut rock = SpaceRock {
            name: name.to_string(),
            position,
            velocity,
            epoch: epoch.clone(),
            reference_plane,
            origin,
            properties: None,
        };

        if let Some(m) = MASSES.get(name.to_lowercase().as_str()) { rock.set_mass(*m) };

        Ok(rock)

    }

    /// Instantiate a SpaceRock with random keplerian elements
    ///
    /// # Arguments
    /// * `epoch` - The epoch of the ephemeris
    /// * `reference_plane` - The coordinate reference_plane of the ephemeris
    /// * `origin` - The origin of the orbit
    ///
    /// # Returns
    /// * A SpaceRock object
    ///
    /// # Example
    /// ```
    /// use spacerocks::SpaceRock;
    /// use spacerocks::Time;
    ///
    /// let epoch = Time::now();
    /// let rock = SpaceRock::random(&epoch, "J2000", "SSB");
    /// ```
    pub fn random(epoch: &Time, reference_plane: &str, origin: &str) -> Result<Self, Box<dyn std::error::Error>> {

        let mut rng = rand::thread_rng();
        let q = rng.gen_range(2.0..50.0);
        let e = rng.gen_range(0.0..1.5);
        let inc = rng.gen_range(0.0..std::f64::consts::PI);
        let arg = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
        let node = rng.gen_range(0.0..2.0 * std::f64::consts::PI);

        // let max_true_anomaly = ((-1.0 / e) as f64).acos();
        let mut max_true_anomaly = 2.0 * std::f64::consts::PI;
        if e > 1.0 {
            max_true_anomaly = ((-1.0 / e) as f64).acos();
        }

        let true_anomaly = rng.gen_range(-max_true_anomaly..max_true_anomaly);

        // let name = format!("{}", uuid::Uuid::new_v4().simple());
        let name = format!("{}", generate_name(2, 4));


        let rock = SpaceRock::from_kepler(&name, q, e, inc, arg, node, true_anomaly, epoch.clone(), reference_plane, origin)?;
        Ok(rock)
    }

    /// Instantiate a SpaceRock from cartesian coordinates
    ///
    /// # Arguments
    /// * `name` - The name of the object
    /// * `x` - The x-coordinate of the object (au)
    /// * `y` - The y-coordinate of the object (au)
    /// * `z` - The z-coordinate of the object (au)
    /// * `vx` - The x-component of the velocity (au/day)
    /// * `vy` - The y-component of the velocity (au/day)
    /// * `vz` - The z-component of the velocity (au/day)
    /// * `epoch` - The epoch of the ephemeris
    /// * `reference_plane` - The coordinate reference_plane of the ephemeris
    /// * `origin` - The origin of the orbit
    ///
    /// # Returns
    /// * A SpaceRock object
    ///
    /// # Example
    /// ```
    /// use spacerocks::SpaceRock;
    /// use spacerocks::Time;
    ///
    /// let epoch = Time::now();
    /// let rock = SpaceRock::from_xyz("Arrokoth", 43.0, 0.0, 0.0, 0.0, 0.0, 0.0, epoch, "J2000", "SSB");
    /// ```
    pub fn from_xyz(name: &str, x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64, epoch: Time, reference_plane: &str, origin: &str) -> Result<Self, Box<dyn std::error::Error>> {

        let reference_plane = ReferencePlane::from_str(reference_plane)?;
        let origin = Origin::from_str(origin)?;

        let position = Vector3::new(x, y, z);
        let velocity = Vector3::new(vx, vy, vz);
        let rock = SpaceRock {
                name: name.to_string(),
                position,
                velocity,
                epoch,
                reference_plane: reference_plane.clone(),
                origin: origin.clone(),
                properties: None,
        };

        Ok(rock)
    }

    /// Get a SpaceRock from the JPL Horizons API
    ///
    /// # Arguments
    /// * `name` - The name of the object 
    /// * `epoch` - The epoch of the ephemeris
    /// * `reference_plane` - The coordinate reference_plane of the ephemeris
    /// * `origin` - The origin of the orbit
    ///
    /// # Returns
    /// * A SpaceRock object
    ///
    /// # Example
    /// ```
    /// use spacerocks::SpaceRock;
    /// use spacerocks::Time;
    ///
    /// let epoch = Time::now();
    /// let rock = SpaceRock::from_horizons("Arrokoth", &epoch, "J2000", "SSB");
    /// ```
    pub fn from_horizons(name: &str, epoch: &Time, reference_plane: &str, origin: &str) -> Result<Self, Box<dyn std::error::Error>> {

        let client = reqwest::blocking::Client::new();

        let mut params = HashMap::new();

        let command_str = format!("'{}'", name);
        params.insert("command", command_str.as_str());

        let mut ep = epoch.clone();

        let timescale = &ep.timescale.to_str().to_uppercase();
        let timeformat = &ep.format.to_str().to_uppercase();

        match reference_plane.to_uppercase().as_str() {
            "J2000" => {
                params.insert("ref_system", "'J2000'");
                params.insert("ref_plane", "'frame'");
            },
            "ECLIPJ2000" => {
                params.insert("ref_system", "'J2000'");
                params.insert("ref_plane", "'ecliptic'");
            },
            _ => {
                return Err("Frame not recognized".into());
            }
        }


        // if timescale == "UTC" {
        //     params.insert("TIME_TYPE", "'UT'");
        // } else {
        //     params.insert("TIME_TYPE", timescale);
        // }

        ep.to_tdb();
        params.insert("TIME_TYPE", "'TDB'");

        let time_list = format!("'{}'", ep.epoch);
        params.insert("TLIST", time_list.as_str());

        let tf = format!("'{}'", timeformat);
        params.insert("TLIST_TYPE", tf.as_str());

        let center = format!("'@{}'", origin);
        params.insert("center", center.as_str());

        params.insert("make_ephem", "'yes'");
        params.insert("ephem_type", "'vectors'");
        params.insert("vec_corr", "'None'");
        params.insert("out_units", "'AU-D'");
        params.insert("csv_format", "'yes'");
        params.insert("vec_delta_t", "'no'");
        params.insert("vec_table", "'2x'");
        params.insert("vec_labels", "'no'");

        let response = client.get("https://ssd.jpl.nasa.gov/api/horizons.api?")
            .query(&params)
            .send()?;

        let json: serde_json::Value = response.json()?;
        let text = json["result"].as_str();

        // println!("{:?}", text);

        let lines: Vec<&str> = text.ok_or("No data")?.split('\n').collect();
        let first_data_line = lines.iter().skip_while(|&line| !line.starts_with("$$SOE")).nth(1).ok_or("No data")?;
        
        let data: Vec<f64> = first_data_line.split(',').filter_map(|s| s.trim().parse::<f64>().ok()).collect();
        // let given_epoch = Time::new(data[0], "tdb", "jd")?;
        let (x, y, z, vx, vy, vz) = (data[1], data[2], data[3], data[4], data[5], data[6]);

        let rock = SpaceRock::from_xyz(name, x, y, z, vx, vy, vz, epoch.clone(), reference_plane, origin)?;
        Ok(rock)
    }

    /// Instantiate a SpaceRock from spherical coordinates (Napier and Holman 2024)
    ///
    /// # Arguments
    /// * `name` - The name of the object
    /// * `phi` - Longitude (radians)
    /// * `theta` - Latutude (radians)
    /// * `r` - Distance from the origin (au)
    /// * `vr` - Radial velocity (au/day)
    /// * `vo` - Tangential velocity (au/day)
    /// * `psi` - Angle between the radial and tangential velocities (radians)
    /// * `epoch` - The epoch of the ephemeris
    /// * `reference_plane` - The coordinate reference_plane of the ephemeris
    /// * `origin` - The origin of the orbit
    ///
    /// # Returns
    /// * A SpaceRock object
    pub fn from_spherical(name: &str, phi: f64, theta: f64, r: f64, vr: f64, vo: f64, psi: f64, epoch: Time, reference_plane: &str, origin: &str) -> Result<Self, Box<dyn std::error::Error>> {

        let pointing = Vector3::new(phi.cos() * theta.cos(), phi.sin() * theta.cos(), theta.sin());
        let position = pointing * r;

        let dhat = Vector3::new(-phi.cos() * theta.sin(), -phi.sin() * theta.sin(), theta.cos());
        let ahat = Vector3::new(-phi.sin(), phi.cos(), 0.0);
        let velocity = pointing * vr + vo * (psi.cos() * ahat + psi.sin() * dhat);

        let x = position.x;
        let y = position.y;
        let z = position.z;
        let vx = velocity.x;
        let vy = velocity.y;
        let vz = velocity.z;

        let rock = SpaceRock::from_xyz(name, x, y, z, vx, vy, vz, epoch, reference_plane, origin)?;
        Ok(rock)
    }


    /// Instantiate a SpaceRock from keplerian elements
    ///
    /// # Arguments
    /// * `name` - The name of the object
    /// * `q` - Perihelion distance (au)
    /// * `e` - Eccentricity
    /// * `inc` - Inclination (radians)
    /// * `arg` - Argument of perihelion (radians)
    /// * `node` - Longitude of the ascending node (radians)
    /// * `true_anomaly` - True anomaly (radians)
    /// * `epoch` - The epoch of the ephemeris
    /// * `reference_plane` - The coordinate reference_plane of the ephemeris
    /// * `origin` - The origin of the orbit
    ///
    /// # Returns
    /// * A SpaceRock object
    pub fn from_kepler(name: &str, q: f64, e: f64, inc: f64, arg: f64, node: f64, true_anomaly: f64, epoch: Time, reference_plane: &str, origin: &str) -> Result<Self, Box<dyn std::error::Error>> {

        // first check that the eccentricity and true anomaly are commensurate
        if e >= 1.0 {
            let max_true_anomaly = (-1.0 / e).acos();
            if true_anomaly.abs() > max_true_anomaly {
                return Err("True anomaly is not commensurate with eccentricity".into());
            }
        }

        let o = Origin::from_str(origin)?;
        let mu = o.mu();

        let p = q * (1.0 + e);
        let h = (p * mu).sqrt();
        let r = p / (1.0 + e * true_anomaly.cos());
        let vr = mu * true_anomaly.sin() * e / h;

        let rot_x = node.cos() * (arg + true_anomaly).cos() - node.sin() * (arg + true_anomaly).sin() * inc.cos();
        let rot_y = node.sin() * (arg + true_anomaly).cos() + node.cos() * (arg + true_anomaly).sin() * inc.cos();
        let rot_z = (arg + true_anomaly).sin() * inc.sin();

        let x = r * rot_x;
        let y = r * rot_y;
        let z = r * rot_z;

        let rot_x2 = node.cos() * (arg + true_anomaly).sin() + node.sin() * (arg + true_anomaly).cos() * inc.cos();
        let rot_y2 = node.sin() * (arg + true_anomaly).sin() - node.cos() * (arg + true_anomaly).cos() * inc.cos();
        let rot_z2 = (arg + true_anomaly).cos() * inc.sin();

        let nudot = h / r.powi(2);
        let vx = vr * rot_x - r * nudot * rot_x2;
        let vy = vr * rot_y - r * nudot * rot_y2;
        let vz = vr * rot_z + r * nudot * rot_z2;

        let rock = SpaceRock::from_xyz(name, x, y, z, vx, vy, vz, epoch, reference_plane, origin)?;
        Ok(rock)
    }

    /// Change the reference plane of the SpaceRock
    ///
    /// # Arguments
    /// * `reference_plane` - The new reference plane
    pub fn change_reference_plane(&mut self, reference_plane: &str) -> Result<(), Box<dyn std::error::Error>> {

        let reference_plane = ReferencePlane::from_str(reference_plane)?;
        if reference_plane == self.reference_plane {
            return Ok(());
        }

        let inv = self.reference_plane.get_rotation_matrix().try_inverse().ok_or("Could not invert rotation matrix")?;
        let rot = reference_plane.get_rotation_matrix() * inv;

        self.position = rot * self.position;
        self.velocity = rot * self.velocity;
        self.reference_plane = reference_plane;

        Ok(())
    }

    /// Change the origin of the SpaceRock
    ///
    /// # Arguments
    /// * `origin` - The SpaceRock object to change the origin to
    pub fn change_origin(&mut self, origin: &SpaceRock) {

        let origin_position = origin.position;
        let origin_velocity = origin.velocity;

        self.position -= origin_position;
        self.velocity -= origin_velocity;

        self.origin = Origin::new_custom(origin.mass() * GRAVITATIONAL_CONSTANT, &origin.name);
    }

    /// Change the origin of the SpaceRock to the solar system barycenter
    /// 
    /// # Example
    /// ```
    /// use spacerocks::SpaceRock;
    /// use spacerocks::Time;
    ///
    /// let epoch = Time::now();
    /// let rock = SpaceRock::from_horizons("Arrokoth", &epoch, "J2000", "SSB");
    /// rock.to_ssb();
    /// ```
    pub fn to_ssb(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // get the ssb from spice
        let mut ssb = SpaceRock::from_spice("ssb", &self.epoch, self.reference_plane.as_str(), self.origin.as_str())?;
        ssb.set_mass(MU_BARY / GRAVITATIONAL_CONSTANT);
        self.change_origin(&ssb);
        Ok(())
    }

    /// Change the origin of the SpaceRock to the heliocenter
    ///
    /// # Example
    /// ```
    /// use spacerocks::SpaceRock;
    /// use spacerocks::Time;
    ///
    /// let epoch = Time::now();
    /// let rock = SpaceRock::from_horizons("Arrokoth", &epoch, "J2000", "SSB");
    /// rock.to_helio();
    /// ```
    pub fn to_helio(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // get the sun from spice
        let sun = SpaceRock::from_spice("sun", &self.epoch, self.reference_plane.as_str(), self.origin.as_str())?;
        self.change_origin(&sun);
        Ok(())
    }

    pub fn r_squared(&self) -> f64 {
        self.position.dot(&self.position)
    }

    pub fn v_squared(&self) -> f64 {
        self.velocity.dot(&self.velocity)
    }

    pub fn v(&self) -> f64 {
        self.velocity.norm()
    }

    pub fn set_mass(&mut self, mass: f64) {
        if self.properties.is_none() {
            self.properties = Some(Properties::default());
        }
        self.properties.as_mut().unwrap().mass = Some(mass);
    }

    pub fn mass(&self) -> f64 {
        match &self.properties {
            Some(p) => p.mass.unwrap_or(0.0),
            None => 0.0,
        }
    }

    pub fn absolute_magnitude(&self) -> f64 {
        match &self.properties {
            Some(p) => p.absolute_magnitude.unwrap_or(0.0),
            None => 0.0,
        }
    }

    pub fn gslope(&self) -> f64 {
        match &self.properties {
            Some(p) => p.gslope.unwrap_or(0.15),
            None => 0.15,
        }
    }

    pub fn radius(&self) -> f64 {
        match &self.properties {
            Some(p) => p.radius.unwrap_or(0.0),
            None => 0.0,
        }
    }

    pub fn albedo(&self) -> f64 {
        match &self.properties {
            Some(p) => p.albedo.unwrap_or(0.0),
            None => 0.0,
        }
    }

    pub fn set_absolute_magnitude(&mut self, absolute_magnitude: f64) {
        if self.properties.is_none() {
            self.properties = Some(Properties::default());
        }
        self.properties.as_mut().unwrap().absolute_magnitude = Some(absolute_magnitude);
        self.properties.as_mut().unwrap().gslope = Some(0.15);
    }

    pub fn set_gslope(&mut self, gslope: f64) {
        if self.properties.is_none() {
            self.properties = Some(Properties::default());
        }
        self.properties.as_mut().unwrap().gslope = Some(gslope);
    }

    pub fn set_radius(&mut self, radius: f64) {
        if self.properties.is_none() {
            self.properties = Some(Properties::default());
        }
        self.properties.as_mut().unwrap().radius = Some(radius);
    }

    pub fn set_albedo(&mut self, albedo: f64) {
        if self.properties.is_none() {
            self.properties = Some(Properties::default());
        }
        self.properties.as_mut().unwrap().albedo = Some(albedo);
    }

    pub fn r(&self) -> f64 {
        self.position.norm()
    }

    pub fn hvec(&self) -> Vector3<f64> {
        self.position.cross(&self.velocity)
    }

    pub fn nvec(&self) -> Vector3<f64> {
        let hvec = self.hvec();
        // Vector3::new(0.0, 0.0, 1.0).cross(&self.hvec())
        Vector3::new(-hvec.y, hvec.x, 0.0)
    }

    pub fn h(&self) -> f64 {
        self.hvec().norm()
    }    

    pub fn evec(&self) -> Vector3<f64> {
        let hvec = self.hvec();
        self.velocity.cross(&hvec) / self.origin.mu() - self.position / self.r()
    }

    pub fn e(&self) -> f64 {
        self.evec().norm()
    }

    pub fn specific_energy(&self) -> f64 {
        self.v_squared() / 2.0 - self.origin.mu() / self.r()
    }

    pub fn a(&self) -> f64 {
        -self.origin.mu() / (2.0 * self.specific_energy())
    }

    pub fn q(&self) -> f64 {
        let h = self.h();
        let e = self.e();
        let mu = self.origin.mu();
        h.powi(2) / (mu * (1.0 + e))
    }

    pub fn p(&self) -> f64 {
        self.q() * (1.0 + self.e())
    }

    pub fn inc(&self) -> f64 {
        let hvec = self.hvec();
        (self.hvec().z / hvec.norm()).acos()
    }

    pub fn arg(&self) -> f64 {
        let orbit_type = OrbitType::from_eccentricity(self.e(), 1e-10).expect("Invalid eccentricity");
        if orbit_type == OrbitType::Circular {
            return 0.0;
        }
        let nvec = self.nvec();
        let evec = self.evec();
        let arg = (nvec.dot(&evec) / (nvec.norm() * evec.norm())).acos();
        if evec.z < 0.0 {
            2.0 * std::f64::consts::PI - arg
        } else {
            arg
        }
    }

    pub fn node(&self) -> f64 {
        let inc = self.inc();
        let tol = 1e-10;
        if inc < tol || inc > std::f64::consts::PI - tol {
            return 0.0;
        }
        let nvec = self.nvec();
        let n = nvec.norm();
        if nvec.y < 0.0 {
            2.0 * std::f64::consts::PI - (nvec.x / n).acos()
        } else {
            (nvec.x / n).acos()
        }
    }

    pub fn true_anomaly(&self) -> f64 {
        let orbit_type = OrbitType::from_eccentricity(self.e(), 1e-10).expect("Invalid eccentricity");
        let nvec = self.nvec();
        if orbit_type == OrbitType::Circular {
            return (nvec.dot(&self.position) / (nvec.norm() * self.r())).acos();
        }
        let nu = (self.evec().dot(&self.position) / (self.e() * self.r())).acos();
        if self.position.dot(&self.velocity) < 0.0 {
            2.0 * std::f64::consts::PI - nu
        } else {
            nu
        }
    }

    pub fn mean_anomaly(&self) -> f64 {
        let conic_anomaly = calc_conic_anomaly_from_true_anomaly(self.e(), self.true_anomaly()).expect("Invalid eccentricity");
        calc_mean_anomaly_from_conic_anomaly(self.e(), conic_anomaly).expect("Invalid eccentricity")
    }

    pub fn conic_anomaly(&self) -> f64 {
        calc_conic_anomaly_from_true_anomaly(self.e(), self.true_anomaly()).expect("Invalid eccentricity")
    }

    // calculate the osculating elements and return a KeplerOrbit object. This is more expensive than the other 
    // individual methods, but cheaper if you need multiple elements
    // pub fn calculate_orbit(&self) -> KeplerOrbit {
    //     OrbitType::from_eccentricity(e, 1e-10).expect("Invalid eccentricity");
    // }

    pub fn observe(&mut self, observer: &Observer) -> Result<Observation, Box<dyn std::error::Error>> {

        // self.change_reference_plane("J2000")?;

        // throw an error if the observer and self have different epochs
        if self.epoch.utc().jd() != observer.epoch().utc().jd() {
            
            return Err("Observer and SpaceRock have different epochs".into());
        }

        if self.reference_plane.as_str() != observer.reference_plane() {
            return Err("Observer and SpaceRock have different reference planes".into());
        }
        // Calculate the topocentric state, correct for light travel time
        let cr = correct_for_ltt(&self, observer);

        // Calaculate the ra, and dec
        let mut ra = cr.position.y.atan2(cr.position.x);
        if ra < 0.0 {
            ra += 2.0 * std::f64::consts::PI;
        }
        let dec = (cr.position.z / cr.position.norm()).asin();

        // Calculate the ra and dec rates
        let xi = cr.position.x.powi(2) + cr.position.y.powi(2);
        let ra_rate = - (cr.position.y * cr.velocity.x - cr.position.x * cr.velocity.y) / xi;
        let num = -cr.position.z * (cr.position.x * cr.velocity.x + cr.position.y * cr.velocity.y) + xi * cr.velocity.z;
        let denom = xi.sqrt() * cr.position.norm_squared();
        let dec_rate = num / denom;

        // calculate the topocentric range and range rate
        let rho = cr.position.norm();
        let rho_rate = cr.position.dot(&cr.velocity) / rho;


        // if self has properties, calculate the magnitude
        let mut mag = None;
        
        if let Some(properties) = &self.properties {

            if let Some(absolute_magnitude) = properties.absolute_magnitude {
                
                let gslope = properties.gslope.unwrap();

                let delta = cr.position.norm();
                let sun_dist = (cr.position + observer.position()).norm();
                let earth_dist = observer.position().norm();
                let q = (sun_dist.powi(2) + delta.powi(2) - earth_dist) / (2.0 * sun_dist * delta);
                // let mut beta = 0.0;
                // match q {
                //     q if q <= -1.0 => beta = std::f64::consts::PI,
                //     q if q >= 1.0 => beta = 0.0,
                //     _ => beta = q.acos(),
                // };
                let beta = match q {
                    q if q <= -1.0 => std::f64::consts::PI,
                    q if q >= 1.0 => 0.0,
                    _ => q.acos(),
                };

                let psi_1 = (-3.332 * ((beta / 2.0).tan()).powf(0.631)).exp();
                let psi_2 = (-1.862 * ((beta / 2.0).tan()).powf(1.218)).exp();
                mag = Some(absolute_magnitude + 5.0 * (sun_dist * delta).log10());
                if psi_1 == 0.0 && psi_2 == 0.0 {
                    mag = mag;
                } else {
                    let mm = mag.unwrap() - 2.5 * ((1.0 - gslope) * psi_1 + gslope * psi_2).log10();
                    mag = Some(mm);
                }
            }
        }

        let observation = Observation::from_complete(self.epoch.clone(), ra, dec, ra_rate, dec_rate, rho, rho_rate, mag, observer.clone());
        Ok(observation)
    }

}



    // pub fn from_state(name: &str, state: StateVector, epoch: Time, reference_plane: &ReferencePlane, origin: &Origin) -> Self {
    //     let position = state.position;
    //     let velocity = state.velocity;
    //     SpaceRock {
    //         name: name.to_string().into(),
    //         position: position,
    //         velocity: velocity,
    //         epoch: epoch,
    //         reference_plane: reference_plane.clone(),
    //         origin: origin.clone(),
    //     }
    // }

    // pub fn from_kepler(name: &str, orbit: KeplerOrbit, epoch: Time, reference_plane: &ReferencePlane, origin: &Origin) -> Self {
    //     let state = calc_xyz_from_kepM(orbit.a, orbit.e, orbit.inc, orbit.arg, orbit.node, orbit.M());
    //     SpaceRock {
    //         name: name.to_string().into(),
    //         position: state.position,
    //         velocity: state.velocity,
    //         epoch: epoch,
    //         reference_plane: reference_plane.clone(),
    //         origin: origin.clone(),
    //     }
    // }


    // pub fn random() -> Self {
    //     let mut rng = rand::thread_rng();
    //     let a = rng.gen_range(40.0..50.0);
    //     let e = rng.gen_range(0.0..0.3);
    //     let inc = rng.gen_range(0.0..std::f64::consts::PI/3.0);
    //     let arg = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
    //     let node = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
    //     let f = rng.gen_range(0.0..2.0 * std::f64::consts::PI);

    //     // uuid for name
    //     let name = format!("{}", uuid::Uuid::new_v4().simple());

    //     SpaceRock::from_kepler(&name, KeplerOrbit::new(a, e, inc, arg, node, f), Time::now(), &ReferencePlane::J2000, &Origin::SSB)
    // }

    // Methods

    // pub fn analytic_propagate(&mut self, epoch: &Time) {

    //     let timescale = &self.epoch.timescale;
    //     let mut epoch = epoch.clone();

    //     epoch.change_timescale(timescale.clone());
    //     let dt = epoch.epoch - self.epoch.epoch;

    //     // check that self.orbit is not None
    //     match &self.orbit {
    //         None => self.calculate_orbit(),
    //         _ => (),
    //     }

    //     if let Some(orbit) = &self.orbit {
    //         let dM = orbit.n() * dt;
    //         let M_new = orbit.M() + dM;
    //         let new_state = calc_xyz_from_kepM(orbit.a, orbit.e, orbit.inc, orbit.arg, orbit.node, M_new);
    //         self.position = Vector3::new(new_state.position[0], new_state.position[1], new_state.position[2]);
    //         self.velocity = Vector3::new(new_state.velocity[0], new_state.velocity[1], new_state.velocity[2]);
    //         self.epoch = epoch;
    //         self.calculate_orbit();
    //     }        
    // }

    // pub fn at(&mut self, epoch: &Time) {
    //     self.analytic_propagate(epoch)
    // }


    // pub fn calculate_orbit(&mut self) {
    //     self.orbit = Some(KeplerOrbit::from_xyz(StateVector {position: self.position, velocity: self.velocity}));
    // }


/// Display the SpaceRock object with each field on a new line
impl std::fmt::Display for SpaceRock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SpaceRock: {}\nEpoch: {:?}\nReference Plane: {}\nOrigin: {}\nPosition: {:?}\nVelocity: {:?}\nProperties: {:?}", 
        self.name, self.epoch, self.reference_plane, self.origin, self.position, self.velocity, self.properties)
    }
}





/// Generate a "name" made up of random syllables.
/// `min_syllables` - minimum number of syllables
/// `max_syllables` - maximum number of syllables
fn generate_name(min_syllables: usize, max_syllables: usize) -> String {
    // Choose how many syllables this name will have
    let syllables_count = rand::thread_rng().gen_range(min_syllables..=max_syllables);

    // Build the name
    let mut name = String::new();
    for _i in 0..syllables_count {
        let s = generate_syllable();
        name.push_str(&s);
    }

    name
}

/// Generate a single syllable in the form:
/// (optional consonant) + vowel + (optional consonant)
fn generate_syllable() -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    // You can include more consonants if you like.
    // Some letters (like 'q', 'x', 'z') might produce more unusual results.
    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm',
        'n', 'p', 'r', 's', 't', 'v', 'w', 'y', 'z', 'x', 'q',
    ];
    
    let mut rng = rand::thread_rng();
    
    let mut syllable = String::new();
    
    // 50% chance to start with a consonant
    if rng.gen_bool(0.5) {
        syllable.push(consonants[rng.gen_range(0..consonants.len())]);
    }
    
    // Always include a vowel
    syllable.push(vowels[rng.gen_range(0..vowels.len())]);
    
    // 40% chance to add a trailing consonant
    if rng.gen_bool(0.4) {
        syllable.push(consonants[rng.gen_range(0..consonants.len())]);
    }
    
    syllable
}