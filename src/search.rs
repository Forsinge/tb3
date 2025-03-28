use crate::dict::*;
use crate::grid::{Grid, IDEAL_POINTS, WORD_BONUSES};
use std::collections::HashMap;

pub const WORD_STACK_DEPTH: usize = 16;
pub const WORD_STACK_DIM: usize = 2048;

pub fn find_words(
    grid: &Grid,
    dict: &[Node],
    word_stack: &mut [(u64, usize); WORD_STACK_DEPTH * WORD_STACK_DIM],
    word_stack_start: usize,
    winning_lengths: [bool; 13],
    idx_stack: &mut [usize; 128],
    seq_stack: &mut [u64; 128],
    dict_stack: &mut [usize; 128],
) -> usize {
    let mut stack_idx;
    let mut word_stack_idx = word_stack_start;
    let mut remaining = grid.remaining;

    while remaining != 0 {
        let index = remaining.trailing_zeros() as usize;
        let bit = 1 << index;
        remaining ^= bit;

        dict_stack[0] = dict[0].children[grid.letters[index]];
        seq_stack[0] = bit;
        idx_stack[0] = index;
        stack_idx = 1;

        while stack_idx != 0 {
            stack_idx -= 1;

            let grid_seq = seq_stack[stack_idx];
            let grid_idx = idx_stack[stack_idx];
            let dict_idx = dict_stack[stack_idx];
            let node = &dict[dict_idx];

            if node.connections & 1 == 1 && winning_lengths[node.len] {
                word_stack[word_stack_idx] = (grid_seq, dict_idx);
                word_stack_idx += 1;
            };

            if node.connections & grid.adjacency_ords[grid_idx] == 0 {
                continue;
            }

            let mut adjacency = grid.adjacency[grid_idx] & !grid_seq;

            while adjacency != 0 {
                let adj_idx = adjacency.trailing_zeros() as usize;
                let adj_bit = 1 << adj_idx;
                let ord = grid.letters[adj_idx];

                dict_stack[stack_idx] = node.children[ord];
                idx_stack[stack_idx] = adj_idx;
                seq_stack[stack_idx] = grid_seq | adj_bit;

                stack_idx += (node.connections >> ord) & 1;
                adjacency ^= adj_bit;
            }
        }
    }

    word_stack_idx
}

pub fn solve_puzzle(
    grid: &mut Grid,
    dict: &Vec<Node>,
    points: u8,
    max_points: &mut u8,
    plays_idx: usize,
    plays: &mut [usize; 32],
    node_count: &mut u64,
    word_stack: &mut [(u64, usize); WORD_STACK_DEPTH * WORD_STACK_DIM],
    cache: &mut HashMap<u64, u8>,
    dict_stack: &mut [usize; 128],
    idx_stack: &mut [usize; 128],
    seq_stack: &mut [u64; 128],
) {
    *node_count += 1;

    if grid.remaining == 0 {
        *max_points = points;
        println!("New best solution: {max_points}p");
        for i in 0..plays_idx {
            println!("{}", reconstruct_word(dict, plays[i]));
        }
        println!();
        return;
    }

    if let Some(upper_bound) = cache.get(&grid.remaining) {
        if points + upper_bound <= *max_points {
            return;
        }
    }

    let mut len = 1;
    let mut winning_lengths = [true; 13];

    let count = grid.remaining.count_ones() as usize;
    while len <= count && len <= 12 {
        let remaining = count - len;
        let word_points = WORD_BONUSES[len];
        let winning_length = points + word_points + IDEAL_POINTS[remaining] > *max_points;
        winning_lengths[len] = winning_length;
        len += 1;
    }

    grid.recompute();
    let start = plays_idx * WORD_STACK_DIM;
    let end = find_words(
        grid,
        dict,
        word_stack,
        start,
        winning_lengths,
        idx_stack,
        seq_stack,
        dict_stack,
    );

    if start == end {
        cache.insert(grid.remaining, 0);
        return;
    }

    let slice = &mut word_stack[start..end];
    slice.sort_unstable_by(|(a, _), (b, _)| b.count_ones().cmp(&a.count_ones()).then(b.cmp(a)));
    word_stack[end] = (0, 0);

    let mut word_stack_idx = start;
    while word_stack_idx < end {
        let (word, idx) = word_stack[word_stack_idx];
        word_stack_idx += 1;

        if word == word_stack[word_stack_idx].0 {
            continue;
        }

        let word_points = WORD_BONUSES[word.count_ones() as usize];
        grid.flip(word);
        plays[plays_idx] = idx;
        solve_puzzle(
            grid,
            dict,
            points + word_points,
            max_points,
            plays_idx + 1,
            plays,
            node_count,
            word_stack,
            cache,
            dict_stack,
            idx_stack,
            seq_stack,
        );
        grid.flip(word);
    }

    cache.insert(grid.remaining, *max_points - points);
}
