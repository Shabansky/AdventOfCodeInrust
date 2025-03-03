use std::fs;
fn main() {
    asserts();

    let file_path = "floors_input.txt";

    match fs::read_to_string(file_path) {
        Ok(floors_input) => {
            println!("The floor is {}", get_floor(&floors_input));
        }
        Err(e) => {
            println!("Error reading file at {file_path}. Error {e}");
        }
    }
}

fn asserts() {
    assert_eq!(0, get_floor(&String::from("(())")));
    assert_eq!(0, get_floor(&String::from("()()")));
    assert_eq!(3, get_floor(&String::from("(((")));
    assert_eq!(3, get_floor(&String::from("(()(()(")));
    assert_eq!(3, get_floor(&String::from("))(((((")));
    assert_eq!(-1, get_floor(&String::from("())")));
    assert_eq!(-1, get_floor(&String::from("))(")));
    assert_eq!(-3, get_floor(&String::from(")))")));
    assert_eq!(-3, get_floor(&String::from(")())())")));
}

fn get_floor(floor_instruction: &String) -> i32 {
    let mut floor = 0;

    for symbol in floor_instruction.chars() {
        if symbol == '(' {
            floor += 1;
        } else if symbol == ')' {
            floor -= 1;
        }
    }

    floor
}
