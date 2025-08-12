mod grid;

use tch::{Tensor, Kind, Device};
use minifb::{Key, Window, WindowOptions, Scale};
use crate::grid::Grid;

fn main() {
    let gw = 5000;
    let gh = 5000;

    let width = 1000;
    let height = 1000;

    let mut grid = Grid::new(gw, gh);
    grid.randomize(0.3);
    let mut window = Window::new(
        "GOL", width, height, WindowOptions::default()
    ).unwrap();
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let buffer = grid_to_buffer(&grid);
        window.update_with_buffer(&buffer, width, height).unwrap();
        grid.next_generation();
    }
}


fn grid_to_buffer(grid: &Grid) -> Vec<u32> {
    let flat = grid.cells
        .to_device(Device::Cpu)
        .contiguous()
        .view([-1])
        .to_kind(tch::Kind::Int64);
    let ints: Vec<i64> = Vec::try_from(flat).expect("Must be Int64");

    ints
    .into_iter()
    .map(|alive| if alive != 0 {0x00FFFFFF} else {0x00000000})
    .collect()
}
