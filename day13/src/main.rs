use std::cmp::*;
mod noms {
    pub use nom::branch::*;
    pub use nom::bytes::complete::*;
    pub use nom::character::complete::*;
    pub use nom::combinator::*;
    pub use nom::multi::*;
    pub use nom::sequence::*;
    pub use nom::IResult;
}

// how many pairs of packets are in the right order
// sum up the indices in the right order: 1, 2, 4, 6 are in the right order so = 13
// there are only 2 lines to compare against every iteration
#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::Int(l), Packet::List(r)) => vec![Packet::Int(*l)].cmp(r),
            (Packet::List(l), Packet::Int(r)) => l.cmp(&vec![Packet::Int(*r)]),
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
        }
    }
}

fn parse(in_val: &str) -> noms::IResult<&str, Packet> {
    noms::alt((
        noms::map(noms::i32, |x| Packet::Int(x)),
        noms::map(
            noms::delimited(
                noms::tag("["),
                noms::separated_list0(noms::tag(","), parse),
                noms::tag("]"),
            ),
            |x| Packet::List(x),
        ),
    ))(in_val)
}

fn parse_val(input: &str) -> Packet {
    parse(input).unwrap().1
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("input13.txt")?;
    let lines: Vec<&str> = file.lines().filter(|x| !x.is_empty()).collect();
    let mut sum = 0;
    let mut mp = vec![];

    let part_1 = || -> i32 {
        let mut p = vec![];
        let mut u = 0;

        for (i, line) in (0..).zip(lines) {
            p.push(line);
            u += 1;
            if u >= 2 {
                u = 0;
                let v1 = parse_val(p[0]);
                let v2 = parse_val(p[1]);
                if v1 < v2 {
                    sum += 1 + i;
                }
                p.clear();
            }
            mp.push(parse_val(line));
        }
        sum / 2
    };
    println!("{}", part_1());

    let part_2 = || -> i32 {
        // [[2]], [[6]]
        let mut key = 1;
        let div_packet1 = Packet::List(vec![Packet::Int(2)]);
        let div_packet2 = Packet::List(vec![Packet::Int(6)]);

        mp.push(div_packet1.clone());
        mp.push(div_packet2.clone());
        mp.sort();

        // multiply the divider packets + index
        for (i, p) in (0..).zip(mp) {
            if p == div_packet1 || p == div_packet2 {
                key *= i + 1;
            }
        }
        key
    };
    println!("{}", part_2());

    Ok(())
}
