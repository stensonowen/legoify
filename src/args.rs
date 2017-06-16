use std::path::PathBuf;
use std::io::{stderr, Write};
use image::{self, DynamicImage, GenericImage};

fn usage(prog: &str) -> ! {
    // if I have to switch to nightly remember to use eprint(ln) instead
    writeln!(stderr(), "
Usage: {} [options] <image>

Options:
    -h, --help     Show this message
    -x, --width    Specify the output width in lego units
    -y, --height   Specify the output height in lego units
",
        prog).unwrap();
    ::std::process::exit(1);
}

fn usage_and(msg: &str, prog: &str) -> ! {
    stderr().write_all(msg.as_bytes()).unwrap();
    usage(prog);
}

fn parse_pos_num(n: &str, prog: &str) -> u32 {
    match n.parse() {
        Ok(m) if m > 0 => m,
        _ => usage_and("Dimension must be a positive number", prog)
    }
}

pub struct Args {
    pub width: u32,
    pub height: u32,
    pub image: DynamicImage,
}

pub fn get_args() -> Args {
    let mut args = ::std::env::args();
    let prog = args.nth(0).unwrap();
    let (mut w, mut h, mut p) = (0, 0, None);
    let (mut expect_w, mut expect_h) = (false, false);
    for arg in args {
        match &arg[..] {
            n if expect_w => { expect_w = false; w = parse_pos_num(&n, &prog) },
            n if expect_h => { expect_h = false; h = parse_pos_num(&n, &prog) },
            "-h" | "--help" => usage(&prog),
            "-x" | "--width" => expect_w = true,
            "-y" | "--height" => expect_h = true,
            n if n.starts_with("-x=") => w = parse_pos_num(&n[3..], &prog),
            n if n.starts_with("--width=") => w = parse_pos_num(&n[8..], &prog),
            n if n.starts_with("-y=") => h = parse_pos_num(&n[3..], &prog),
            n if n.starts_with("--height=") => h = parse_pos_num(&n[9..], &prog),
            i => if p.is_some() {
                usage_and("Found unexpected arg: `{}`", &prog)
            } else {
                let pb = PathBuf::from(i); // allocation :(
                if pb.exists() == false {
                    usage_and("Must supply a valid image path", &prog);
                } else {
                    p = Some(pb);
                }
            }
        }
    }
    if expect_w || expect_h {
        usage_and("Final argument incomplete", &prog);
    }
    let path: PathBuf = p.unwrap_or_else(|| usage(&prog));
    let pic = image::open(&path).expect("Could open source image");
    let (pic_w, pic_h) = pic.dimensions();
    match (w, h) {
        (0,0) => Args { width: pic_w, height: pic_h, image: pic },
        (x,0) => Args { width: x, height: pic_h*x/pic_w, image: pic },
        (0,y) => Args { width: pic_w*y/pic_h, height: y, image: pic },
        (x,y) => Args { width: x, height: y, image: pic },
    }
}
