use std::collections::HashSet;
use crate::grid::Cell;

pub trait Pattern {
    fn at(&self, x: u32, y: u32) -> HashSet<Cell>;
}

pub struct Blinker;
impl Pattern for Blinker {
    fn at(&self, x: u32, y: u32) -> HashSet<Cell> {
        let mut cells = HashSet::new();
        for i in 0..=2 {
            cells.insert(Cell::new(x+i, y));
        }
        cells
    }
}


pub struct Glider;
impl Pattern for Glider {
    fn at(&self, x: u32, y: u32) -> HashSet<Cell> {
        let mut cells = HashSet::new();
        for i in 0..=2 {
            cells.insert(Cell::new(x+i, y+2));
        }
        cells.insert(Cell::new(x+1, y));
        cells.insert(Cell::new(x+2, y+1));

        cells
    }
}
