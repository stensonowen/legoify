#[macro_use]
extern crate clap;
extern crate image;

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

fn main() {
    let matches = clap::App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .arg(clap::Arg::with_name("input")
             .required(true)
             .index(1)
             .value_name("image")
             .help("source image")
             .takes_value(true))
        .arg(clap::Arg::with_name("width")
             .short("y")
             .takes_value(true)
             .help("number of lego units across"))
        .arg(clap::Arg::with_name("height")
             .short("x")
             .takes_value(true)
             .help("number of lego units tall"))
        .get_matches();

    let input_path = Path::new(matches.value_of("input").unwrap());
    let input = image::open(input_path).expect("Couldn't open image");

    //let img = image::open(&Path::new("test.jpg")).unwrap();
    //let (width, height) = img.dimensions();
    //let p = img.get_pixel(0,0);
    //p.foo();
}
