use std::fmt;
use super::Grid;

pub struct MonoGrid {
    //color: 
    pub presence:   Grid<bool>,
    pub block:  Grid<u32>,
    block_count: u32,
}

impl MonoGrid {
    pub fn from(mono: Grid<bool>) -> Self {
        let (w, h) = (mono.width(), mono.height());
        MonoGrid {
            presence: mono,
            block:    Grid::blank(0u32, w, h),
            block_count: 1
        }
    }
    // contiguity tests:
    // a given point will have a nonnegative number of true points in any direction
    // a column-wise test returns the true points going down
    // a row-wise test returns the true points going right
    // the point being tested must be true (?)
    // a `0` means not even the point itself is true
    // a `1` means only the point is true
    pub fn contig_col(&self, x: usize, y: usize) -> usize {
        self.presence.col_iter_from(x, y).take_while(|&&c| c).count()
    }
    pub fn contig_row(&self, x: usize, y: usize) -> usize {
        self.presence.row_iter_from(y, x).take_while(|&&c| c).count()
    }

    pub fn naive_make_row(&mut self, y: usize) {
        let mut first_present = self.presence.row_iter(y).take_while(|&&c| !c).count();
        if first_present == self.presence.width() {
            // row is empty
            return;
        }
        let mut length = self.contig_row(first_present, y);
        for &val in [4, 2, 1].iter() { // TODO: make a constant or enum or something
            let n = length / val;
            length %= val;
            // todo: add mut_row_iter() or something for these
            for _ in 0..n {
                // add a new block
                for _ in 0..val {
                    // mark all cells as belonging to that block
                    self.block.set(first_present, y, self.block_count);
                    first_present += 1;
                }
                self.block_count += 1;
            }
        }
    }

}

impl fmt::Debug for MonoGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "MonoGrid with {} blocks:\n", self.block_count)?;
        writeln!(f, "{:?}{:?}", self.presence, self.block)
    }
}
