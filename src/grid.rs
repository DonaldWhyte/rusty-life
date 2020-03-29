extern crate rand;

use std::fmt::Write;

static DEAD: bool = false;
static LIVE: bool = true;

// The grid is stored as a vector of booleans. Each element of the vector
// re esents a cell in the grid. The cells are stored in row major form.
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>
}

impl Grid {

    pub fn new<R: rand::Rng>(width: usize,
                             height: usize,
                             rng: &mut R) -> Grid
    {
        static CELL_INITIALIZED_AS_LIVE_PROBABILITY: f64 = 0.1;
        let mut generate_initial_cell_value = || {
            if rng.gen_range(0.0, 1.0) <= CELL_INITIALIZED_AS_LIVE_PROBABILITY {
                LIVE
            } else {
                DEAD
            }
        };

        let num_cells = width * height;
        return Grid {
            width: width,
            height: height,
            cells:
                (0..num_cells)
                .map(|_| generate_initial_cell_value())
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
                .map(|cell_num| self.next_state_of_cell_num(cell_num))
                .collect()
        }
    }

    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }

    pub fn cell_state(&self, x: usize, y: usize) -> bool {
        assert!(x < self.width && y < self.height);
        let cell_num = x + y * self.width;
        self.cells[cell_num]
    }

    fn next_state_of_cell_num(&self, cell_num: usize) -> bool {
        assert!(cell_num < self.num_cells());

        let x = cell_num % self.width;
        let y = cell_num / self.width;
        assert!(x < self.width && y < self.height);

        self.next_state_of_cell(x, y)
    }

    fn next_state_of_cell(&self, x: usize, y: usize) -> bool {
        let current_state = self.cell_state(x, y);
        let num_live_neighbours = self.num_live_neighbours(x, y);

        if current_state == LIVE &&
           (num_live_neighbours == 2 || num_live_neighbours == 3)
        {
            LIVE
        } else if current_state == DEAD && num_live_neighbours == 3 {
            LIVE
        } else {
            DEAD
        }
    }

    fn num_live_neighbours(&self, x: usize, y: usize) -> usize {
        let move_coord = |base: i64, n: i64, length: i64| {
            (base + n).rem_euclid(length) as usize
        };
        let move_x = |n: i64| move_coord(x as i64, n, self.width as i64);
        let move_y = |n: i64| move_coord(y as i64, n, self.height as i64);
        let neighbours = vec![
            self.cell_state(move_x(-1), move_y(-1)),  // top left
            self.cell_state(x, move_y(-1)),           // top middle
            self.cell_state(move_x(1), move_y(-1)),   // top right
            self.cell_state(move_x(-1), y),           // middle left
            self.cell_state(move_x(1), y),            // middle right
            self.cell_state(move_x(-1), move_y(1)),   // bottom left
            self.cell_state(x, move_y(1)),            // bottom middle
            self.cell_state(move_x(1), move_y(1))     // bottom right
        ];
        neighbours.iter().filter(|&n| *n == LIVE).count()
    }

}

impl std::fmt::Display for Grid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write_top_or_bottom_border(fmt, self.width)?;

        for y in 0..self.height {
            fmt.write_char('|')?;
            for x in 0..self.width {
                fmt.write_char(state_to_char(self.cell_state(x, y)))?;
            }
            fmt.write_char('|')?;
            fmt.write_char('\n')?;
        }

        write_top_or_bottom_border(fmt, self.width)?;

        Ok(())
    }
}

fn write_top_or_bottom_border(fmt: &mut std::fmt::Formatter,
                              length: usize) -> std::fmt::Result {
    fmt.write_char('+')?;
    for _ in 0..length {
        fmt.write_char('-')?;
    }
    fmt.write_char('+')?;
    fmt.write_char('\n')?;
    Ok(())
}

fn state_to_char(state: bool) -> char {
    if state {
        'X'
    } else {
        ' '
    }
}
