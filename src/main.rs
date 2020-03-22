extern crate rand;
extern crate term_size;

mod grid;

fn run(width: usize, height: usize) {
    let mut rng = rand::thread_rng();

    let width_without_borders = width - 2;
    // The bottom border also includes an extra newline, which is why we
    // subtract the full height by three instead of two.
    let height_without_borders = height - 3;
    let mut grid = grid::Grid::new(
        width_without_borders,
        height_without_borders,
        &mut rng);

    loop {
        static CLEAR_TERMIAL_CONTROL_CHAR: &str = "\x1B[2J";
        println!("{}{}", CLEAR_TERMIAL_CONTROL_CHAR, grid);

        static SLEEP_INTERVAL: std::time::Duration =
            std::time::Duration::from_millis(1000 / 1); // 10 FPS
        std::thread::sleep(SLEEP_INTERVAL);

        grid = grid.update();
    }
}


fn main() {
  if let Some((width, height)) = term_size::dimensions() {
    run(width, height);
  } else {
    println!("Could not determine terminal width and height. Aborting.")
  }
}
