use core::panic;
use std::time::Duration;
use std::{fs, thread};

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
    visited_cells: u32,
}

impl Grid {
    fn new() -> Grid {
        let row = Row { cells: vec![0] };
        let grid = Grid {
            cursor: Cursor { x: 0, y: 0 },
            rows: vec![row],
            width: 1,
            height: 1,
            visited_cells: 1,
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
        let cell = self.rows[y_coord].cells[x_coord];

        if cell == 0 {
            self.visited_cells += 1;
        }
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
    let file_path = "santa_directions.txt";

    match fs::read_to_string(file_path) {
        Ok(presents) => {
            let mut grid = Grid::new();

            for char in presents.chars() {
                match char {
                    '<' => grid.move_left(),
                    '>' => grid.move_right(),
                    '^' => grid.move_up(),
                    'v' => grid.move_down(),
                    _ => panic!("Invalid character. Exiting."),
                }

                // Feel free to uncomment the below for some terminal fun!
                // thread::sleep(Duration::from_millis(50));
                // print!("{}[2J", 27 as char);
                // draw(&grid);
            }
            println!("{}", grid.visited_cells);
        }
        Err(e) => {
            println!("Error reading file at {file_path}. Error {e}");
        }
    }
}

fn draw(grid: &Grid) {
    println!("{} cells visited", grid.visited_cells);
    for row in grid.rows.iter() {
        for cell in row.cells.iter() {
            print!("{}", cell);
        }
        print!("\n");
    }
}
