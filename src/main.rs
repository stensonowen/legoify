#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate docopt;

use image::{Pixel, Primitive};
use std::path::Path;

fn delta<S: Primitive, T: Pixel<Subpixel=S>>(x: T, y: T) -> S {
    // Euclidean distance square
    // https://en.wikipedia.org/wiki/Color_difference#Euclidean
    // TODO: incorporate alpha values?
    x.to_rgb().data.into_iter()
        .zip(y.to_rgb().data.into_iter())
        .map(|(&a,&b)| (a-b)*(a-b))
        .fold(S::zero(), |acc,i| acc+i)
}

const USAGE: &'static str = "
Usage: legoify [options] <image>
       legoify (--help | --version)

Options:
    -h, --help     Show this message
    -v, --version  Show the version
    -x, --width    Specify the output width in lego units
    -y, --height   Specify the output height in lego units
";

fn main() {
    let input_path = Path::new(matches.value_of("input").unwrap());
    let input = image::open(input_path).expect("Couldn't open image");

    //let img = image::open(&Path::new("test.jpg")).unwrap();
    //let (width, height) = img.dimensions();
    //let p = img.get_pixel(0,0);
    //p.foo();
}
