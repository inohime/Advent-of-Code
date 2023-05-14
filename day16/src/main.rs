use regex::Regex;
use std::collections::{HashMap, HashSet};
use utils::*;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: usize,
    nearby_valves: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    fn new(name: &'a str, flow_rate: usize, nearby_valves: Vec<&'a str>) -> Self {
        Self {
            name,
            flow_rate,
            nearby_valves,
        }
    }
}

type SmolTunnels<'a> = HashMap<&'a Valve<'a>, Vec<&'a Valve<'a>>>;
type SmolDistances<'a> = HashMap<&'a Valve<'a>, Vec<(&'a Valve<'a>, usize)>>;
type SmolPaths<'a> = HashSet<&'a Valve<'a>>;

fn setup_distance_map<'a>(
    valves: &'a Vec<Valve>,
    tunnel_valves: &SmolTunnels<'a>,
) -> SmolDistances<'a> {
    let mut distance_map = SmolDistances::new();

    for valve in valves.iter() {
        if valve.name != "AA" && valve.flow_rate <= 0 {
            continue;
        }

        let mut visited = set![];
        visited.insert(valve);

        let mut queue = deque![];
        queue.push_back((valve, 0));

        while let Some((curr, distance)) = queue.pop_front() {
            for &neighbour in tunnel_valves
                .get(curr)
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
            {
                if visited.contains(neighbour) {
                    continue;
                }

                visited.insert(neighbour);

                // good apples!
                if neighbour.flow_rate > 0 {
                    distance_map
                        .entry(valve)
                        .and_modify(|v| v.push((neighbour, distance + 1)))
                        .or_insert(vec![(neighbour, distance + 1)]);
                }

                queue.push_back((neighbour, distance + 1));
            }
        }
    }

    distance_map
}

// depth-first search
fn search<'a>(
    base: &SmolDistances<'a>,
    curr_valve: &'a Valve<'a>,
    curr_path: &mut Vec<&'a Valve<'a>>,
    usable_valves: &mut SmolPaths<'a>,
    time: usize,
    total_release: usize,
) -> (Vec<&'a Valve<'a>>, usize) {
    let mut best_path = curr_path.clone();
    let mut best_release = total_release;

    for &(neighbour, distance) in base[curr_valve].iter() {
        if time >= distance + 1 && usable_valves.contains(neighbour) {
            curr_path.push(neighbour);
            usable_valves.remove(neighbour);

            let time_left = time - distance - 1;
            let curr_release = neighbour.flow_rate * time_left;
            let (new_path, new_release) = search(
                base,
                neighbour,
                curr_path,
                usable_valves,
                time_left,
                total_release + curr_release,
            );

            if new_release > best_release {
                best_path = new_path;
                best_release = new_release;
            }

            curr_path.pop();
            usable_valves.insert(neighbour);
        }
    }

    (best_path, best_release)
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("input16.txt")?;
    let lines: Vec<&str> = file.trim().lines().collect();
    let pattern = Regex::new(
        r"Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? (\w{2}(, \w{2})*)",
    )
    .unwrap();

    let valves: Vec<Valve> = lines
        .iter()
        .flat_map(|&line| {
            pattern
                .captures_iter(line)
                .map(|c| {
                    Valve::new(
                        c.get(1).unwrap().as_str(),
                        c[2].parse::<usize>().unwrap(),
                        c.get(3).unwrap().as_str().split(", ").collect(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let tunnel_valves: SmolTunnels = valves
        .iter()
        .map(|v| {
            (
                v,
                v.nearby_valves
                    .iter()
                    .map(|&name| {
                        let found = valves.iter().position(|v| v.name == name).unwrap();
                        &valves[found]
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let base_map = setup_distance_map(&valves, &tunnel_valves);

    let part_1 = || -> usize {
        let start_valve: Vec<&Valve> = valves.iter().filter(|&x| x.name == "AA").collect();
        let mut usable_valves =
            SmolPaths::from_iter(base_map.iter().map(|(&v, _)| v).filter(|&v| v.name != "AA"));

        let (_, best_release) = search(
            &base_map,
            start_valve[0],
            &mut vec![start_valve[0]],
            &mut usable_valves,
            30,
            0,
        );

        best_release
    };
    println!("{}", part_1());

    let part_2 = || -> usize {
        let start_valve: Vec<&Valve> = valves.iter().filter(|&x| x.name == "AA").collect();
        let mut best_release = 0;

        for _ in 0..(2500) {
            let usable_valves =
                SmolPaths::from_iter(base_map.iter().map(|(&v, _)| v).filter(|&v| v.name != "AA"));

            let mut elephant_usable_valves = usable_valves
                .iter()
                .zip(0..)
                .filter(|(&_, i)| i % 2 == 0)
                .map(|(&v, _)| v)
                .collect();

            let mut my_usable_valves = usable_valves
                .difference(&elephant_usable_valves)
                .cloned()
                .collect();

            let (_, my_best_release) = search(
                &base_map,
                start_valve[0],
                &mut vec![start_valve[0]],
                &mut my_usable_valves,
                26,
                0,
            );

            let (_, elephant_best_release) = search(
                &base_map,
                start_valve[0],
                &mut vec![start_valve[0]],
                &mut elephant_usable_valves,
                26,
                0,
            );

            best_release = best_release.max(my_best_release + elephant_best_release);
        }

        best_release
    };
    println!("{}", part_2());

    Ok(())
}
