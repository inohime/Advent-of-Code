use std::cmp::Ordering::*;

macro_rules! set {
    ($ ($key: expr), *) => {{
        let mut set = std::collections::HashSet::new();
        $(set.insert($key);)*
        set
    }}
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("input9.txt")?;
    // part 1
    println!(
        "{:?}",
        || -> Option<i32> {
            let mut head @ mut tail = [0, 0];
            let mut visited = set![tail];
            for line in file.lines() {
                let mut s = line.split(' ');
                let (dir, steps): (&str, i32) = (s.next()?, s.next()?.parse().unwrap());
                const N: i32 = 1;
                for _ in 0..steps {
                    // move head
                    match dir {
                        "U" => head[1] += N,
                        "D" => head[1] -= N,
                        "L" => head[0] -= N,
                        "R" => head[0] += N,
                        _ => {}
                    }

                    if head[0].abs_diff(tail[0]) > 1 || head[1].abs_diff(tail[1]) > 1 {
                        for i in 0..=1 {
                            match head[i].cmp(&tail[i]) {
                                Greater => tail[i] += 1,
                                Less => tail[i] -= 1,
                                Equal => {}
                            }
                        }
                    }
                    visited.insert(tail);
                }
            }
            Some(visited.len() as i32)
        }()
        .unwrap()
    );
    // part 2
    println!(
        "{:?}",
        || -> Option<i32> {
            let mut knots = [[0, 0]; 10];
            let mut visited = set![knots[9]];
            for line in file.lines() {
                let mut s = line.split(' ');
                let (dir, steps): (&str, i32) = (s.next()?, s.next()?.parse().unwrap());
                const N: i32 = 1;
                for _ in 0..steps {
                    // move head
                    match dir {
                        "U" => knots[0][1] += N,
                        "D" => knots[0][1] -= N,
                        "L" => knots[0][0] -= N,
                        "R" => knots[0][0] += N,
                        _ => {}
                    }

                    for knot in 1..knots.len() {
                        let head = knots[knot - 1];
                        let tail = knots[knot];
                        if head[0].abs_diff(tail[0]) > 1 || head[1].abs_diff(tail[1]) > 1 {
                            for i in 0..=1 {
                                match head[i].cmp(&tail[i]) {
                                    Greater => knots[knot][i] += 1,
                                    Less => knots[knot][i] -= 1,
                                    Equal => {}
                                }
                            }
                        }
                        visited.insert(knots[9]);
                    }
                }
            }
            Some(visited.len() as i32)
        }()
        .unwrap()
    );

    Ok(())
}
