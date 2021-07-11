
# Hierarchical Pathfinding

A Rust crate to find Paths on a Grid using HPA* (Hierarchical Pathfinding A*) and Hierarchical Dijkstra.

[![Tests](https://github.com/mich101mich/hierarchical_pathfinding/actions/workflows/test.yml/badge.svg)](https://github.com/mich101mich/hierarchical_pathfinding/actions/workflows/test.yml)

## Description
Provides a fast algorithm for finding Paths on a Grid-like structure by caching segments of Paths to form a Node Graph. Finding a Path in that Graph is a lot faster than on the Grid itself, but results in Paths that are _slightly_ worse than the optimal Path.

Implementation based on the Paper ["Near Optimal Hierarchical Path-Finding"](https://www.researchgate.net/profile/Adi-Botea/publication/228785110_Near_optimal_hierarchical_path-finding_HPA/links/09e41508fc2fed9a72000000/Near-optimal-hierarchical-path-finding-HPA.pdf).

### Advantages
- Finding a Path is a lot faster compared to regular algorithms (A*, Dijkstra)
- It is always correct: A Path is found **if and only if** it exists
  - This means that Hierarchical Pathfinding can be used as Heuristic to check if a Path exists and how long it will roughly be (upper bound)

### Disadvantages
- Paths are slightly worse (negligible in most cases)
- Creating the cache takes time (only happens once at the start)
- Changes to the Grid require updating the cache
  - Whenever a Tile within a Chunk changes, that entire Chunk needs to recalculate its Paths. Performance depends on Chunk size (configurable) and the number of Nodes in a Chunk

## Use Case

Finding Paths on a Grid is an expensive Operation. Consider the following Setup:

![The Setup](https://github.com/mich101mich/hierarchical_pathfinding/blob/master/img/problem.png?raw=true)

In order to calculate a Path from Start to End using regular A*, it is necessary to check a
lot of Tiles:

![A*](https://github.com/mich101mich/hierarchical_pathfinding/blob/master/img/a_star.png?raw=true)

(This is simply a small example, longer Paths require a quadratic increase in Tile checks,
and unreachable Goals require the check of _**every single**_ Tile)

The Solution that Hierarchical Pathfinding provides is to divide the Grid into Chunks and
cache the Paths between Chunk entrances as a Graph of Nodes:

![The Graph](https://github.com/mich101mich/hierarchical_pathfinding/blob/master/img/hpa.png?raw=true)

This allows Paths to be generated by connecting the Start and End to the Nodes within the
Chunk and using the Graph for the rest:

![The Solution](https://github.com/mich101mich/hierarchical_pathfinding/blob/master/img/hpa_solution.png?raw=true)

## Example

```rust
use hierarchical_pathfinding::prelude::*;

let mut pathfinding = PathCache::new(
    (width, height),   // the size of the Grid
    |(x, y)| walking_cost(x, y),   // get the cost for walking over a Tile
    ManhattanNeighborhood::new(width, height),   // the Neighborhood
    PathCacheConfig { chunk_size: 3, ..Default::default() },   // config
);

let start = (0, 0);
let goal = (4, 4);

// find_path returns Some(Path) on success
let path = pathfinding.find_path(
    start,
    goal,
    |(x, y)| walking_cost(x, y),
);

if let Some(path) = path {
    println!("Number of steps: {}", path.length());
    println!("Total Cost: {}", path.cost());
    for (x, y) in path {
        println!("Go to {}, {}", x, y);
    }
}
```
