mod grid;
mod rules;
mod patterns;

use std::collections::HashSet;
use std::num::NonZeroUsize;
use std::rc::Rc;
use lru::LruCache;
use minifb::{Key, Window, WindowOptions};

use crate::grid::{Grid, Cell};
// use crate::patterns::{Blinker, Glider};
use crate::rules::GameOfLife;


const WIDTH: usize = 2000;
const HEIGHT: usize = 2000;

fn main() {
    let mut window = Window::new(
        "Game of Life", 1000, 1000, WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut grid = Grid::new(WIDTH as u32, HEIGHT as u32);
    grid.randomize(0.1);
    let rules = GameOfLife;

    const CACHE_CAPACITY: usize = 5;
    let capacity = NonZeroUsize::new(CACHE_CAPACITY).expect("Must be > 0.");
    let mut history: LruCache<Rc<Grid>, ()> = LruCache::new(capacity);

    let mut grid_rc: Rc<Grid> = Rc::new(grid);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if history.get(&grid_rc).is_some() {
            break;
        }
        history.put(Rc::clone(&grid_rc), ());

        let new_grid = grid_rc.next_generation(&rules);

        update_buffer(&mut buffer, &new_grid.new_alive_cells, &new_grid.new_dead_cells);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        grid = new_grid;
        grid_rc = Rc::new(grid);
    }
}

fn update_buffer(buffer: &mut Vec<u32>, newly_alive_cells: &HashSet<Cell>, dead_cells: &HashSet<Cell>) {
    for (i, cell_set) in [&newly_alive_cells, &dead_cells].iter().enumerate() {
        for cell in cell_set.iter() {
            let index = (cell.y() as usize * WIDTH) + cell.x() as usize;
            if index >= buffer.len() {
                panic!("Index our of bounds!");
            }
            buffer[index] = if i == 0 { 0x00FFFFFF } else { 0x00000000 }
        }
    }
}
