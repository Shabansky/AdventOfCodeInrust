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
struct Cursor {
    x: u32,
    y: u32,
}

impl Cursor {
    fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn move_down(&mut self) {
        self.y += 1;
    }
}

#[derive(Debug)]
struct Grid {
    cursor: Cursor,
    rows: Vec<Row>,
    width: u32,
    height: u32,
}

impl Grid {
    fn new() -> Grid {
        let row = Row { cells: vec![0] };
        let grid = Grid {
            cursor: Cursor { x: 0, y: 0 },
            rows: vec![row],
            width: 1,
            height: 1,
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

    fn increment_cell(&mut self) {
        let y_coord = self.cursor.y as usize;
        let x_coord = self.cursor.x as usize;
        self.rows[y_coord].cells[x_coord] += 1;
    }

    fn move_right(&mut self) {
        if self.cursor.x + 1 == self.width {
            self.grow_right();
        }

        self.cursor.move_right();
        self.increment_cell();
    }

    fn move_left(&mut self) {
        if self.cursor.x == 0 {
            self.grow_left();
        } else {
            self.cursor.move_left();
        }

        self.increment_cell();
    }

    fn move_up(&mut self) {
        if self.cursor.y == 0 {
            self.grow_up();
        } else {
            self.cursor.move_up();
        }

        self.increment_cell();
    }

    fn move_down(&mut self) {
        if self.cursor.y + 1 == self.height {
            self.grow_down();
        }

        self.cursor.move_down();
        self.increment_cell();
    }
}

fn main() {
    let mut grid = Grid::new();
    grid.move_right();
    grid.move_down();
    grid.move_left();
    grid.move_up();
    grid.move_left();
    grid.move_up();
    grid.move_right();
    grid.move_down();
    draw(&grid);
}

fn draw(grid: &Grid) {
    for row in grid.rows.iter() {
        for cell in row.cells.iter() {
            print!("{}", cell);
        }
        print!("\n");
    }
}
