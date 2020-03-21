extern crate rand;
extern crate term_size;

static DEAD_CELL: &str = " ";
static LIVE_CELL: &str = "X";//"\u{25A0}";

// The grid is stored as a vector of one-char strings for easy terminal
// rendering. Each element of the vector represents a cell in the grid. The
// cells are stored in x major form.
//
// Using strings instead of chars, despite being less efficient, because it's
// much easier to deal with unicode characters in full strings in Rust.
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<String>
}

impl Grid {

    pub fn new<R: rand::Rng>(width: usize,
                             height: usize,
                             rng: &mut R) -> Grid
    {
        static CELL_INITIALIZED_AS_LIVE_PROBABILITY: f64 = 0.35;
        let mut generate_initial_cell_value = || {
            if rng.gen_range(0.0, 1.0) <= CELL_INITIALIZED_AS_LIVE_PROBABILITY {
                LIVE_CELL
            } else {
                DEAD_CELL
            }
        };

        let num_cells = width * height;
        return Grid {
            width: width,
            height: height,
            cells:
                (0..num_cells)
                .map(|_| generate_initial_cell_value().to_string())
                .collect()
        }
    }

    pub fn update(&self) -> Grid {
        // Source of update rules:
        // https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules
        Grid {
            width: self.width,
            height: self.height,
            cells:
                (0..self.num_cells())
                .map(|cell_num| self.next_state_of_cell_num(cell_num).to_string())
                .collect()
        }
    }

    pub fn num_cells(&self) -> usize {
        self.width * self.height
    }

    pub fn cell_state(&self, x: i64, y: i64) -> &str {
        if x < 0 || x >= self.width as i64 ||
           y < 0 || y >= self.height as i64
        {
            DEAD_CELL  // out of bounds
        } else {
            let cell_num = x as usize + y as usize * self.width;
            &self.cells[cell_num]
        }
    }

    fn next_state_of_cell_num(&self, cell_num: usize) -> &str {
        let x = cell_num % self.width;
        let y = cell_num / self.width;
        self.next_state_of_cell(x, y)
    }

    fn next_state_of_cell(&self, x: usize, y: usize) -> &str {
        let current_state = self.cell_state(x as i64, y as i64);
        let num_live_neighbours = self.num_live_neighbours(x, y);

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

    fn num_live_neighbours(&self, x: usize, y: usize) -> usize {
        let signed_x = x as i64;
        let signed_y = y as i64;
        let neighbours = vec![
            self.cell_state(signed_x - 1, signed_y),  // left
            self.cell_state(signed_x + 1, signed_y),  // right
            self.cell_state(signed_x, signed_y - 1),  // above
            self.cell_state(signed_x, signed_y + 1)   // below
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

fn run(width: usize, height: usize) {
    let mut rng = rand::thread_rng();
    let mut grid = Grid::new(width, height, &mut rng);

    loop {
        static CLEAR_TERMIAL_CONTROL_CHAR: &str = "\x1B[2J";
        print!("{}{}", CLEAR_TERMIAL_CONTROL_CHAR, grid);

        static SLEEP_INTERVAL: std::time::Duration =
            std::time::Duration::from_millis(1000 / 1); // 10 FPS
        std::thread::sleep(SLEEP_INTERVAL);

        grid = grid.update();
    }
}


fn main() {
    term_size::dimensions().map_or_else(
        || {
            println!("Could not determine terminal width and height. Aborting.")
        },
        |size| {
            run(size.0, size.1);
        }
    );
}
