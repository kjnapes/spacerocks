
use crate::SpaceRock;
use crate::time::Time;
use crate::nbody::integrators::Integrator;
use crate::nbody::forces::Force;

use nalgebra::Vector3;

// use rayon::prelude::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Leapfrog {
    pub timestep: f64,
}

impl Leapfrog {
    pub fn new(timestep: f64) -> Leapfrog {
        Leapfrog { timestep }
    }
}

impl Integrator for Leapfrog {

    fn step(&mut self, particles: &mut Vec<SpaceRock>, epoch: &mut Time, forces: &Vec<Box<dyn Force + Send + Sync>>) {
        // drift
        for particle in &mut *particles {
            particle.position += particle.velocity * 0.5 * self.timestep;
            particle.epoch += 0.5 * self.timestep;
        }
      
        let mut accelerations = vec![Vector3::new(0.0, 0.0, 0.0); particles.len()];
        for force in forces {
            let acc = force.calculate_acceleration(particles);
            for (idx, a) in acc.iter().enumerate() {
                accelerations[idx] += a;
            }
        }

        // for particle in &mut *particles {
        //     particle.velocity += self.timestep * particle.acceleration;
        //     particle.position += particle.velocity * 0.5 * self.timestep;
        //     particle.epoch += 0.5 * self.timestep;
        // }

        *epoch += self.timestep;

        for (particle, acceleration) in particles.iter_mut().zip(accelerations.iter()) {
            particle.velocity += self.timestep * acceleration;
            particle.position += particle.velocity * 0.5 * self.timestep;
            particle.epoch = epoch.clone();
        }

        

    }

    fn timestep(&self) -> f64 {
        self.timestep
    }

    fn set_timestep(&mut self, timestep: f64) {
        self.timestep = timestep;
    }
}
