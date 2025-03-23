# IT3708 - Project 2: Memetic Genetic Algorithm

A Memetic Genetic Algorithm implementation in Rust for solving nurse routing problems. 
Achieved competitive results on benchmark datasets (e.g., 859.3904 on train_9), and second place in the course competition.
Key features include heuristic operations, parallelization, and adaptive population management.

## Table of Contents
- [Installation Prerequisites](#installation-prerequisites)
- [Usage](#usage)
- [Algorithm Overview](#algorithm-overview)
- [Configuration](#configuration)
- [Key Components](#key-components)
- [Results](#results)
- [License](#license)

## Installation Prerequisites


## Usage
```zsh
cargo run --release
```

## Algorithm Overview
### Population Initialization
- **Feasible Instantiation**: Constructs semi-feasible solutions through randomized patient insertion
- **File Instantiation**: Loads pre-saved high-quality individuals

### Parent Selection
- **Linear Ranking**: Primary method using rank-based probabilities ($s=1.5$ typically)
- **Alternatives Implemented**: Fitness-proportionate and tournament selection

### Crossover (Visma Variant)
1. Parent nurse selection and patient removal
2. Repair via optimal insertion testing
3. Fitness-guided position selection

### Mutation Operators
- **Primary**: Heuristic cluster and swap mutations
- **Secondary**: Random swap, insert, and destroy-repair mutations

### Child Selection
- Uses $(λ+μ)$ strategy with crowding-based similarity metrics

### Restart Mechanisms
- **Hard Restart**: Full population reinitialization
- Soft Restart (implemented but unused)

### Elitism
- Preserves top 1 individual between generations

### Parallelization
- **Islands Model**: Recommended for Intel/Mac systems
- **Multithreading**: Rayon-based loop parallelization

## Configuration
Hyperparameters controlled via `config/config.yaml`:
- Population size
- Mutation rates
- Selection parameters
- Parallelization mode

## Key Observations
1. Maintains mixed feasible/infeasible population
2. Mutation-free runs achieved 5% margins in early tests
3. Crowding critical for diversity maintenance
4. Penalty function weights:
    - Time window violations: 9×travel time
    - Demand violations: +1000
    - Return time violations: +3000

## Results
| Dataset  | Best Score | Benchmark Margin |
|----------|------------|-------------------|
| test_0   | 828.0649   | <5%               |
| test_1   | 1576.1025  | <5%               | 
| test_2   | 930.9652   | <5%               |

Solution visualizations available in report figures.

## License
*[Specify license here]*
