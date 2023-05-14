use itertools::Itertools;
use regex::Regex;

/// Manhattan Distance
///
/// Equation:
/// |x1 - x2| + |y1 - y2|
///
/// Description:
/// sum of the absolute values or the difference in both coords
///
/// * `p1` tuple of x1, y1
/// * `p2` tuple of x2, y2
fn mdst(p1: (i64, i64), p2: (i64, i64)) -> Option<u64> {
    Some(p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1))
}

fn parse_num(v: &str) -> i64 {
    v.parse::<i64>().unwrap()
}

fn find_ranges(target: i64, list: &mut Vec<[i64; 4]>) -> Vec<(i64, i64)> {
    let x: Vec<(i64, i64)> = list
        .iter()
        .filter_map(|pos| {
            let sensor = (pos[0], pos[1]);
            let beacon = (pos[2], pos[3]);
            mdst(sensor, beacon)
                .filter(|&dist| target.abs_diff(sensor.1) <= dist)
                .map(|dist| {
                    let sy_dist = target.abs_diff(sensor.1);
                    let k = (dist - sy_dist) as i64;
                    (sensor.0 - k, sensor.0 + k)
                })
        })
        .collect();

    x.into_iter()
        .sorted_by_key(|&(s, e)| (s, e))
        .coalesce(|(prev_s, prev_e), (curr_s, curr_e)| {
            if prev_e >= curr_s {
                Ok((prev_s, prev_e.max(curr_e)))
            } else {
                Err(((prev_s, prev_e), (curr_s, curr_e)))
            }
        })
        .collect()
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("input15.txt")?;
    let lines: Vec<&str> = file.lines().collect();

    let mut list: Vec<[i64; 4]> = vec![];
    let pattern = Regex::new(
        r"Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)",
    )
    .unwrap();

    for line in lines {
        for c in pattern.captures_iter(line) {
            list.push([
                parse_num(&c[1]),
                parse_num(&c[2]),
                parse_num(&c[3]),
                parse_num(&c[4]),
            ]);
        }
    }

    let mut part_1 = || -> i64 {
        find_ranges(2000000, &mut list)
            .iter()
            .map(|x| x.1 - x.0)
            .sum()
    };
    println!("{}", part_1());

    let mut part_2 = || -> i64 {
        const TARGET: i64 = 4000000;
        let mut freq = 0;

        for i in 0..=TARGET {
            let ranges = find_ranges(i, &mut list);
            if ranges[0].0 <= 0 && ranges[0].1 > TARGET {
                continue;
            }

            for beacon in ranges.iter() {
                if beacon.0 <= 0 && beacon.1 >= 0 {
                    freq = (beacon.1 + 1) * TARGET + i as i64;
                }
            }
        }

        freq
    };
    println!("{}", part_2());

    Ok(())
}
