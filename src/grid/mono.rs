use super::MonoGrid;

impl MonoGrid {
    // contiguity tests:
    // a given point will have a nonnegative number of true points in any direction
    // a column-wise test returns the true points going down
    // a row-wise test returns the true points going right
    // the point being tested must be true (?)
    // a `0` means not even the point itself is true
    // a `1` means only the point is true
    fn contig_col(&self, x: usize, y: usize) -> u8 {
        unimplemented!()
    }
    fn foo(&self) -> bool {
        *self.get(0,0).unwrap()
    }

}
