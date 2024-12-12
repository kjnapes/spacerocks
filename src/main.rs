use spacerocks::{SpaceRock, Time, SpiceKernel, Simulation};
use plotters::prelude::*;
use nalgebra::Vector3;
// hash map
use std::collections::HashMap;

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
    sim.move_to_center_of_mass();

    // storage for positions
    // set up a hash map to store the positions of the rocks
    let mut positions: HashMap<String, Vec<Vector3<f64>>> = HashMap::new();


    let dt = 1000.0;
    let t_total = 365.25 * 100_000.0;
    let n_epochs = (t_total / dt) as usize;

    for i in 0..n_epochs {
        let t = epoch.clone() + (i as f64) * dt;
        sim.integrate(&t);
        for rock in &sim.particles {
            let pos = rock.position;
            let name = rock.name.clone();
            if positions.contains_key(&name) {
                positions.get_mut(&name).unwrap().push(pos);
            } else {
                positions.insert(name.to_string(), vec![pos]);
            }
        }
    }
    
    let root = BitMapBackend::new("plot.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Arrokoth", ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(-50.0..50.0, -50.0..50.0)?;

    chart.configure_mesh().draw()?;

    for (name, pos) in positions.iter() {
        let x: Vec<f64> = pos.iter().map(|v| v.x).collect();
        let y: Vec<f64> = pos.iter().map(|v| v.y).collect();

        // scatter plot
        chart.draw_series(
            x.iter().zip(y.iter()).map(|(x, y)| {
                Circle::new((*x, *y), 2, &BLACK)
            })
        )?;
    }
   




    Ok(())
}