#[macro_use]
extern crate lazy_static;
extern crate image;

use image::{GenericImage};
mod args;
mod colors;


fn main() {
    let args = args::get_args();
    println!("Width: \t{}", args.width);
    println!("Height:\t{}", args.height);
    println!("Original: {}x{}", args.image.width(), args.image.height());
    let img = colors::round_image(&args);
    img.save("out.png").unwrap();
}


