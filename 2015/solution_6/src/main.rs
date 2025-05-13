use std::default::Default;

const SQUARE_SIDE: usize = 10;

#[derive(Clone, Debug)]
enum LightState {
    Lit,
    Unlit,
}
#[derive(Clone, Debug)]
struct Light {
    state: LightState,
}

impl Light {
    fn new() -> Self {
        Light {
            state: LightState::Unlit,
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    //Map allocated on heap via vec as a 1000x1000 on an array will exceed the stack memory limit.
    let map = build_map();
}

fn build_map() -> Vec<Vec<Light>> {
    let map = vec![vec![Light::default(); SQUARE_SIDE]; SQUARE_SIDE];

    for row in &map {
        for col in row {
            print!("{col:?} ");
        }
    }

    map
}
