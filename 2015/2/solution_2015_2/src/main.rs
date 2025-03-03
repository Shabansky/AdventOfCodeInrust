type Feet = i32;

struct Box {
    l: Feet,
    w: Feet,
    h: Feet,
}

impl Box {
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
    let box1 = Box { l: 2, w: 3, h: 4 };
    assert_eq!(58, box1.get_wrapping_paper_needed());

    let box2 = Box { l: 1, w: 1, h: 10 };
    assert_eq!(43, box2.get_wrapping_paper_needed());
}
