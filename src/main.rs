use spacerocks::{SpaceRock, Time, SpiceKernel, Simulation};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let spice_root = "/Users/kjnapier/data/spice";

    // define the epoch
    let epoch = Time::now();

    // load spice kernels
    let mut kernel = SpiceKernel::new();
    kernel.load(format!("{}/de440s.bsp", spice_root).as_str())?;
    kernel.load(format!("{}/latest_leapseconds.tls", spice_root).as_str())?;

    // load earth from spice
    // let earth = SpaceRock::from_spice("EarTh", &epoch, "EcLiPj2000", "sSb")?;
    // println!("{}", earth);

    // load arrokoth from spice
    let rock = SpaceRock::from_horizons("Arrokoth", &epoch, "j2000", "ssb")?;
    // println!("{}", rock);

    // create a simulation and add arrokoth
    let mut sim = Simulation::giants(&epoch, "ECLIPJ2000", "SSB")?;
    sim.add(rock)?; 

    sim.integrate(&(epoch + 1.0));
    for rock in sim.particles {
        println!("{}", rock);
    }


    Ok(())
}