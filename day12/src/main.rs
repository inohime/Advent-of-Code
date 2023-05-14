use utils::*;
use std::cmp::Ordering::*;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct SmolNode {
    height: u8,
    ch: char,
    pos: [isize; 2],
}

impl SmolNode {
    fn new(height: u8, ch: char, pos: [isize; 2]) -> Self {
        Self { height, ch, pos }
    }
}

type SmolTree = HashMap<SmolNode, Vec<SmolNode>>;

// breadth-first search
fn search(tree: &SmolTree, start: SmolNode, end: SmolNode) -> Option<usize> {
    let mut queue = deque![];
    let mut visited = set![];

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((curr, steps)) = queue.pop_front() {
        if curr == end {
            return Some(steps);
        }

        let neighbours = &tree[&curr];
        for next in neighbours {
            if visited.insert(*next) {
                queue.push_back((*next, steps + 1));
            }
        }
    }

    None
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("input12.txt")?;
    let lines: Vec<&str> = file.lines().collect();

    let mut start @ mut end = None;

    let grid: Vec<SmolNode> = (0..lines.len())
        .flat_map(|i| (0..lines[0].len()).map(move |j| (i, j)))
        .map(|(r, c)| {
            let cell = lines[r].chars().nth(c).unwrap();
            match cell {
                'S' => {
                    start = Some(SmolNode::new(b'a' - b'a', 'a', [r as isize, c as isize]));
                    start.unwrap()
                }
                'E' => {
                    end = Some(SmolNode::new(b'z' - b'a', 'z', [r as isize, c as isize]));
                    end.unwrap()
                }
                _ => SmolNode::new(cell as u8 - b'a', cell, [r as isize, c as isize]),
            }
        })
        .collect();

    let tree: SmolTree = grid
        .iter()
        .map(|&x| {
            // up/ down/ left/ right
            let d = vec![(0, -1), (0, 1), (-1, 0), (1, 0)]
                .iter()
                .filter_map(|(i, j)| {
                    let next = [x.pos[0] + i, x.pos[1] + j];
                    if let Some(np) = grid.iter().position(|z| z.pos == next) {
                        let v = grid[np].height;
                        match v.cmp(&(x.height + 1)) {
                            Less => Some(grid[np]),
                            Equal => Some(grid[np]),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect();
            (x, d)
        })
        .collect();

    // part 1
    println!("{:?}", search(&tree, start.unwrap(), end.unwrap()));

    // part 2
    // find the 'a' that is closest to the goal
    let nearest: Vec<_> = tree.iter().filter(|x| x.0.pos == [30, 0]).collect();
    println!("{:?}", search(&tree, *nearest[0].0, end.unwrap()));

    Ok(())
}
