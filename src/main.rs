#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
extern crate image;

mod args;
mod grid;


fn main() {
    let scaled = args::scale_args();
    let img = grid::ColorGrid::from_image(&scaled);
    for (i,j) in img.row_iter_from(0, 5).unwrap().enumerate() {
        println!("{}:  {:?}", i, j);
    }
    //let monos = img.separate_colors();
    //img.export("out1.png").unwrap()
}


