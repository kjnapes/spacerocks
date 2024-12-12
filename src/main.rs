use spacerocks::{SpaceRock, Time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rock = SpaceRock::from_horizons("Arrokoth", &Time::now(), "j2000", "ssb")?;
    println!("{:?}", rock);

    rock.set_mass(1.0);
    println!("{:?}", rock);

    Ok(())
}