use std::default::Default;
use std::fmt::Display;

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

impl Display for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            LightState::Lit => write!(f, "1"),
            LightState::Unlit => write!(f, "0"),
        }
    }
}

fn main() {
    //Map allocated on heap via vec as a 1000x1000 on an array will exceed the stack memory limit.
    let mut map = build_map();
    let coords = ((2, 3), (5, 5));
    let y_low = coords.0 .1;
    let y_diff = coords.1 .1 - coords.0 .1;
    let x_low = coords.0 .0;
    let x_diff = coords.1 .0 - coords.0 .0;

    for row in &map {
        for light in row {
            print!("{light}");
        }
        println!();
    }

    for row in &mut map[y_low..(y_low + y_diff)] {
        for light in &mut row[x_low..(x_low + x_diff)] {
            light.state = LightState::Lit;
        }
    }

    println!();

    for row in &map {
        for light in row {
            print!("{light}");
        }
        println!(" ");
    }
}

fn build_map() -> Vec<Vec<Light>> {
    vec![vec![Light::default(); SQUARE_SIDE]; SQUARE_SIDE]
}
