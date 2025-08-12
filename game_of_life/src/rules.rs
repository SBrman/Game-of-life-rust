use crate::cell::Cell;

pub trait RuleSet {
    fn next_state(&self, cell: Cell, live_neighbors: u8) -> Cell;
}

pub struct GameOfLife;

impl RuleSet for GameOfLife {
    fn next_state(&self, cell: Cell, live_neighbors: u8) -> Cell {
        match (cell, live_neighbors) {
            // A live cell with 2 or 3 neighbor survives
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
            // A dead cell with exactly 3 neighbors becomes alive
            (Cell::Dead, 3) => Cell::Alive,
            // All other cases lead to death
            _ => Cell::Dead,
        }
    }
}


pub struct HighLife;

impl RuleSet for HighLife {
    fn next_state(&self, cell: Cell, live_neighbors: u8) -> Cell {
        match (cell, live_neighbors) {
            // A live cell with 2 or 3 neighbor survives
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
            // A dead cell with exactly 3 neighbors becomes alive
            (Cell::Dead, 3) | (Cell::Dead, 6) => Cell::Alive,
            // All other cases lead to death
            _ => Cell::Dead,
        }
    }
}
