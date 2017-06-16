#![feature(plugin)]
#![plugin(phf_macros)]
extern crate phf;
extern crate image;

use image::{GenericImage};
mod args;
mod colors;

fn main() {
    let args = args::get_args();
    println!("Width: \t{}", args.width);
    println!("Height:\t{}", args.height);
    println!("Original: {}x{}", args.image.width(), args.image.height());
}

