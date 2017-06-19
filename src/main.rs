#[macro_use]
extern crate lazy_static;
extern crate image;

mod args;
mod grid;


fn main() {
    let scaled = args::scale_args();
    let img = grid::ColorGrid::from_image(&scaled);
    img.export("out1.png").unwrap()
}


