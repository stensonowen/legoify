extern crate image;

use image::{Pixel, Primitive, GenericImage};
mod args;

fn main() {
    let args = args::get_args();
    println!("Width: \t{}", args.width);
    println!("Height:\t{}", args.height);
    println!("Original: {}x{}", args.image.width(), args.image.height());
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

