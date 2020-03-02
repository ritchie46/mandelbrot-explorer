use mandelbrot::{parse_complex, parse_pair, render_concurrently, write_image};
use std::io;
use std::io::Write;
use std::time;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(
            io::stderr(),
            "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT"
        )
        .unwrap();
        writeln!(
            io::stderr(),
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        )
        .unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    let t0 = time::Instant::now();
    render_concurrently(&mut pixels, bounds, upper_left, lower_right);
    println!("{} seconds", t0.elapsed().as_secs_f32());

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file")
}
