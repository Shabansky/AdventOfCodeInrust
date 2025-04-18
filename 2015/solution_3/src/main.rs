use core::panic;
use std::fs;

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
    active_cursor: usize,
    cursors: Vec<Cursor>,
    rows: Vec<Row>,
    width: u32,
    height: u32,
    visited_cells: u32,
}

impl Grid {
    fn new() -> Grid {
        let row = Row { cells: vec![1] };
        let grid = Grid {
            active_cursor: 0,
            cursors: vec![Cursor { x: 0, y: 0 }],
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

        for (index, cursor) in self.cursors.iter_mut().enumerate() {
            if index != self.active_cursor {
                cursor.x += 1;
            }
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

        for (index, cursor) in self.cursors.iter_mut().enumerate() {
            if index != self.active_cursor {
                cursor.y += 1;
            }
        }

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

    fn add_cursor(&mut self) {
        self.cursors.push(Cursor { x: 0, y: 0 });
    }

    fn rotate_cursors(&mut self) {
        let mut next_cursor = self.active_cursor + 1;

        if let None = self.cursors.get(next_cursor) {
            next_cursor = 0;
        };

        self.set_active_cursor(next_cursor);
    }

    fn get_active_cursor(&self) -> &Cursor {
        &self.cursors[self.active_cursor]
    }

    fn get_active_cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursors[self.active_cursor]
    }

    fn set_active_cursor(&mut self, index: usize) {
        let Some(_) = self.cursors.get(index) else {
            panic!("Trying to retrieve non-existing cursor with index {index}")
        };

        self.active_cursor = index;
    }

    fn increment_cell(&mut self) {
        let active_cursor = self.get_active_cursor();
        let y_coord = active_cursor.y as usize;
        let x_coord = active_cursor.x as usize;
        let cell = self.rows[y_coord].cells[x_coord];

        if cell == 0 {
            self.visited_cells += 1;
        }
        self.rows[y_coord].cells[x_coord] += 1;
    }

    fn move_right(&mut self) {
        let active_cursor = self.get_active_cursor();
        if active_cursor.x + 1 == self.width {
            self.grow_right();
        }

        self.get_active_cursor_mut().move_right();

        self.increment_cell();
    }

    fn move_left(&mut self) {
        let active_cursor = self.get_active_cursor_mut();
        if active_cursor.x == 0 {
            //Explicit move of the cursor is not needed as it will maintain a x of 0
            self.grow_left();
        } else {
            active_cursor.move_left();
        }

        self.increment_cell();
    }

    fn move_up(&mut self) {
        let active_cursor = self.get_active_cursor_mut();
        if active_cursor.y == 0 {
            //Explicit move of the cursor is not needed as it will maintain a y of 0
            self.grow_up();
        } else {
            active_cursor.move_up();
        }

        self.increment_cell();
    }

    fn move_down(&mut self) {
        let active_cursor = self.get_active_cursor();
        if active_cursor.y + 1 == self.height {
            self.grow_down();
        }

        self.get_active_cursor_mut().move_down();

        self.increment_cell();
    }
}

fn main() {
    let file_path = "santa_directions.txt";

    let santa_map = match fs::read_to_string(file_path) {
        Ok(map) => map,
        Err(e) => {
            panic!("Error reading file at {file_path}. Error {e}");
        }
    };

    let mut grid = Grid::new();
    grid.add_cursor();

    for char in santa_map.chars() {
        match char {
            '<' => grid.move_left(),
            '>' => grid.move_right(),
            '^' => grid.move_up(),
            'v' => grid.move_down(),
            _ => panic!("Invalid character. Exiting."),
        }

        grid.rotate_cursors();

        // Feel free to uncomment the below for some terminal fun!
        // use std::thread;
        // use std::time::Duration;
        // thread::sleep(Duration::from_millis(10));
        // print!("{}[2J", 27 as char);
        // draw(&grid);
    }
    println!("{}", grid.visited_cells);
}

fn draw(grid: &Grid) {
    println!("{} cells visited", grid.visited_cells);
    for row in grid.rows.iter() {
        for cell in row.cells.iter() {
            if *cell == 0 {
                print!(" ");
            } else {
                print!("{}", cell);
            }
        }
        print!("\n");
    }
}
