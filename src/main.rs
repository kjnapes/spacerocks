use spacerocks::{SpaceRock, Time};

fn main() {
    let rock = SpaceRock::from_horizons("Arrokoth", &Time::now(), "j2000", "ssb");
    println!("{:?}", rock);
}