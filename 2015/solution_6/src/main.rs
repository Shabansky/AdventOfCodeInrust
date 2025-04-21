const SQUARE_SIDE: usize = 1000;

fn main() {
    //Map allocated on heap via vec as a 1000x1000 on an array will exceed the stack memory limit.
    let map = vec![vec![0; SQUARE_SIDE]; SQUARE_SIDE];

    for row in &map {
        for col in row {
            print!("{col} ");
        }
        println!("");
    }
}
