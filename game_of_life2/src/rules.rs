
pub trait RuleSet {
    fn next_state(&self, cell_is_alive: bool, live_neighbors: u8) -> bool;
}


pub struct GameOfLife;
impl RuleSet for GameOfLife {
    fn next_state(&self, cell_is_alive: bool, live_neighbors: u8) -> bool {
        match (cell_is_alive, live_neighbors) {
            (true, 2) | (true, 3) => true,
            (false, 3) => true,
            _ => false,
        }
    }
}
