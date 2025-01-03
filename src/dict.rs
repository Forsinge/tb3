use std::fs::File;
use std::io;
use std::io::BufRead;

const AE: u64 = 27;
const AA: u64 = 28;
const OE: u64 = 29;
const ORD_OFFSET: u64 = 64;

#[derive(Default)]
pub struct Node {
    pub children: [usize; 31],
    pub connections: usize,
}

pub fn get_dict_tree(grid_input: &str, dictionary_path: &str) -> Result<Vec<Node>, io::Error> {
    let filtered = grid_input.replace('\n', "");
    let file = File::open(dictionary_path)?;
    let mut words = Vec::new();

    for line in io::BufReader::new(file).lines().flatten() {
        let len = line.chars().count();
        let mut include = true;
        let mut letters = filtered.clone();
        for c in line.chars() {
            match letters.find(c) {
                None => include = false,
                Some(i) => {
                    let _ = letters.remove(i);
                }
            }
        }

        if include && len <= 12 {
            words.push(line.clone());
        }
    }

    let mut tree = vec![Node::default()];

    for word in words {
        let mut curr_idx = 0;
        for ch in word.chars() {
            let ord = to_bits(ch);
            let bit = 1 << ord;
            let len = tree.len();

            if tree.get(curr_idx).unwrap().connections & bit == 0 {
                tree.get_mut(curr_idx).unwrap().connections |= bit;
                tree.get_mut(curr_idx).unwrap().children[ord as usize] = len;
                tree.push(Node::default());
            }

            curr_idx = tree.get(curr_idx).unwrap().children[ord as usize];
        }
        tree[curr_idx].connections |= 1;
    }

    Ok(tree)
}

pub fn reconstruct_word(dict: &Vec<Node>, idx: usize) -> String {
    let mut s = String::new();
    let mut i = 0;
    while i < idx {
        let node = &dict[i];
        for j in (0..node.children.len()).rev() {
            if node.children[j] <= idx && node.children[j] != 0 {
                s.push(from_bits(j as u64));
                i = node.children[j];
                break;
            }
        }
    }
    s
}

pub const fn to_bits(l: char) -> u64 {
    match l {
        'Ä' => AE,
        'Å' => AA,
        'Ö' => OE,
        _ => l as u64 - ORD_OFFSET,
    }
}

pub const fn from_bits(l: u64) -> char {
    match l {
        AE => 'Ä',
        AA => 'Å',
        OE => 'Ö',
        _ => (l + ORD_OFFSET) as u8 as char,
    }
}
