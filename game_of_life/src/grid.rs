use crate::cell::Cell;
use crate::rules::RuleSet;
use rand::Rng;


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}


impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        let cells = vec![Cell::Dead; (width * height) as usize];
        Grid {width, height, cells}
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::rng();

        for x in 0..self.width {
            for y in 0..self.height {
                if rng.random::<f64>() < 0.5 {
                    self.set_cell(x, y, Cell::Alive);
                } else {
                    self.set_cell(x, y, Cell::Dead);
                }
            }
        }
    }

    pub fn print(&self) {
        println!();
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x, y).unwrap();
                match cell {
                    Cell::Alive => print!("⚪"),
                    Cell::Dead => print!("⚫"),
                }
            }
            println!();
        }
        println!();
    }

    pub fn all_dead(&self) -> bool {
        !self.cells.iter().any(|&cell| cell == Cell::Alive)
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.cells.get(index)
        } 
        else {
            None
        }
    }

    pub fn set_cell(&mut self, x: u32, y: u32, state: Cell) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.cells[index] = state;
        }
    }

    pub fn count_live_neighbors(&self, x: u32, y: u32) -> u8 {
        let mut live_neighbors: u8 = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { 
                    continue; 
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                    continue;
                }

                if let Some(Cell::Alive) = self.get_cell(nx as u32, ny as u32) {
                    live_neighbors += 1;
                }

            }
        }

        live_neighbors
    }

    #[allow(dead_code)]
    pub fn next_generation_gol(&self) -> Grid {
        let mut new_grid = self.clone();

        for (i, current_cell) in self.cells.iter().enumerate() {
            let x = i as u32 % self.width;
            let y = i as u32 / self.width;

            let alive_neighbors = self.count_live_neighbors(x, y);

            let new_cell_state = match (*current_cell, alive_neighbors) {
                // Underpopulation: <2 alive_neighbors dies
                (Cell::Alive, 0..=1) => Cell::Dead,

                // Survival: 2 < alive_neighbors < 4 survives
                (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,

                // Overpopulation: More than 3 alive_neighbors dies
                (Cell::Alive, 4..=8) => Cell::Dead,

                // Reproduction: A dead cell with exactly 3 alive_neighbors becomes alive
                (Cell::Dead, 3) => Cell::Alive,

                // Anything else leads to death
                _ => Cell::Dead,
            };

            new_grid.set_cell(x, y, new_cell_state);
        }

        new_grid
    }

    pub fn next_generation<T: RuleSet>(&self, rules: &T) -> Grid {
        let mut new_grid = self.clone();

        for (i, current_cell_state) in self.cells.iter().enumerate() {
            let x = i as u32 % self.width;
            let y = i as u32 / self.width;

            let alive_neighbors = self.count_live_neighbors(x, y);
            let new_cell_state = rules.next_state(*current_cell_state, alive_neighbors);
            new_grid.set_cell(x, y, new_cell_state);
        }

        new_grid
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_grid() {
        let grid_width = 10;
        let grid_height = 10;

        let grid = Grid::new(grid_width, grid_height);
        for y in 0..grid_height {
            for x in 0..grid_width {
                assert_eq!(*grid.get_cell(x, y).unwrap(), Cell::Dead);
            }
        }
    }

    #[test]
    fn test_set_cell_changes_state() {
        let mut grid = Grid::new(3, 3);

        let x = 1;
        let y = 2;

        grid.set_cell(x, y, Cell::Alive);
        assert_eq!(*grid.get_cell(x, y).unwrap(), Cell::Alive);

        grid.set_cell(x, y, Cell::Dead);
        assert_eq!(*grid.get_cell(x, y).unwrap(), Cell::Dead);
    }

    #[test]
    fn test_oob_get_cell_returns_none() {
        let grid = Grid::new(5, 5);

        assert!(grid.get_cell(5, 5).is_none());
        assert!(grid.get_cell(10, 0).is_none());
    }
}
