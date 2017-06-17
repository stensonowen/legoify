#[macro_use]
extern crate lazy_static;
extern crate image;

mod args;
mod colors;


fn main() {
    let scaled = args::scale_args();
    let img = colors::round_image(&scaled);
    img.save("out.png").unwrap();
}


