#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
extern crate image;

mod args;
mod grid;


fn main() {
    let scaled = args::scale_args();
    let img = grid::ColorGrid::from_image(&scaled);
    //for (i,j) in img.row_iter_from(100, 5).enumerate() { println!("{}:  {:?}", i, j); }
    let monos = img.separate_colors();

    let w = grid::colors::Color::White;
    let ww = monos.0[&w].clone();
    let mut white = grid::mono::MonoGrid::from(ww);
    //println!("{:?}", white);
    //let s: Vec<_> = white.presence.row_iter(1).collect();
    //println!("{:?}", s);
    //return;
    println!("{:?}", white);
    white.naive_make_row(1);
    println!("{:?}", white);

    
    //for x in 0 .. white.width() { println!("{}: {}", x, white.contig_col(x, x)); }

    //img.export("out1.png").unwrap()
}


