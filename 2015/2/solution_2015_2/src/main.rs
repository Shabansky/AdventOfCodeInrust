use std::fs;

type Feet = u32;

struct Box {
    l: Feet,
    w: Feet,
    h: Feet,
}

impl Box {
    fn from_string(string: &str) -> Result<Self, String> {
        let vec_slice = split_string_by_delim(string, 'x');

        let dimensions_vec = vec_string_to_u32(vec_slice);

        match dimensions_vec {
            Ok(vec) => {
                if let [l, w, h] = vec[..] {
                    return Ok(Box { l, w, h });
                }
            }
            Err(err) => {
                return Err(err);
            }
        }

        Err(String::from("Malformed input"))
    }

    fn get_wrapping_paper_needed(&self) -> Feet {
        let mut wrapping_paper = 0;

        wrapping_paper += self.get_box_area();
        wrapping_paper += self.get_smallest_side_area();

        wrapping_paper
    }

    fn get_smallest_side_area(&self) -> Feet {
        let side_lw = BoxSide(self.l, self.w).area();
        let side_wh = BoxSide(self.w, self.h).area();
        let side_lh = BoxSide(self.l, self.h).area();

        let sides = [side_lw, side_wh, side_lh];
        sides.iter().min().unwrap().clone()
    }

    fn get_box_area(&self) -> Feet {
        let side_lw = BoxSide(self.l, self.w).area();
        let side_wh = BoxSide(self.w, self.h).area();
        let side_lh = BoxSide(self.l, self.h).area();

        side_lw * 2 + side_wh * 2 + side_lh * 2
    }

    fn get_box_volume(&self) -> Feet {
        self.l * self.w * self.h
    }

    fn get_ribbon_needed(&self) -> Feet {
        let mut ribbon: Feet = 0;

        let side_lw = BoxSide(self.l, self.w).perimeter();
        let side_wh = BoxSide(self.w, self.h).perimeter();
        let side_lh = BoxSide(self.l, self.h).perimeter();

        let sides = [side_lw, side_wh, side_lh];
        ribbon += sides.iter().min().unwrap();
        ribbon += self.get_box_volume();

        ribbon
    }
}

struct BoxSide(Feet, Feet);

impl BoxSide {
    fn perimeter(&self) -> Feet {
        2 * self.0 + 2 * self.1
    }

    fn area(&self) -> Feet {
        self.0 * self.1
    }
}

fn main() {
    let file_path = "presents_dimensions.txt";
    let mut paper_total: Feet = 0;
    let mut ribbon_total: Feet = 0;

    match fs::read_to_string(file_path) {
        Ok(presents) => {
            for line in presents.lines() {
                let box1_input = line;

                let some_box = Box::from_string(box1_input);

                match some_box {
                    Ok(res) => {
                        paper_total += res.get_wrapping_paper_needed();
                        ribbon_total += res.get_ribbon_needed();
                    }
                    Err(err) => {
                        println!("{err}");
                    }
                };
            }
            println!("Paper needed: {paper_total}");
            println!("Ribbon needed: {ribbon_total}");
        }
        Err(e) => {
            println!("Error reading file at {file_path}. Error {e}");
        }
    }
}

fn split_string_by_delim(input: &str, delim: char) -> Vec<String> {
    input.split(delim).map(String::from).collect()
}

fn vec_string_to_u32(input: Vec<String>) -> Result<Vec<u32>, String> {
    let mut result = vec![];

    for i in input {
        match i.parse::<u32>() {
            Ok(num) => result.push(num),
            Err(_) => return Err(format!("Failed to parse '{}' as u32", i)),
        }
    }

    Ok(result)
}
