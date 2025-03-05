type Feet = u32;

struct Box {
    l: Feet,
    w: Feet,
    h: Feet,
}

impl Box {
    fn from_string(string: String) -> Result<Self, String> {
        let dimensions_vec = convert_vec_string_to_u32(split_string_by_delim(string));

        if dimensions_vec.is_err() {
            return Err(String::from("Malformed input"));
        }

        if let [l, w, h] = dimensions_vec.unwrap()[..] {
            return Ok(Box { l, w, h });
        }

        Err(String::from("Malformed input"))
    }

    fn get_wrapping_paper_needed(&self) -> Feet {
        let mut wrapping_paper = 0;

        let side_lw = self.l * self.w;
        let side_wh = self.w * self.h;
        let side_lh = self.l * self.h;

        let sides = [side_lw, side_wh, side_lh];
        let smallest_side = sides.iter().min().unwrap();

        wrapping_paper += side_lw * 2 + side_wh * 2 + side_lh * 2;
        wrapping_paper += smallest_side;

        wrapping_paper
    }
}

fn main() {
    let box1_input = String::from("2x3x4");

    let some_box = Box::from_string(box1_input);

    let paper = match some_box {
        Ok(res) => res.get_wrapping_paper_needed(),
        Err(err) => {
            println!("{err}");
            0
        }
    };

    println!("{paper}");
}

fn split_string_by_delim(input: String) -> Vec<String> {
    let mut delims: Vec<usize> = vec![];

    for (i, v) in input.chars().enumerate() {
        if v == 'x' {
            delims.push(i);
        }
    }

    let mut last_delim: usize = 0;
    let mut segments: Vec<String> = vec![];
    for i in delims {
        let slice = &input[last_delim..i];

        segments.push(String::from(slice));
        //Skip one char to the right to avoid the "x"
        last_delim = i + 1;
    }
    if last_delim < input.len() {
        let slice = &input[last_delim..input.len()];
        segments.push(String::from(slice));
    }

    segments
}

fn convert_vec_string_to_u32(input: Vec<String>) -> Result<Vec<u32>, bool> {
    let mut result = vec![];

    for i in input {
        if !string_slice_is_num(&i) {
            return Err(false);
        }
        result.push(string_to_int(&i));
    }

    Ok(result)
}

fn string_slice_is_num(slice: &str) -> bool {
    !slice.parse::<u32>().is_err()
}

fn string_to_int(slice: &str) -> u32 {
    slice.parse::<u32>().unwrap()
}
