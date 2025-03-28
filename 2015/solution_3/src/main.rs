#[derive(Debug)]
struct Row {
    cells: Vec<u32>,
}

impl Row {
    fn prepend(&mut self) {
        let mut new_cells: Vec<u32> = vec![0];
        new_cells.append(&mut self.cells);
        self.cells = new_cells;
    }

    fn append(&mut self) {
        self.cells.push(0);
    }
}

#[test]
fn test_prepend() {
    let mut row = Row { cells: vec![1, 2] };
    row.prepend();
    assert_eq!(vec![0, 1, 2], row.cells);
}

#[test]
fn test_append() {
    let mut row = Row { cells: vec![1, 2] };
    row.append();
    assert_eq!(vec![1, 2, 0], row.cells);
}

#[derive(Debug)]
struct Grid {
    cursor: (u32, u32),
    rows: Vec<Row>,
    width: u32,
    height: u32,
}

impl Grid {
    fn new() -> Grid {
        let row = Row { cells: vec![0] };
        let grid = Grid {
            cursor: (0, 0),
            rows: vec![row],
            width: 1,
            height: 0,
        };

        grid
    }

    fn grow_left(&mut self) {
        for row in self.rows.iter_mut() {
            row.prepend();
        }

        self.width += 1;
    }

    fn grow_right(&mut self) {
        for row in self.rows.iter_mut() {
            row.append();
        }

        self.width += 1;
    }

    fn grow_up(&mut self) {
        let mut new_rows = vec![self.new_row()];
        new_rows.append(&mut self.rows);
        self.rows = new_rows;

        self.height += 1;
    }

    fn grow_down(&mut self) {
        self.rows.push(self.new_row());

        self.height += 1;
    }

    fn new_row(&self) -> Row {
        Row {
            cells: vec![0; self.width as usize],
        }
    }
}

fn main() {
    let mut grid = Grid::new();
    println!("{:#?}", grid);
    grid.grow_left();
    println!("{:#?}", grid);
    grid.grow_right();
    println!("{:#?}", grid);
    grid.grow_up();
    println!("{:#?}", grid);
    grid.grow_down();
    println!("{:#?}", grid);
}
