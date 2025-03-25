#[derive(Debug)]
struct Row {
    cells: Vec<u32>,
}

#[derive(Debug)]
struct Grid {
    coordinate: (u32, u32),
    rows: Vec<Row>,
}

impl Grid {
    fn new() -> Grid {
        let mut row = Row { cells: vec![0] };
        let mut grid = Grid {
            rows: vec![row],
            coordinate: (0, 0),
        };

        grid
    }
}

fn main() {
    let mut grid = Grid::new();
    println!("{:?}", grid);
}
