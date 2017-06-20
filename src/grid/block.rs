use std::collections::HashMap;

/*
use std::slice::Iter;
use super::{Color, ColorGrid};
use std::ops::Range;
// A horizontal/vertical unit-width line segment (row/column) composed of zero 
//  or more contiguous populated swaths
// Every row and column in a grid is a ColinearSegment, but the contents of it
//  are the lines of legos that lie upon it (or cover it entirely)
struct ColinearSegments(Vec<Range<usize>>);

pub struct ColorMap {
    // every color's area is composed of one ColinearSegment per column (or row)
    // if there is no block of this color in an image, each element CS is empty
    //cols: Vec<ColinearSegments>,
    rows: Vec<ColinearSegments>,      // alternate representation
}

impl ColinearSegments {
    fn from_row(row: Iter<&Color>) {}
}

impl ColorMap {
    fn from_color_grid(cg: &ColorGrid) {
    }
}
*/

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Block {
    OneByOne,
    TwoByOne,
    TwoByTwo,
    FourByOne,
    FourByTwo,
    EightByTwo
    //OneByOne,
    //OneByTwo,
    //TwoByTwo,
    //TwoByFour,
    //TwoByEight,
}


lazy_static! {
    pub static ref BLOCK_RATIOS: HashMap<Block, f64> = {
        let mut map = HashMap::new();
        // uhh I'm basing this on memories from 10 years ago
        // is there a better authority on this? besides acquiring/counting legos?
        map.insert(Block::OneByOne,     0.05);
        map.insert(Block::TwoByOne,     0.10);
        map.insert(Block::TwoByTwo,     0.20);
        map.insert(Block::FourByOne,    0.10);
        map.insert(Block::FourByTwo,    0.40);
        map.insert(Block::EightByTwo,   0.15);
        map
    };
}
