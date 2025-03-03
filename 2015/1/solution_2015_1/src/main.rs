use std::fs;

struct SantaFloorTracer {
    floor: i32,
    floor_at_first_basement: i32,
}

impl SantaFloorTracer {
    fn get_floor(&mut self, floor_instruction: &String) -> i32 {
        let mut floor_count = 0;
        let mut first_basement_reached = false;

        self.floor = 0;

        for symbol in floor_instruction.chars() {
            match symbol {
                '(' => self.floor += 1,
                ')' => self.floor -= 1,
                _ => continue,
            }

            floor_count += 1;

            if self.floor == -1 && first_basement_reached == false {
                first_basement_reached = true;
                self.floor_at_first_basement = floor_count;
            }
        }

        self.floor
    }
}

fn main() {
    let mut santa_tracer = SantaFloorTracer {
        floor: 0,
        floor_at_first_basement: 0,
    };

    asserts(&mut santa_tracer);

    let file_path = "floors_input.txt";

    match fs::read_to_string(file_path) {
        Ok(floors_input) => {
            println!("The floor is {}", santa_tracer.get_floor(&floors_input));
            println!(
                "Position of first basement char is {}",
                santa_tracer.floor_at_first_basement
            );
        }
        Err(e) => {
            println!("Error reading file at {file_path}. Error {e}");
        }
    }
}

fn asserts(santa_tracer: &mut SantaFloorTracer) {
    assert_eq!(0, santa_tracer.get_floor(&String::from("(())")));
    assert_eq!(0, santa_tracer.get_floor(&String::from("()()")));
    assert_eq!(3, santa_tracer.get_floor(&String::from("(((")));
    assert_eq!(3, santa_tracer.get_floor(&String::from("(()(()(")));
    assert_eq!(3, santa_tracer.get_floor(&String::from("))(((((")));
    assert_eq!(-1, santa_tracer.get_floor(&String::from("())")));
    assert_eq!(-1, santa_tracer.get_floor(&String::from("))(")));
    assert_eq!(-3, santa_tracer.get_floor(&String::from(")))")));
    assert_eq!(-3, santa_tracer.get_floor(&String::from(")())())")));

    santa_tracer.get_floor(&String::from(")"));
    assert_eq!(1, santa_tracer.floor_at_first_basement);

    santa_tracer.get_floor(&String::from("()())"));
    assert_eq!(5, santa_tracer.floor_at_first_basement);
}
