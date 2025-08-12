# Game of Life – Rust Implementations

> Three high-performance Conway’s Game of Life implementations in Rust, exploring data structures, rendering techniques, and CPU tensor computation.

## Overview

This repository contains three different versions of Conway’s Game of Life, each optimized in a different way:

1. **Terminal Version** – Simple vector-based grid; prints directly to the console.  
2. **Minifb + HashSet** – Uses a `HashSet` for efficient alive-cell checks and early termination based on state history; handles grids up to ~2000×2000 smoothly.  
3. **Minifb + `tch` (libtorch CPU)** – Tensor-based computation using PyTorch’s C++ backend; supports massive grids (e.g., 5000×5000) with minimal lag.  

## Highlights
- Explore Rust performance trade-offs.
- Learn about `minifb` for real-time visualization.
- Compare vector, hash-based, and tensor-based implementations.

## Requirements
- Rust (latest stable)
- For the `tch` version: [libtorch](https://pytorch.org/get-started/locally/)

