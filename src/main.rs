extern crate getopts;
extern crate image;

use getopts::Options;
use image::{Pixel, Primitive};
use std::path::Path;

fn usage(program: &str, opts: &Options) -> ! {
	let brief = format!("Usage: {} [options] <image>", program);
    print!("{}", opts.usage(&brief));
    ::std::process::exit(1);
}

fn delta<S: Primitive, T: Pixel<Subpixel=S>>(x: T, y: T) -> S {
    // Euclidean distance square
    // https://en.wikipedia.org/wiki/Color_difference#Euclidean
    // TODO: incorporate alpha values?
    x.to_rgb().data.into_iter()
        .zip(y.to_rgb().data.into_iter())
        .map(|(&a,&b)| (a-b)*(a-b))
        .fold(S::zero(), |acc,i| acc+i)
}

fn main() {
    let mut args = ::std::env::args();
    let program = args.nth(0).unwrap();
    let args: Vec<String> = args.collect();

    let mut opts = Options::new();
    opts.reqopt("", "image",  "input image", "IMAGE");
    opts.optopt("x", "width",  "output width in lego units", "X");
    opts.optopt("y", "height", "output height in lego units", "Y");
    opts.optopt("h", "help",   "print this message", "");
    //let matches = opts.parse(&args).unwrap_or_else(|_| usage(&program, &opts));
    let matches = opts.parse(&args).unwrap();
    if matches.opt_present("i") == false || matches.opt_present("h") {
        usage(&program, &opts);
    }

    //let input_path = Path::new(matches.value_of("input").unwrap());
    //let input = image::open(input_path).expect("Couldn't open image");

    //let img = image::open(&Path::new("test.jpg")).unwrap();
    //let (width, height) = img.dimensions();
    //let p = img.get_pixel(0,0);
    //p.foo();
}
