// signal strength = cycle number * the value of the X register
/* instructions:
    addx: 2 cycles to complete
    noop: 1 cycles to complete

    example input: 15 - 11 + 6 - 3 + 5 - 1 -8 + 13 + 4 + noop == 20 + (X = 1)
    this is 19 cycles + the first cycle of addx - 1 + X (1)
*/
fn is_ok_add(arr: &mut Vec<i32>, cycle: i32, val: i32) {
    if cycle % 40 == 20 {
        arr.push(val * cycle);
    }
}

fn run_tracer(cycle: i32, val: i32, output: &mut String) {
    if ((cycle - 1) % 40 as i32).abs_diff(val) <= 1 { 
        output.push('#');
    } else {
        output.push('.');
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("input10.txt")?;
    // part 1
    println!(
        "sum: {}",
        || -> i32 {
            let mut x = 1;
            let mut cycle = 1;
            let signals: Vec<i32> = file.lines().flat_map(|s| {
                let mut z = vec![];
                cycle += 1;
                match s.split_once(' ') {
                    Some(("addx", v)) => {
                        is_ok_add(&mut z, cycle, x);
                        cycle += 1;
                        x += v.parse::<i32>().unwrap();
                        is_ok_add(&mut z, cycle, x);
                    }
                    _ => {}
                };
                z
            })
            .collect();

            signals.iter().sum()
        }()
    );
    
    // part 2
    println!(
        "{}",
        || -> String {
            let mut crt = String::with_capacity(6 * 40); // col, row
            let mut x = 1;
            let mut cycle = 1;
            let _: Vec<()> = file.lines().map(|s| {
                run_tracer(cycle, x, &mut crt);
                cycle += 1;
                match s.split_once(' ') {
                    Some(("addx", v)) => {
                        run_tracer(cycle, x, &mut crt);
                        cycle += 1;
                        x += v.parse::<i32>().unwrap();
                    }
                    _ => {}
                };
            })
            .collect();
            
            crt.chars()
                .collect::<Vec<char>>()
                .chunks(40)
                .map(|x| x.iter().collect())
                .collect::<Vec<String>>()
                .join("\n")
        }()
    );

    Ok(())
}