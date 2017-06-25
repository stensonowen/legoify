use image::{Pixel, ImageBuffer, GenericImage, DynamicImage};
use std::collections::HashMap;
use std::slice::Iter;
use std::path::Path;
use std::{io, fmt};
use std::fmt::Debug;
use std::hash::Hash;

pub mod colors; // TODO: un-pub
use self::colors::{Color, COLORS_MAP, nearest_color};
mod block;
pub mod mono;

#[derive(PartialEq, Eq, Clone)]
pub struct Grid<T: Debug + Eq + Clone>(Vec<Vec<T>>);

impl<T: Debug + Eq + Clone> Grid<T> {
    fn blank(default: T, width: usize, height: usize) -> Grid<T> {
        let row: Vec<_> = (0..width).map(|_| default.clone()).collect();
        let all: Vec<_> = (0..height).map(|_| row.clone()).collect();
        Grid(all)
    }
    // basic access 
    fn get(&self, x: usize, y: usize) -> &T {
        self.0.get(y).and_then(|row| row.get(x)).unwrap()
    }
    fn set(&mut self, x: usize, y: usize, val: T) {
        let ref mut col = self.0[y];
        col[x] = val;
    }
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn row_iter(&self, y: usize) -> Iter<T> {
        self.0.get(y).map(|r| r.iter()).unwrap()
    }
    pub fn row_iter_from(&self, y: usize, x: usize) -> Iter<T> { // beware footgun
        self.0.get(y).map(|r| r[x..].iter()).unwrap()
    }

    pub fn col_iter<'a>(&'a self, x: usize) -> Box<Iterator<Item=&'a T> + 'a> {
        self.col_iter_from(x, 0)
    }
    pub fn col_iter_from<'a>(&'a self, x: usize, y: usize) 
        -> Box<Iterator<Item=&'a T> + 'a>
    {
        // pretty gross, maybe rewrite in the future?
        // would be an interesting candidate for `-> impl Iterator` but idk
        // maybe that'll happen when I get forced onto nightly
        Box::new(self.0[y..].iter().map(move |col| col.get(x).unwrap()))
    }

    pub fn to_mono(&self, val: &T) -> Grid<bool> {
        Grid(self.0
             .iter()
             .map(|row| row.iter()
                  .map(|t| t == val)
                  .collect())
             .collect())
    }
}

impl<T: Debug + Eq + Hash + Copy> Grid<T> {
    fn counts(&self) -> HashMap<T,usize> {
        let mut counts = HashMap::new();
        for cell in self.0.iter().flat_map(|row| row.iter()) {
            let count = counts.entry(*cell).or_insert(0);
            *count += 1;
        }
        counts
    }
}

impl<T: Debug + Eq + Clone> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0 .. self.height() {
            for c in self.row_iter(r) {
                let preview = format!("{:?}", c);
                let short = if preview.len() > 3 {
                    &preview[..3]
                } else {
                    &preview[..]
                };
                write!(f, "{:3} ", short)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}


//pub type MonoGrid = Grid<bool>;
pub type MonoGrid_ = Grid<bool>;

pub type ColorGrid = Grid<&'static Color>;

impl ColorGrid {
    pub fn from_image(input: &DynamicImage) -> Self {
        let pixels: Vec<_> = input.pixels().collect();
        Grid(pixels.chunks(input.width() as usize)
            .map(|row| row.iter()
                 .map(|&(_,_,rgba)| nearest_color(&rgba.to_rgb()))
                 .collect())
            .collect())
    }
    fn _from_image_2(input: &DynamicImage) -> Self {
        let (w,h) = (input.width() as usize, input.height() as usize);
        let mut grid: Vec<_> = (0..h).map(|_| Vec::with_capacity(w)).collect();
        for (_,y,rgba) in input.pixels() {
            grid[y as usize].push(nearest_color(&rgba.to_rgb()));
        }
        Grid(grid)
    }
    pub fn export<P: AsRef<Path>>(&self, out: P) -> io::Result<()> {
        let (w, h) = (self.width() as u32, self.height() as u32);
        ImageBuffer::from_fn(w, h, |x,y| { 
            let color = self.get(x as usize, y as usize);
            COLORS_MAP[color]
        }).save(out)
    }
    //pub fn pixels(&'static self) -> Box<Iterator<Item=&'static Color>> { 
    //    Box::new(self.0.iter().flat_map(|row| row.iter()).map(|&c| c))
    //}
    pub fn color_counts(&self) -> HashMap<&'static Color,usize> {
        self.counts()
    }
    pub fn separate_colors(&self) -> ColorGrids {
        // probably a cool, better way to do this without helpers
        // TODO: revisit
        let mut c_grids: HashMap<&'static Color, MonoGrid_> = HashMap::new();
        for col in self.color_counts().keys() {
            let mono = self.to_mono(col);
            c_grids.insert(col, mono);
        }
        ColorGrids(c_grids)
    }
}

pub struct ColorGrids(pub HashMap<&'static Color, MonoGrid_>); // TODO: un-pub

