use regex::Regex;
use std::default::Default;
use std::fmt::Display;
use std::ops::Range;
use std::str::FromStr;

const SQUARE_SIDE: usize = 10;

#[derive(Clone, Debug, PartialEq)]
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

    fn turn_on(&mut self) {
        self.state = LightState::Lit;
    }

    fn turn_off(&mut self) {
        self.state = LightState::Unlit;
    }

    fn toggle(&mut self) {
        if self.state == LightState::Unlit {
            self.state = LightState::Lit;
        } else {
            self.state = LightState::Unlit;
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

    fn apply(&mut self, action: ActionRectangleSelection) {
        for row in &mut self.fields[action.get_height()] {
            for light in &mut row[action.get_width()] {
                match action.action {
                    Action::TurnOn => light.turn_on(),
                    Action::TurnOff => light.turn_off(),
                    Action::Toggle => light.toggle(),
                }
            }
        }
    }
}

impl Display for SquareMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for row in &self.fields {
            for light in row {
                write!(f, "{light}");
            }
            println!();
        })
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

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct ActionRectangleSelection {
    origin: Coordinate,
    destination: Coordinate,
    action: Action,
}

impl ActionRectangleSelection {
    fn new(origin: Coordinate, destination: Coordinate, action: Action) -> Self {
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

impl FromStr for ActionRectangleSelection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //turn on 0,0 through 999,999
        let regex = Regex::new(r"(.+)\s(\d+,\d+) through (\d+,\d+)").unwrap();

        let Some(captures) = regex.captures(s) else {
            return Err("No matches in input string".to_string());
        };

        if captures.len() != 4 {
            return Err("Input does not have all the needed information".to_string());
        }

        println!("{captures:#?}");

        //Check for action
        let action = match captures[1].as_ref() {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => return Err("Invalid action specified".to_string()),
        };
        let origin = Coordinate::new(0, 0);
        let dest = Coordinate::new(999, 999);
        Ok(Self::new(origin, dest, action))
    }
}

fn main() {
    //Map allocated on heap via vec as a 1000x1000 on an array will exceed the stack memory limit.
    let mut map = SquareMap::new(SQUARE_SIDE);
    let action =
        ActionRectangleSelection::new(Coordinate::new(2, 3), Coordinate::new(5, 5), Action::TurnOn);

    println!("{map}");

    map.apply(action);
    println!("{map}");

    let action = ActionRectangleSelection::from_str("turn on 0,0 through 999,999");
    println!("{action:?}");
}
