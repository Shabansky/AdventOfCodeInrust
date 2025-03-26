#[derive(Debug)]
struct Row {
    cells: Vec<u32>,
}

#[derive(Debug)]
struct Grid {
    cursor: (u32, u32),
    rows: Vec<Row>,
}

impl Row {
    fn prepend(&mut self) {
        let mut new_cells: Vec<u32> = vec![0];
        new_cells.append(&mut self.cells);
        self.cells = new_cells;
    }
}

#[test]
fn test_prepend() {
    let mut row = Row { cells: vec![1, 2] };
    row.prepend();
    assert_eq!(vec![0, 1, 2], row.cells);
}

impl Grid {
    fn new() -> Grid {
        let mut row = Row { cells: vec![0] };
        let mut grid = Grid {
            rows: vec![row],
            cursor: (0, 0),
        };

        grid
    }
}

fn main() {
    let mut grid = Grid::new();
    println!("{:?}", grid);
}
