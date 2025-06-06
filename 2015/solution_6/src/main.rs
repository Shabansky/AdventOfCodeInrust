use regex::Regex;
use std::default::Default;
use std::fmt::Display;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

//Map allocated on heap via vec as a 1000x1000 on an array will exceed the stack memory limit.
const SQUARE_SIDE: usize = 1000;

enum Errors {
    OriginLargetDestination,
}

#[derive(Clone, Debug, PartialEq)]
enum LightState {
    Lit,
    Unlit,
}

trait Light {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn toggle(&mut self);
    fn get_value(&self) -> usize;
}

#[derive(Clone, Debug)]
struct LightBinary {
    state: LightState,
}

impl LightBinary {
    fn new() -> Self {
        Self {
            state: LightState::Unlit,
        }
    }

    fn is_on(&self) -> bool {
        self.state == LightState::Lit
    }
}

impl Light for LightBinary {
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

    fn get_value(&self) -> usize {
        if self.state == LightState::Lit {
            1
        } else {
            0
        }
    }
}

impl Default for LightBinary {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, Default)]
struct LightAmplifying {
    state: usize,
}

impl LightAmplifying {
    fn new() -> Self {
        Self { state: 0 }
    }
}

impl Light for LightAmplifying {
    fn turn_on(&mut self) {
        self.state += 1;
    }

    fn turn_off(&mut self) {
        if self.state > 0 {
            self.state -= 1;
        }
    }

    fn toggle(&mut self) {
        self.state += 2;
    }

    fn get_value(&self) -> usize {
        self.state
    }
}

struct SquareMap<T> {
    fields: Vec<Vec<T>>,
}

impl<T: Light + Clone + Default> SquareMap<T> {
    fn new(side: usize) -> Self {
        Self {
            fields: vec![vec![T::default(); side]; side],
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

    fn num_lights(&self) -> usize {
        self.fields
            .iter()
            .flatten()
            .map(|light| light.get_value())
            .sum()
    }
}

impl<T: Display> Display for SquareMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.fields {
            for light in row {
                write!(f, "{light} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for LightBinary {
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

impl TryFrom<&str> for Coordinate {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut coords = s.split(',');
        let x = coords.next().unwrap().parse::<usize>().unwrap();
        let y = coords.next().unwrap().parse::<usize>().unwrap();
        Ok(Coordinate::new(x, y))
    }
}

#[derive(Debug)]
struct SequentialCoordinates {
    origin: Coordinate,
    destination: Coordinate,
}

impl TryFrom<(Coordinate, Coordinate)> for SequentialCoordinates {
    type Error = Errors;

    fn try_from(coordinates: (Coordinate, Coordinate)) -> Result<Self, Self::Error> {
        let (origin, destination) = coordinates;
        if origin.x > destination.x || origin.y > destination.y {
            Err(Errors::OriginLargetDestination)
        } else {
            Ok(SequentialCoordinates {
                origin,
                destination,
            })
        }
    }
}

#[derive(Debug)]
struct ActionRectangleSelection {
    coordinates: SequentialCoordinates,
    action: Action,
}

impl ActionRectangleSelection {
    fn new(coordinates: SequentialCoordinates, action: Action) -> Self {
        Self {
            coordinates,
            action,
        }
    }

    fn get_width(&self) -> RangeInclusive<usize> {
        self.coordinates.origin.x..=self.coordinates.destination.x
    }

    fn get_height(&self) -> RangeInclusive<usize> {
        self.coordinates.origin.y..=self.coordinates.destination.y
    }
}

impl FromStr for ActionRectangleSelection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //turn on 0,0 through 999,999
        let regex = Regex::new(r"(.+)\s(\d+,\d+) through (\d+,\d+)").unwrap();

        let Some(captures) = regex.captures(s) else {
            return Err("No matches in input string");
        };

        if captures.len() != 4 {
            return Err("Input does not have all the needed information");
        }
        //Check for action
        let action = match captures[1].as_ref() {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => return Err("Invalid action specified"),
        };

        let origin = Coordinate::try_from(&captures[2])?;

        let dest = Coordinate::try_from(&captures[3])?;

        //        let dest = Coordinate::new(9, 9);
        match SequentialCoordinates::try_from((origin, dest)) {
            Err(_) => Err("Origin coordinate cannot be larger than destination coordinate"),
            Ok(coordinates) => Ok(Self::new(coordinates, action)),
        }
    }
}

fn main() {
    let mut map: SquareMap<LightAmplifying> = SquareMap::new(SQUARE_SIDE);

    let file_path = "input.txt";
    let text = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            panic!("Error reading file at {file_path}. Error {e}");
        }
    };

    //Read input char by char
    for line in text.lines() {
        let action: ActionRectangleSelection = match ActionRectangleSelection::from_str(line) {
            Ok(action) => action,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        };
        map.apply(action);
    }

    // println!("{action:?}");

    println!("COUNT: {}", map.num_lights());
    // println!("{map}");
}
