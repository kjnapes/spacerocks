use std::collections::HashMap;

use crate::SpaceRock;
use crate::constants::GRAVITATIONAL_CONSTANT;
use crate::time::Time;
use crate::{ReferencePlane, Origin};
use crate::errors::SimulationError;


use crate::nbody::forces::{Force, NewtonianGravity};
use crate::nbody::integrators::{Integrator, Leapfrog};


use nalgebra::Vector3;


pub struct Simulation {
    pub particles: Vec<SpaceRock>,
    pub epoch: Time,
    pub particle_index_map: HashMap<String, usize>,

    pub reference_plane: ReferencePlane,
    pub origin: Origin,

    pub integrator: Box<dyn Integrator + Send + Sync>,
    pub forces: Vec<Box<dyn Force + Send + Sync>>,
}

impl Simulation {

    pub fn new() -> Simulation {
        Simulation {
            particles: Vec::new(), 
            epoch: Time::now(), 
            forces: vec![Box::new(NewtonianGravity)],
            reference_plane: ReferencePlane::ECLIPJ2000,
            origin: Origin::SSB,
            integrator: Box::new(Leapfrog::new(1.0)),
            particle_index_map: HashMap::new()
        }
    }

    pub fn giants(epoch: &Time, reference_plane: &str, origin: &str) -> Result<Simulation, Box<dyn std::error::Error>> {

        let mut sim = Simulation::new();
        sim.epoch = epoch.clone();
        sim.integrator = Box::new(Leapfrog::new(1.0));

        // add sun, jupiter barycenter, saturn barycenter, uranus barycenter, neptune barycenter.
        for name in ["sun", "jupiter barycenter", "saturn barycenter", "uranus barycenter", "neptune barycenter"].iter() {
            let particle = SpaceRock::from_spice(name, epoch, reference_plane, origin)?;
            sim.add(particle)?;
        }
        Ok(sim)
    }

    pub fn add(&mut self, mut particle: SpaceRock) -> Result<(), Box<dyn std::error::Error>> {

        if self.epoch != particle.epoch {
            // return Err(SimulationError::EpochMismatch(particle.epoch.clone(), self.epoch.clone(), particle.name.clone()));
            let err = SimulationError::EpochMismatch(particle.epoch.clone(), self.epoch.clone(), particle.name.clone());
            return Err(err.into());
        }

        if particle.origin.clone() != self.origin {
            if !self.particle_index_map.contains_key(&particle.origin.to_string()) {
                // return Err(SimulationError::OriginMismatch(particle.origin, self.origin.clone()));
                let err = SimulationError::OriginMismatch(particle.origin.clone(), self.origin.clone());
                return Err(err.into());
            }
            let origin = &self.particles[self.particle_index_map[&particle.origin.to_string()]];
            particle.change_origin(&origin);
            println!("Changing origin of {} from {} to {}", particle.name, particle.origin, origin.name);
        }

        particle.change_reference_plane(self.reference_plane.as_str())?;
        self.particle_index_map.insert((*particle.name).to_string(), self.particles.len());
        self.particles.push(particle);

        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        if self.particle_index_map.contains_key(name) {
            let idx = self.particle_index_map[name];
            self.particles.remove(idx);
            self.particle_index_map.remove(name);
            // for (key, value) in self.particle_index_map.iter_mut() {
            //     if *value > idx {
            //         *value -= 1;
            //     }
            // }
            for value in self.particle_index_map.values_mut() {
                if *value > idx {
                    *value -= 1;
                }
            }
        } else {
            return Err(format!("No particle found with name {}", name));
        }
        Ok(())
    }

    pub fn step(&mut self) {
        self.integrator.step(&mut self.particles, &mut self.epoch, &self.forces);
    }

    pub fn move_to_center_of_mass(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut total_mass = 0.0;
        let mut center_of_mass = Vector3::new(0.0, 0.0, 0.0);
        let mut center_of_mass_velocity = Vector3::new(0.0, 0.0, 0.0);

        for particle in &self.particles {
            if particle.mass() == 0.0 {
                continue;
            }
            center_of_mass += particle.mass() * particle.position;
            center_of_mass_velocity += particle.mass() * particle.velocity;
            total_mass += particle.mass();
        }

        center_of_mass /= total_mass;
        center_of_mass_velocity /= total_mass;

        let x = center_of_mass.x;
        let y = center_of_mass.y;
        let z = center_of_mass.z;
        let vx = center_of_mass_velocity.x;
        let vy = center_of_mass_velocity.y;
        let vz = center_of_mass_velocity.z;
       
        let mut origin_rock = SpaceRock::from_xyz("simulation_barycenter", 
                                                  x, y, z, vx, vy, vz, 
                                                  self.epoch.clone(), 
                                                  self.reference_plane.as_str(),
                                                  self.origin.as_str())?;
        origin_rock.set_mass(total_mass);

        for particle in &mut self.particles {
            particle.change_origin(&origin_rock);
        }

        let origin = Origin::new_custom(total_mass * GRAVITATIONAL_CONSTANT, "simulation_barycenter");
        self.origin = origin;
        Ok(())
    }

    pub fn change_origin(&mut self, origin: &str) -> Result<(), String> {

        if !self.particle_index_map.contains_key(origin) {
           return Err(format!("Origin {} not found in perturbers", origin));
        }

        let new_origin = Origin::new_custom(self.particles[self.particle_index_map[origin]].mass() * GRAVITATIONAL_CONSTANT, origin);

        self.origin = new_origin;

        let origin_position = self.particles[self.particle_index_map[origin]].position;
        let origin_velocity = self.particles[self.particle_index_map[origin]].velocity;

        for particle in &mut self.particles {
            particle.position -= origin_position;
            particle.velocity -= origin_velocity;
        }

        Ok(())
    }

    pub fn integrate(&mut self, epoch: &Time) {

        let mut epoch = epoch.clone();
        epoch.change_timescale(self.epoch.timescale.clone());

        let dt = &epoch - &self.epoch;
        if dt.abs() < 1e-16 {
            return;
        }

        if dt < 0.0 {
            if self.integrator.timestep() > 0.0 {
                self.integrator.set_timestep(-self.integrator.timestep());
            }
        }

        loop {
            let dt = &epoch - &self.epoch;
            if dt.abs() < self.integrator.timestep().abs() {
                break;
            }
            if dt < 0.0 {
                if self.integrator.timestep() > 0.0 {
                    self.integrator.set_timestep(-self.integrator.timestep());
                }
            } else {
                if self.integrator.timestep() < 0.0 {
                    self.integrator.set_timestep(-self.integrator.timestep());
                }
            }
            self.step();
        }
        
        let dt = &epoch - &self.epoch;
        if dt.abs() < 1e-16 {
            return;
        }
        // create an exact match for the epoch
        let old_timestep = self.integrator.timestep();
        self.integrator.set_timestep(&epoch - &self.epoch);
        self.step();
        // reset the timestep
        self.integrator.set_timestep(old_timestep);
    }

    pub fn get_particle(&self, name: &str) -> Result<&SpaceRock, Box<dyn std::error::Error>> {
        if self.particle_index_map.contains_key(name) { 
            let idx = self.particle_index_map[name];
            let p = &self.particles[idx];
            return Ok(p);
        }
        return Err(format!("{} not found in simulation", name).into());
    }

    pub fn energy(&self) -> f64 {
        let mut kinetic_energy = 0.0;
        let mut potential_energy = 0.0;

        for idx in 0..self.particles.len() {
            kinetic_energy += 0.5 * self.particles[idx].mass() * self.particles[idx].velocity.norm_squared();
            for jdx in (idx + 1)..self.particles.len() {
                let r = (self.particles[idx].position - self.particles[jdx].position).norm();
                potential_energy -= GRAVITATIONAL_CONSTANT * self.particles[idx].mass() * self.particles[jdx].mass() / r;
            }
        }
        return kinetic_energy + potential_energy;
    }

    pub fn add_force(&mut self, force: Box<dyn Force + Send + Sync>) {
        self.forces.push(force);
    }

}



// pub fn planets(epoch: &Time, frame: &CoordinateFrame, origin: &Origin) -> Result<Simulation, String> {
    //     let mut sim = Simulation::new();
    //     sim.epoch = epoch.clone();
    //     sim.frame = Some(frame.clone());
    //     sim.origin = Some(origin.clone());
    //     sim.integrator = Box::new(IAS15::new(1.0));
    //     sim.add_force(Box::new(NewtonianGravity));

    //     let names = ["sun", "mercury barycenter", "venus barycenter", "earth barycenter", "mars barycenter", "jupiter barycenter", 
    //                  "saturn barycenter", "uranus barycenter", "neptune barycenter"];
    //     for name in names.iter() {
    //         let mut particle = SpaceRock::from_spice(name, epoch, frame, origin);
    //         sim.add(particle)?;
    //     }
    //     Ok(sim)
    // }

    // pub fn horizons(epoch: &Time, frame: &CoordinateFrame, origin: &Origin) -> Result<Simulation, String> {
    //     let mut sim = Simulation::new();
    //     sim.epoch = epoch.clone();
    //     sim.frame = Some(frame.clone());
    //     sim.origin = Some(origin.clone());
    //     sim.integrator = Box::new(IAS15::new(1.0));
    //     sim.add_force(Box::new(NewtonianGravity));

    //     let names = ["sun", "mercury barycenter", "venus barycenter", "earth", "moon", "mars barycenter", "jupiter barycenter", 
    //                  "saturn barycenter", "uranus barycenter", "neptune barycenter", "pluto barycenter", "2000001", "2000002", 
    //                     "2000003", "2000004", "2000007", "2000010", "2000015", "2000016", "2000031", "2000052", "2000065", "2000087",
    //                     "2000088", "2000107", "2000511", "2000704"];
    //     for name in names.iter() {
    //         let mut particle = SpaceRock::from_spice(name, epoch, frame, origin);
    //         sim.add(particle)?;
    //     }
    //     Ok(sim)
    // }