// ! grid: [x, y, w, h]
fn count_edges(grid: [usize; 4]) -> usize {
    let mut counter = 0;
    if grid[1] - 1 == 0 || grid[1] + 1 == grid[2] - 1 {
        if grid[0] - 1 == 0 {
            counter += grid[3];
        }
    }
    if grid[0] - 1 == 0 || grid[0] + 1 == grid[3] - 1 {
        counter += 1;
    }
    counter
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("input8.txt")?;
    let stream: Vec<Vec<u32>> = file
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let arr_w = stream.len();
    let arr_h = stream[0].len();
    // part 1
    println!(
        "sum: {}",
        || -> usize {
            let mut sum = 0;
            for row in 1..arr_w - 1 {
                for col in 1..arr_h - 1 {
                    // edges
                    sum += count_edges([col, row, arr_w, arr_h]);
                    let curr_tree = stream[row][col];
                    // up
                    if (0..row).rev().all(|x| curr_tree > stream[x][col]) {
                        sum += 1;
                        continue;
                    }
                    // down
                    if (row + 1..arr_w).all(|x| curr_tree > stream[x][col]) {
                        sum += 1;
                        continue;
                    }
                    // left
                    if (0..col).rev().all(|x| curr_tree > stream[row][x]) {
                        sum += 1;
                        continue;
                    }
                    // right
                    if (col + 1..arr_h).all(|x| curr_tree > stream[row][x]) {
                        sum += 1;
                        continue;
                    }
                }
            }
            sum
        }()
    );
    // part 2
    println!(
        "scenic score: {}",
        || -> usize {
            let mut scenic_scores = vec![];
            for row in 1..arr_w - 1 {
                for col in 1..arr_h - 1 {
                    let curr_tree = stream[row][col];
                    // up
                    let mut view_same = 0;
                    let mut uv_dist = (0..row)
                        .rev()
                        .take_while(|&x| {
                            if curr_tree == stream[x][col] {
                                view_same += 1;
                                return false;
                            }
                            curr_tree > stream[x][col]
                        })
                        .count();
                    uv_dist += view_same;
                    view_same = 0;
                    // down
                    let mut dv_dist = (row + 1..arr_w)
                        .take_while(|&x| {
                            if curr_tree == stream[x][col] {
                                view_same += 1;
                                return false;
                            }
                            curr_tree > stream[x][col]
                        })
                        .count();
                    dv_dist += view_same;
                    view_same = 0;
                    // left
                    let mut lv_dist = (0..col)
                        .rev()
                        .take_while(|&x| {
                            if curr_tree == stream[row][x] {
                                view_same += 1;
                                return false;
                            }
                            curr_tree > stream[row][x]
                        })
                        .count();
                    lv_dist += view_same;
                    view_same = 0;
                    // right
                    let mut rv_dist = (col + 1..arr_h)
                        .take_while(|&x| {
                            if curr_tree == stream[row][x] {
                                view_same += 1;
                                return false;
                            }
                            curr_tree > stream[row][x]
                        })
                        .count();
                    rv_dist += view_same;

                    scenic_scores.push(uv_dist * dv_dist * lv_dist * rv_dist);
                }
            }
            *scenic_scores.iter().max().unwrap()
        }()
    );

    Ok(())
}
