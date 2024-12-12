use crate::SpaceRock;
use crate::time::Time;
use crate::nbody::forces::Force;


pub trait Integrator: Send + Sync + IntegratorClone {
    fn step(&mut self, particles: &mut Vec<SpaceRock>, epoch: &mut Time, forces: &Vec<Box<dyn Force + Send + Sync>>);
    fn timestep(&self) -> f64;
    fn set_timestep(&mut self, timestep: f64);
}


pub trait IntegratorClone {
    fn clone_box(&self) -> Box<dyn Integrator + Send + Sync>;
}

impl<T> IntegratorClone for T
where
    T: 'static + Integrator + Clone,
{
    fn clone_box(&self) -> Box<dyn Integrator + Send + Sync> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Integrator + Send + Sync>{
    fn clone(&self) -> Box<dyn Integrator + Send + Sync> {
        self.clone_box()
    }
}
