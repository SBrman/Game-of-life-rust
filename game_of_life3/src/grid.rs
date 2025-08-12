use tch::{Tensor, Kind, Device};

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Tensor,
    kernel: Tensor,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {

        let device = Device::Cpu;
        let shape = [height as i64, width as i64];
        let cells = Tensor::zeros(&shape, (Kind::Int64, device));
        let kernel = Tensor::from_slice(&[
            1.0f32, 1.0, 1.0, 
            1.0, 0.0, 1.0, 
            1.0, 1.0, 1.0, 
        ])
            .view([1, 1, 3, 3])
            .to_device(device);

        Self { width, height, cells, kernel }
    }

    pub fn randomize(&mut self, density: f64) {
        let random_tensor = Tensor::rand(
            &[self.height as i64, self.width as i64], 
            (Kind::Float, Device::Cpu)
        );
        self.cells = random_tensor.gt(density).to_kind(Kind::Int64);
    }

    pub fn alive_neighbors(&self) -> Tensor {
        self.cells
        .to_kind(Kind::Float)
        .unsqueeze(0)
        .unsqueeze(0)
        .conv2d(&self.kernel, None::<&Tensor>, 1, 1, 1, 1)
        .squeeze()
        .to_kind(Kind::Int64)
    }

    pub fn next_generation(&mut self) {
        let alive_neighbors = self.alive_neighbors();
        let alive_mask = self.cells.to_kind(Kind::Int64);

        let two = alive_neighbors.eq(2);
        let three = alive_neighbors.eq(3);

        let survive = alive_mask.logical_and(&two.logical_or(&three));
        let born = alive_mask.logical_not().logical_and(&three);

        let new_cells = survive.logical_or(&born).to_kind(Kind::Int64);
        self.cells = new_cells;
    }
}
