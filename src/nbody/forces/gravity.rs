use crate::nbody::forces::Force;
use crate::spacerock::SpaceRock;
use crate::constants::GRAVITATIONAL_CONSTANT;

use nalgebra::Vector3;


#[derive(Debug, Clone, Copy)]
pub struct NewtonianGravity;

impl Force for NewtonianGravity {

    fn calculate_acceleration(&self, entities: &mut Vec<SpaceRock>) -> Vec<Vector3<f64>> {
        // Naive implementation of Newtonian gravity. O(0.5 * n^2) complexity.
        // Speed it up if you want!

        let mut acceleration = vec![Vector3::zeros(); entities.len()];

        let n_entities = entities.len();
        for idx in 0..n_entities {

            // let idx_massless = entities[idx].mass() == 0.0;

            for jdx in (idx + 1)..n_entities {

                if (entities[idx].mass() == 0.0) & (entities[jdx].mass() == 0.0) {
                    continue;
                }

                // if idx_massless & (entities[jdx].mass() == 0.0) {
                //     continue;
                // }

                let r_vec = entities[idx].position - entities[jdx].position;
                let r = r_vec.norm();

                let xi = -GRAVITATIONAL_CONSTANT * r_vec / (r * r * r);
                let idx_acceleration = xi * entities[jdx].mass();
                let jdx_acceleration = -xi * entities[idx].mass();
                acceleration[idx] += idx_acceleration;
                acceleration[jdx] += jdx_acceleration;
            }
        }
        acceleration
    }

 
}