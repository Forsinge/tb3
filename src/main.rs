mod dict;
mod grid;
mod search;

use std::collections::HashMap;
use std::env;
use crate::dict::get_dict_tree;
use crate::search::{search_grid, WORD_STACK_DEPTH, WORD_STACK_DIM};
use grid::*;
use std::time::Instant;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please specify a path to a dictionary");
        return;
    }

    println!("Enter grid. Empty line to finish:");

    let mut grid_rows = Vec::new();
    let mut row_buf = String::new();

    loop {
        if let Err(e) = stdin().read_line(&mut row_buf) {
            eprintln!("Unable to read grid: {e}");
            return;
        }

        if row_buf.trim().is_empty() {
            break;
        }

        grid_rows.push(row_buf.trim().to_string());
        row_buf.clear();
    }

    let grid_input = grid_rows.join("\n");
    let dict_tree;

    match get_dict_tree(&grid_input, &args[1]) {
        Ok(tree) => dict_tree = tree,
        Err(e) => {
            eprintln!("Could not create dictionary: {e}");
            return;
        }
    }

    let mut grid = Grid::new(&grid_input);
    let points = 0;
    let plays_idx = 0;
    let mut max_points = 10;
    let mut plays = [0; 32];
    let mut node_count = 0;
    let mut word_stack = vec![(0, 0); WORD_STACK_DEPTH * WORD_STACK_DIM];
    let mut cache = HashMap::with_capacity(65536);
    let clock = Instant::now();

    search_grid(
        &mut grid,
        &dict_tree,
        points,
        &mut max_points,
        plays_idx,
        &mut plays,
        &mut node_count,
        &mut word_stack,
        &mut cache,
    );

    println!();
    println!("Search complete.");
    println!("Cached nodes: {}", cache.len());
    println!("Nodes visited: {node_count}");
    println!("Finished in {:?} ms", clock.elapsed().as_millis());
}
