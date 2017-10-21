[![Build Status](https://travis-ci.org/amrhassan/rust-graphic.svg?branch=master)](https://travis-ci.org/amrhassan/rust-graphic) [![crates.io](https://img.shields.io/crates/v/graphic.svg)](https://crates.io/crates/graphic)


# rust-graphic #
Graph data structures for Rust

# Usage #
Add it to your project dependencies in `Cargo.toml`:
```toml
[dependencies]
graphic = "0.0.1"
```

# Examples #
```rust
extern crate graphic;
use graphic::*;

fn main() {

        // Create a directed graph
        let mut graph = DirectedGraph::new();
        
        // Create vertices (nodes)
        let zero = graph.add_vertex("zero".to_string());
        let one = graph.add_vertex("one".to_string());
        let two = graph.add_vertex("two".to_string());
        let three = graph.add_vertex("three".to_string());
        let four = graph.add_vertex("four".to_string());
        let five = graph.add_vertex("five".to_string());

        // Form edges by connecting vertices
        graph.connect(zero, one, 5).unwrap();
        graph.connect(zero, two, 3).unwrap();
        graph.connect(one, three, 6).unwrap();
        graph.connect(one, two, 2).unwrap();
        graph.connect(two, four, 4).unwrap();
        graph.connect(two, five, 2).unwrap();
        graph.connect(two, three, 7).unwrap();
        graph.connect(three, five, 1).unwrap();
        graph.connect(three, four, -1).unwrap();
        graph.connect(four, five, -2).unwrap();

        // Print Depth-first order from the first vertex
        println!("Depth-first order from {:?}", zero);
        for vertex in graph.depth_first_iter(zero) {
            println!("  {}", vertex.value);
        }
        println!("");
        
        // Print Breadth-first order from the first vertex
        println!("Breadth-first order from {:?}", zero);
        for vertex in graph.breadth_first_iter(zero) {
            println!("  {}", vertex.value);
        }
        println!("");
        
        // Print values in topological order
        println!("Topological order:");
        for vertex in graph.topologically_ordered_iter() {
            println!("  {}", vertex.value);
        }
        println!("");
        
        // Find maximum path from a vertex
        let longest_distance = graph.longest_distance_from(one).expect("Failed to find maximum distance because graph is cyclic or empty");
        println!("Longest distance from {:?} is {:?}", one, longest_distance);
}
```

# API Doc #
You can find API doc of the latest release [here](https://amrhassan.github.io/rust-graphic/graphic/).
