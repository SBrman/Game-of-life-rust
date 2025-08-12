mod cell;
mod grid;
mod rules;

use std::thread;
use std::rc::Rc;
use std::time::Duration;
use std::collections::{HashSet, VecDeque};

use crate::grid::Grid;
// use crate::rules::{GameOfLife, HighLife};
use crate::rules::GameOfLife;

fn main() { 

    let mut grid = Grid::new(100, 100);
    let rules = GameOfLife;
    // let rules = HighLife;
    grid.randomize();

    let mut history_set: HashSet<Rc<Grid>> = HashSet::new();
    let mut history_queue: VecDeque<Rc<Grid>> = VecDeque::new();
    const HISTORY_SIZE: usize = 10;

    for i in 0..1000 {
        if grid.all_dead() {
            println!("All cells dead! Simulation Ended.");
            break;
        }

        let current_grid_rc = Rc::new(grid.clone());
        if history_set.contains(&current_grid_rc) {
            println!("Loop detected! Simulation Ended.");
            break;
        }
        history_set.insert(current_grid_rc.clone());
        history_queue.push_back(current_grid_rc.clone());

        if history_queue.len() > HISTORY_SIZE {
            if let Some(old_grid_rc) = history_queue.pop_front() {
                history_set.remove(&old_grid_rc);
            }
        }

        print!("{}[2J", 27 as char);
        println!("Generation {}", i);

        grid.print();
        thread::sleep(Duration::from_millis(50));

        grid = grid.next_generation(&rules);
    }
}

