extern crate rand;
extern crate termsize;

static CLEAR_TERMIAL_CONTROL_CHAR: &str = "\x1B[2J";
static SLEEP_INTERVAL: std::time::Duration =
    //std::time::Duration::from_millis(1000 / 30); // 30 FPS
    std::time::Duration::from_secs(3);

static DEAD_CELL: &str = " ";
static LIVE_CELL: &str = "\u{25A0}";
static CELL_INITIALIZED_AS_LIVE_PROBABILITY: f64 = 0.1;

// The grid is stored as a vector of one-char strings for easy terminal
// rendering. Each element of the vector represents a cell in the grid. The
// cells are stored in row major form.
//
// Using strings instead of chars, despite being less efficient, because it's
// much easier to deal with unicode characters in full strings in Rust.
struct Grid {
    num_rows: usize,
    num_columns: usize,
    cells: Vec<String>
}

impl Grid {

    pub fn new<R: rand::Rng>(num_rows: usize,
                             num_columns: usize,
                             rng: &mut R) -> Grid
    {
        let num_cells = num_rows * num_columns;
        return Grid {
            num_rows: num_rows,
            num_columns: num_columns,
            cells:
                (0..num_cells)
                .map(|_| generate_initial_cell_value(rng))
                .collect()
        }
    }

    pub fn update(&self) -> Grid {
        // Source of update rules:
        // https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules
        Grid {
            num_rows: self.num_rows,
            num_columns: self.num_columns,
            cells:
                (0..self.num_cells())
                .map(|cell_num| self.next_state_of_cell_num(cell_num).to_string())
                .collect()
        }
    }

    pub fn num_cells(&self) -> usize {
        self.num_rows * self.num_columns
    }

    pub fn num_live_cells(&self) -> usize {
        self.cells.iter().filter(|&neighbour| *neighbour == LIVE_CELL).count()
    }

    pub fn num_dead_cells(&self) -> usize {
        self.cells.iter().filter(|&neighbour| *neighbour == DEAD_CELL).count()
    }

    pub fn cell_state(&self, row: i64, column: i64) -> &str {
        if row < 0 || row >= self.num_rows as i64 ||
           column < 0 || column >= self.num_columns as i64
        {
            DEAD_CELL  // out of bounds
        } else {
            let cell_num = row as usize + (column as usize * self.num_rows);
            &self.cells[cell_num]
        }
    }

    fn next_state_of_cell_num(&self, cell_num: usize) -> &str {
        let row = cell_num % self.num_columns;
        let column = cell_num / self.num_rows;
        self.next_state_of_cell(row, column)
    }

    fn next_state_of_cell(&self, row: usize, column: usize) -> &str {
        let current_state = self.cell_state(row as i64, column as i64);
        let num_live_neighbours = self.num_live_neighbours(row, column);

        if current_state == LIVE_CELL &&
           (num_live_neighbours == 2 || num_live_neighbours == 3)
        {
            LIVE_CELL
        } else if current_state == DEAD_CELL && num_live_neighbours == 3 {
            LIVE_CELL
        } else {
            DEAD_CELL
        }
    }

    fn num_live_neighbours(&self, row: usize, column: usize) -> usize {
        let signed_row = row as i64;
        let signed_column = column as i64;
        let neighbours = vec![
            self.cell_state(signed_row - 1, signed_column),  // left
            self.cell_state(signed_row + 1, signed_column),  // right
            self.cell_state(signed_row, signed_column - 1),  // above
            self.cell_state(signed_row, signed_column + 1)   // below
        ];
        neighbours.iter().filter(|&n| *n == LIVE_CELL).count()
    }

}

impl std::fmt::Display for Grid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&self.cells.join(""))?;
        Ok(())
    }
}

fn generate_initial_cell_value<R: rand::Rng>(rng: &mut R) -> String {
    if rng.gen_range(0.0, 1.0) <= CELL_INITIALIZED_AS_LIVE_PROBABILITY {
        LIVE_CELL.to_string()
    } else {
        DEAD_CELL.to_string()
    }
}

fn run(rows: usize, columns: usize) {
    let mut rng = rand::thread_rng();
    let mut grid = Grid::new(rows, columns, &mut rng);

    loop {
        //print!("{}{}", CLEAR_TERMIAL_CONTROL_CHAR, grid);
        println!("{} {}", grid.num_live_cells(), grid.num_dead_cells());

        std::thread::sleep(SLEEP_INTERVAL);

        grid = grid.update();
    }
}


fn main() {
    termsize::get().map_or_else(
        || {
            println!("Could not determine terminal width and height. Aborting.")
        },
        |size| {
            run(size.rows as usize, size.cols as usize)
        }
    );
}
