#![allow(unused)]

use std::collections::HashSet;
use utils::*;

fn simulate_sand(sand: (i32, i32), settled: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    [
        (sand.0, sand.1 + 1),     // down
        (sand.0 - 1, sand.1 + 1), // down left
        (sand.0 + 1, sand.1 + 1), // down right
    ]
    .iter()
    .find(|&x| !settled.contains(&x))
    .cloned()
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("input14.txt")?;
    let lines: Vec<&str> = file.trim().lines().collect();

    let mut unknown = 0;
    // the points connect and make a horizontal/vertical line
    let z: Vec<Vec<(i32, i32)>> = lines
        .into_iter()
        .map(|x| {
            x.trim()
                .split(" -> ")
                .map(|y| {
                    let z = y.split(",").collect::<Vec<&str>>();
                    (z[0].parse().unwrap(), z[1].parse().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let t = vec![
        vec![(498, 4), (498, 6), (496, 6)],
        vec![(503, 4), (502, 4), (502, 9), (494, 9)],
    ];

    let mut settled: HashSet<(i32, i32)> = z
        .into_iter()
        .flat_map(|x| {
            x.windows(2)
                .flat_map(|uv| {
                    let ((x1, y1), (x2, y2)) = (uv[0].min(uv[1]), uv[0].max(uv[1]));
                    unknown = unknown.max(y2 + 1);
                    if x1 == x2 {
                        return (y1..=y2).map(|y| (x1, y)).collect::<HashSet<_>>();
                    }
                    (x1..=x2).map(|x| (x, y1)).collect()
                })
                .collect::<HashSet<_>>()
        })
        .collect();

    // falls down until the unknown is reached
    let mut part_1 = || -> u32 {
        // do the sand simulation
        let mut sand_units = 0;
        let mut sp1 = settled.clone();
        let mut src = (500, 0);

        while src.1 <= unknown {
            if let Some(sand) = simulate_sand(src, &sp1) {
                src = sand;
            } else {
                sp1.insert(src);
                sand_units += 1;
                // reset
                src = (500, 0);
            }
        }

        sand_units
    };
    println!("{}", part_1());

    // falls down (going up) and "builds up a 'pile of sand'" until 500, 0 is reached
    let mut part_2 = || -> u32 {
        // do the sand simulation
        let mut sand_units = 0;
        let mut sp2 = settled.clone();
        let mut src = (500, 0);

        while !sp2.contains(&(500, 0)) {
            loop {
                if src.1 >= unknown {
                    break;
                } else if let Some(sand) = simulate_sand(src, &sp2) {
                    src = sand;
                    continue;
                }
                break;
            }
            sp2.insert(src);
            sand_units += 1;
            src = (500, 0);
        }

        sand_units
    };
    println!("{}", part_2());

    Ok(())
}
