use evalexpr::*;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: String,
    div: usize,
    targets: [usize; 2],
    inspect_count: usize,
}

fn item_op(op: &String, old: usize) -> usize {
    let mut cntx = HashMapContext::new(); // ineff
    _ = cntx.set_value("old".into(), Value::Int(old as i64));
    eval_int_with_context(&op[..], &cntx).unwrap() as usize
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("input11.txt")?;
    let lines: Vec<&str> = file
        .lines()
        .filter(|&x| !x.is_empty())
        .map(|x| x.trim())
        .collect();

    let mut mz = vec![];

    for (line, _) in lines.chunks(6).map(|x| x).zip(0..) {
        mz.push(Monkey {
            items: line[1]
                .strip_prefix("Starting items: ")
                .map(|x| x.split(", ").map(|x| x.parse::<usize>().unwrap()).collect())
                .unwrap(),
            op: line[2][line[2].find('=').unwrap() + 2..].into(),
            div: line[3][line[3].rfind(' ').unwrap() + 1..]
                .parse::<usize>()
                .unwrap(),
            targets: [
                line[4][line[4].rfind(' ').unwrap() + 1..]
                    .parse::<usize>()
                    .unwrap(),
                line[5][line[5].rfind(' ').unwrap() + 1..]
                    .parse::<usize>()
                    .unwrap(),
            ],
            inspect_count: 0,
        });
    }

    (|| {
        // part 1
        let mut m1 = mz.clone();
        let mut items = vec![vec![]; m1.len()];
        for _ in 0..20 {
            m1.iter_mut().zip(0..).for_each(|(m, i)| {
                m.items.append(&mut items[i]);
                m.inspect_count += m.items.len();
                m.items.drain(..).for_each(|x| {
                    let mut worry_lvl = item_op(&m.op, x);
                    worry_lvl /= 3;
                    let td = worry_lvl % m.div == 0;
                    items[if td { m.targets[0] } else { m.targets[1] }].push(worry_lvl);
                });
            })
        }
        let mut x: Vec<usize> = m1.iter().map(|m| m.inspect_count).collect();
        x.sort_by(|a, b| b.cmp(a));
        println!("{}", x.iter().take(2).product::<usize>());
    })();

    (|| {
        // part 2
        let mut m2 = mz.clone();
        let mut items = vec![vec![]; m2.len()];
        let reducer = m2.iter().fold(1, |acc, m| acc * m.div);
        for _ in 0..10000 {
            m2.iter_mut().zip(0..).for_each(|(m, i)| {
                m.items.append(&mut items[i]);
                m.inspect_count += m.items.len();
                m.items.drain(..).for_each(|x| {
                    let mut worry_lvl = item_op(&m.op, x);
                    worry_lvl %= reducer;
                    let td = worry_lvl % m.div == 0;
                    items[if td { m.targets[0] } else { m.targets[1] }].push(worry_lvl);
                });
            })
        }
        let mut x: Vec<usize> = m2.iter().map(|m| m.inspect_count).collect();
        x.sort_by(|a, b| b.cmp(a));
        println!("{}", x.iter().take(2).product::<usize>());
    })();

    Ok(())
}
