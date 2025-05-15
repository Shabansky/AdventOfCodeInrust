use std::default::Default;
use std::fmt::Display;
use std::ops::Range;

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

struct SquareMap {
    fields: Vec<Vec<Light>>,
}

impl SquareMap {
    fn new(side: usize) -> Self {
        Self {
            fields: vec![vec![Light::default(); side]; side],
        }
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

enum Actions {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
struct ActionRectangleSelection {
    origin: Coordinate,
    destination: Coordinate,
    action: Actions,
}

impl ActionRectangleSelection {
    fn new(origin: Coordinate, destination: Coordinate, action: Actions) -> Self {
        Self {
            origin,
            destination,
            action,
        }
    }

    fn get_width(&self) -> Range<usize> {
        self.origin.x..self.destination.x
    }

    fn get_height(&self) -> Range<usize> {
        self.origin.y..self.destination.y
    }
}

fn main() {
    //Map allocated on heap via vec as a 1000x1000 on an array will exceed the stack memory limit.
    let mut map = SquareMap::new(SQUARE_SIDE);
    let action = ActionRectangleSelection::new(
        Coordinate::new(2, 3),
        Coordinate::new(5, 5),
        Actions::TurnOn,
    );

    for row in &map.fields {
        for light in row {
            print!("{light}");
        }
        println!();
    }
    for row in &mut map.fields[action.get_height()] {
        for light in &mut row[action.get_width()] {
            light.state = LightState::Lit;
        }
    }

    println!();

    for row in &map.fields {
        for light in row {
            print!("{light}");
        }
        println!(" ");
    }
}
