extern crate docopt;
extern crate rand;
extern crate term_size;

mod grid;

const USAGE: &'static str = "
Usage: rusty-life [options]

Options:
    --fps=<fps>  Frames/grid updates to render/run per second [default: 5].
";

fn main() {
    let args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    let fps = args.get_str("--fps").parse::<u64>().unwrap();

    if let Some((width, height)) = term_size::dimensions() {
      run(width, height, fps);
    } else {
      println!("Could not determine terminal width and height. Aborting.")
    }
}

fn run(width: usize, height: usize, fps: u64) {
    let mut rng = rand::thread_rng();

    let width_without_borders = width - 2;
    // The bottom border also includes an extra newline, which is why we
    // subtract the full height by three instead of two.
    let height_without_borders = height - 3;
    let mut grid = grid::Grid::new(
        width_without_borders,
        height_without_borders,
        &mut rng);

    let sleep_interval = std::time::Duration::from_millis(1000 / fps);

    loop {
        static CLEAR_TERMIAL_CONTROL_CHAR: &str = "\x1B[2J";
        print!("{}{}", CLEAR_TERMIAL_CONTROL_CHAR, grid);

        std::thread::sleep(sleep_interval);

        grid = grid.update();
    }
}
