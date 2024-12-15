
use crate::SpaceRock;
use crate::time::Time;
use crate::Simulation;
use crate::nbody::integrators::Integrator;
use crate::nbody::forces::Force;

use std::sync::Arc;
use rayon::prelude::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct MVS {
    pub timestep: f64,
}

impl MVS {
    pub fn new(timestep: f64) -> MVS {
        MVS { timestep }
    }
}

impl Integrator for MVS {

    fn step(&mut self, particles: &mut Vec<SpaceRock>, epoch: &mut Time, forces: &Vec<Box<dyn Force + Send + Sync>>) {

        // drift
        for particle in &mut *particles {
            if particle.name == Arc::new("Sun".to_string()) {
                continue;
            }
            if particle.name == Arc::new("sun".to_string()) {
                continue;
            }
            particle.analytic_propagate(&(particle.epoch.clone() + self.timestep / 2.0));
        }
      
        for force in forces {
            force.apply(&mut particles[1..].to_vec());
        }

        for particle in &mut *particles {
            if particle.name == Arc::new("Sun".to_string()) {
                continue;
            }
            if particle.name == Arc::new("sun".to_string()) {
                continue;
            }
            particle.velocity += particle.acceleration * self.timestep / 2.0;
            particle.analytic_propagate(&(particle.epoch.clone() + self.timestep / 2.0));
        }

        *epoch += self.timestep;

    }

    fn timestep(&self) -> f64 {
        self.timestep
    }

    fn set_timestep(&mut self, timestep: f64) {
        self.timestep = timestep;
    }
}
