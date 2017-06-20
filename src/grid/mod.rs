use image::{Pixel, ImageBuffer, GenericImage, DynamicImage};
use std::collections::HashMap;
use std::slice::Iter;
use std::path::Path;
use std::io;
use std::fmt::Debug;

mod colors;
use self::colors::{Color, COLORS_MAP, nearest_color};
mod block;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T: Debug + Eq>(Vec<Vec<T>>);

impl<T: Debug + Eq> Grid<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.0.get(y).and_then(|row| row.get(x))
    }
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }
    pub fn row_iter(&self, y: usize) -> Option<Iter<T>> {
        self.0.get(y).map(|r| r.iter())
    }
    pub fn to_mono(&self, val: &T) -> MonoGrid {
        Grid(self.0
             .iter()
             .map(|row| row.iter()
                  .map(|t| t == val)
                  .collect())
             .collect())
    }
}

pub type MonoGrid = Grid<bool>;

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
            let color = self.get(x as usize, y as usize).unwrap();
            COLORS_MAP[color]
        }).save(out)
    }
    //pub fn pixels(&'static self) -> Box<Iterator<Item=&'static Color>> { 
    //    Box::new(self.0.iter().flat_map(|row| row.iter()).map(|&c| c))
    //}
    pub fn color_counts(&self) -> HashMap<&'static Color,usize> {
        let mut cols = HashMap::new();
        for pixel in self.0.iter().flat_map(|row| row.iter()) {
            let count = cols.entry(*pixel).or_insert(0);
            *count += 1;
        }
        cols
    }
    pub fn separate_colors(&self) -> ColorGrids {
        // probably a cool, better way to do this without helpers
        // TODO: revisit
        let mut c_grids: HashMap<&'static Color, MonoGrid> = HashMap::new();
        for col in self.color_counts().keys() {
            let mono = self.to_mono(col);
            c_grids.insert(col, mono);
        }
        ColorGrids(c_grids)
    }
}

pub struct ColorGrids(HashMap<&'static Color, MonoGrid>);

