# Advent of Code 2023 Solutions

**Language**: Rust

Every day has a dedicated file in the `solutions` directory, and two inputs (one the actual input, another the example suffixed with 'e') in the `inputs` directory.

Different people have different ways of approaching Advent of Code. Mine is not competitive. Instead, I'm just trying to explore rust while having some fun. Many of these solutions can be briefer, even in Rust, but I'm trying to keep them as readable as possible.

## Usage of External Crates
I try to solve these problem within the bound of the standard library. However, the standard library is limited compared to many other languages. Things like regex, mundane mathematical operations, parallelism are absent. Therefore, I'm using a few external libraries.

Currently I'm using:
- [regex](https://crates.io/crates/regex) for regex
- [rayon](https://crates.io/crates/rayon) for parallelism
- [num](https://crates.io/crates/num) for mathematical operations
- [derive_deref](https://crates.io/crates/derive_deref) because implementing deref is tediously trivial
- [indexmap](https://lib.rs/crates/indexmap) because doing it by hand is tedious

## Credits
Shamelessly stole and adapted [the scaffolding mechanism found here](https://github.com/fspoettel/advent-of-code-rust) to my need.

## Runtime

| Day/Part | Part 1 | Part 2 | Total |
|:---------|-------:|-------:|------:|
| **Day 01** | 39.95μs | 561.03μs | 600.98μs |
| **Day 02** | 36.78μs | 36.61μs | 73.39μs |
| **Day 03** | 544.57μs | 320.43μs | 864.99μs |
| **Day 04** | 128.17μs | 129.22μs | 257.39μs |
| **Day 05** | 24.58μs | 34.65μs | 59.24μs |
| **Day 06** | 7.06μs | 7.41μs | 14.47μs |
| **Day 07** | 278.94μs | 282.18μs | 561.12μs |
| **Day 08** | 756.95μs | 1.36ms | 2.12ms |
| **Day 09** | 181.98μs | 181.21μs | 363.19μs |
| **Day 10** | 5.24ms | 9.70ms | 14.94ms |
| **Day 11** | 1.26ms | 1.26ms | 2.52ms |
| **Day 12** | 12.20ms | 588.90ms | 601.10ms |
| **Day 13** | 482.00μs | 488.49μs | 970.49μs |
| **Day 14** | 45.87μs | 27.08ms | 27.12ms |
| **Day 15** | 49.12μs | 393.33μs | 442.45μs |
| **Day 16** | 1.90ms | 180.90ms | 182.80ms |
| **Day 17** | 96.01ms | 245.32ms | 341.34ms |
| **Day 18** | 47.37μs | 47.10μs | 94.47μs |
| **Day 19** | 324.49μs | 3.75ms | 4.08ms |


**Total runtime: 1.18s**

