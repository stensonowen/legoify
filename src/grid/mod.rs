use image::{Pixel, ImageBuffer, GenericImage, DynamicImage};
use std::collections::HashMap;
use std::slice::Iter;
use std::path::Path;
use std::io;

mod color;
use self::color::{Color, COLORS_MAP, nearest_color};
mod block;
use self::block::Block;

#[derive(Debug, PartialEq, Eq)]
pub struct ColorGrid(Vec<Vec<&'static Color>>);  // TODO: Option<Color> ?

impl ColorGrid {
    pub fn from_image(input: &DynamicImage) -> Self {
        let pixels: Vec<_> = input.pixels().collect();
        ColorGrid(pixels.chunks(input.width() as usize)
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
        ColorGrid(grid)
    }
    fn get(&self, x: usize, y: usize) -> Option<&'static Color> {
        self.0.get(y).and_then(|row| row.get(x).map(|&c| c))
    }
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
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
    pub fn color_counts(&self) -> HashMap<&Color,usize> {
        let mut cols = HashMap::new();
        // include absent colors?
        //let mut cols: HashMap<&Color,usize> = COLORS_MAP.keys()
        //    .map(|c| (c,0))
        //    .collect();
        for pixel in self.0.iter().flat_map(|row| row.iter()) {
            //let count = cols.get_mut(pixel).unwrap();
            let count = cols.entry(*pixel).or_insert(0);
            *count += 1;
        }
        cols
    }
    pub fn row_iter(&self, y: usize) -> Option<Iter<&Color>> {
        self.0.get(y).map(|r| r.iter())
    }
}

struct BlockGrid(HashMap<(usize,usize),Block>);

struct Region {
    color: &'static Color,
}


