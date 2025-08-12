use std::collections::{HashSet, HashMap};
use crate::rules::RuleSet;
use crate::patterns::Pattern;
use std::hash::{Hash, Hasher, DefaultHasher};
use std::sync::OnceLock;
use rand::Rng;


const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
     (0, -1),           (0, 1),
     (1, -1),  (1, 0),  (1, 1),
];


#[derive(Hash, Eq, Clone, Copy, Debug, PartialEq, Ord, PartialOrd)]
pub struct Cell(u32, u32);

impl Cell {
    pub fn new(x: u32, y: u32) -> Self { Cell(x, y) }
    pub fn x(&self) -> u32 { self.0 }
    pub fn y(&self) -> u32 { self.1 }
}


#[derive(Eq, Clone, Debug, PartialEq)]
pub struct Grid {
    width: u32,
    height: u32,
    pub alive_cells: HashSet<Cell>,
    pub new_alive_cells: HashSet<Cell>,
    pub new_dead_cells: HashSet<Cell>,
    cached_hash: OnceLock<u64>,
}

impl Hash for Grid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let computed_hash = self.cached_hash.get_or_init(|| {
            let mut hasher = DefaultHasher::new();
            self.width.hash(&mut hasher);
            self.height.hash(&mut hasher);

            let mut combined: u64 = 0;
            for cell in &self.alive_cells {
                let mut cell_hasher = DefaultHasher::new();
                cell.hash(&mut cell_hasher);
                combined ^= cell_hasher.finish();
            }
            combined.hash(&mut hasher);

            hasher.finish()
        });
    computed_hash.hash(state);
    }
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        Self { 
            width, 
            height, 
            alive_cells: HashSet::new(),
            new_alive_cells: HashSet::new(),
            new_dead_cells: HashSet::new(),
            cached_hash: OnceLock::new()
        }
    }

    pub fn randomize(&mut self, density: f64) {
        let mut rng = rand::rng();
        let total_cells = (self.width * self.height) as f64;
        let cells_to_create = (total_cells * density) as u32;
        
        self.alive_cells.clear();
        for _ in 0..cells_to_create {
            let x = rng.random_range(0..self.width);
            let y = rng.random_range(0..self.height);
            self.alive_cells.insert(Cell::new(x, y));
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        print!("{}[2J", 27 as char);
        println!();
        for y in 0..self.height {
            for x in 0..self.width {
                let temp_cell = Cell::new(x, y);
                if self.is_alive(&temp_cell) {
                    print!("⚪");
                } else {
                    print!("⚫");
                }
            }
            println!();
        }
        println!();
    }

    pub fn neighbors(&self, cell: &Cell) -> [Cell; 8] {
        let mut neighbors = [Cell(0, 0); 8];
        for (i, &(dx, dy)) in NEIGHBOR_OFFSETS.iter().enumerate() {
            neighbors[i] = self.wrapped_cell(
                cell.x() as i32 + dx,
                cell.y() as i32 + dy
            )
        }
        neighbors
    }

    pub fn next_generation<T: RuleSet>(&self, rules: &T) -> Self {
        let mut neighbor_counts = HashMap::new();
        for cell in &self.alive_cells {
            for neighbor in self.neighbors(cell) {
                *neighbor_counts.entry(neighbor).or_insert(0) += 1;
            }
        }

        let capacity_estimate = self.alive_cells.len() * 2;
        let mut next_gen_alive_cells: HashSet<Cell> = HashSet::with_capacity(capacity_estimate);
        let mut new_alive_cells: HashSet<Cell> = HashSet::with_capacity(capacity_estimate / 2);
        let mut new_dead_cells: HashSet<Cell> = HashSet::with_capacity(capacity_estimate / 2);

        for (cell, &alive_neighbors) in neighbor_counts.iter() {
            let current_state = self.is_alive(cell);

            if rules.next_state(current_state, alive_neighbors) {
                next_gen_alive_cells.insert(*cell);
                if !current_state {
                    new_alive_cells.insert(*cell);
                }
            } else if current_state {
                new_dead_cells.insert(*cell);
            }
        }

        Grid { 
            width: self.width, 
            height: self.height, 
            alive_cells: next_gen_alive_cells,
            new_alive_cells: new_alive_cells,
            new_dead_cells: new_dead_cells,
            cached_hash: OnceLock::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_pattern<T: Pattern>(&mut self, pattern: T, x: u32, y: u32) {
        self.alive_cells.extend(pattern.at(x, y));
    }

    fn wrapped_cell(&self, mut nx: i32, mut ny: i32) -> Cell {
        if nx < 0 {
            nx += self.width as i32;
        }
        if ny < 0 {
            ny += self.height as i32;
        }

        let wnx = nx % self.width as i32;
        let wny = ny % self.height as i32;

        Cell::new(wnx as u32, wny as u32)
    }

    pub fn is_alive(&self, cell: &Cell) -> bool {
        self.alive_cells.contains(cell)
    }

}
