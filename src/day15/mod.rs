use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("40"),
        Step::Second => String::from("315"),
    }
}

fn n_risk(y: usize, x: usize, risk: &[Vec<usize>]) -> usize {
    let mut min = usize::MAX;
    if x > 0 && risk[y][x - 1] < min {
        min = risk[y][x - 1];
    }
    if x < risk[0].len() - 1 && risk[y][x + 1] < min {
        min = risk[y][x + 1];
    }
    if y > 0 && risk[y - 1][x] < min {
        min = risk[y - 1][x];
    }
    if y < risk.len() - 1 && risk[y + 1][x] < min {
        min = risk[y + 1][x];
    }
    min
}

fn count(data: &[Vec<usize>]) -> usize {
    let y_len = data.len();
    let x_len = data[0].len();
    // Fill up risk assessment for each point with maximum possible value
    let mut risk: Vec<Vec<usize>> = vec![vec![10 * x_len * y_len; x_len]; y_len];
    // Repeat while risk assessment could be reduced at least in one point
    loop {
        let mut reduced = false;
        for y in 0..y_len {
            for x in 0..x_len {
                // smallest risk to arrive from a neighbour
                let c_risk = if y + x == 0 {
                    0
                } else {
                    n_risk(y, x, &risk) + data[y][x]
                };
                if c_risk < risk[y][x] {
                    risk[y][x] = c_risk;
                    reduced = true;
                }
            }
        }
        if !reduced {
            break risk[y_len - 1][x_len - 1];
        }
    }
}

fn count_fives(data: &[Vec<usize>]) -> usize {
    let y_len = data.len();
    let x_len = data[0].len();
    let map: Vec<Vec<usize>> = (0..y_len * 5)
        .map(|y| {
            (0..x_len * 5)
                .map(|x| (data[y % y_len][x % x_len] + y / y_len + x / x_len - 1) % 9 + 1)
                .collect()
        })
        .collect();
    count(&map)
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<Vec<usize>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    match step {
        Step::First => count(&data).to_string(),
        Step::Second => count_fives(&data).to_string(),
    }
}
