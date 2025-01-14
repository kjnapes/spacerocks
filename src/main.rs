use spacerocks::{SpaceRock, Time, SpiceKernel, Observatory, Simulation};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let spice_root = "/Users/kjnapier/data/spice";

    // define the epoch
    let epoch = Time::now();

    println!("{}", epoch);

    // // // load spice kernels
    let mut kernel = SpiceKernel::new();
    kernel.load(format!("{}/sb441-n373s.bsp", spice_root).as_str())?;
    kernel.load(format!("{}/de440s.bsp", spice_root).as_str())?;
    kernel.load(format!("{}/latest_leapseconds.tls", spice_root).as_str())?;
    kernel.load(format!("{}/earth_latest_high_prec.bpc", spice_root).as_str())?;


    // observer not working rn, so commenting out for complilation
    let f51 = Observatory::from_obscode("F51")?;

    let observer = f51.at(&epoch, "J2000", "SSB")?;
    println!("{:?}", observer);

    let mut arrokoth = SpaceRock::from_horizons("Arrokoth", &epoch, "j2000", "ssb")?;

    let mut sim = Simulation::horizons(&epoch, "J2000", "SSB")?;
    sim.add(arrokoth)?;

    println!("{}", sim.epoch);

    // let observer = f51.at(&epoch)?;

    // let mut arrokoth = SpaceRock::from_horizons("Arrokoth", &epoch, "j2000", "ssb")?;
    // let observation = arrokoth.observe(&observer)?;
    // let mut arrokoth = SpaceRock::from_horizons("Arrokoth", &epoch, "j2000", "ssb")?;
    // let observation = arrokoth.observe(&observer)?;

    // println!("{}", observation);
    // println!("{}", observation);
    


    // // load arrokoth from spice
    // let rock = SpaceRock::from_horizons("Arrokoth", &epoch, "j2000", "ssb")?;
    // // println!("{}", rock);

    // create a simulation and add arrokoth
    // let mut sim = Simulation::giants(&epoch, "ECLIPJ2000", "SSB")?;
    // sim.add(rock)?; 
    // sim.move_to_center_of_mass()?;

    // let dt = 10.0;
    // let t_total = 365.25 * 10_000.0;
    // let n_epochs = (t_total / dt) as usize;

    // let mut positions: HashMap<String, Vec<Vector3<f64>>> = HashMap::new();
    // for i in 0..n_epochs {
    //     let t = epoch.clone() + (i as f64) * dt;
    //     sim.integrate(&t);
    //     for rock in &sim.particles {
    //         let pos = rock.position;
    //         let name = rock.name.clone();
    //         if positions.contains_key(&name) {
    //             positions.get_mut(&name).unwrap().push(pos);
    //         } else {
    //             positions.insert(name.to_string(), vec![pos]);
    //         }
    //     }
    // }
    
    // let caption = format!("{} year simulation", (t_total / 365.25) as usize);
    // let root = BitMapBackend::new("plot.png", (800, 800)).into_drawing_area();
    // root.fill(&WHITE)?;
    // let mut chart = ChartBuilder::on(&root)
    //     .caption(caption, ("sans-serif", 30).into_font())
    //     .margin(5)
    //     .x_label_area_size(40)
    //     .y_label_area_size(40)
    //     .build_ranged(-50.0..50.0, -50.0..50.0)?;
    // chart.configure_mesh().draw()?;

    // for (name, pos) in positions.iter() {
    //     let x: Vec<f64> = pos.iter().map(|v| v.x).collect();
    //     let y: Vec<f64> = pos.iter().map(|v| v.y).collect();
    //     chart.draw_series(
    //         x.iter().zip(y.iter()).map(|(x, y)| {
    //             Circle::new((*x, *y), 1, &BLACK)
    //         })
    //     )?;
    // }
   
    Ok(())
}