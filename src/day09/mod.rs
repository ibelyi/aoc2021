use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("15"),
        Step::Second => String::from("1134"),
    }
}

fn count_low(floor: &[Vec<usize>]) -> usize {
    let x_len = floor[0].len();
    let y_len = floor.len();
    let mut count = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            let mut lowest = true;
            if y > 0 {
                lowest = floor[y][x] < floor[y - 1][x];
            }
            if lowest && y < y_len - 1 {
                lowest = floor[y][x] < floor[y + 1][x];
            }
            if lowest && x > 0 {
                lowest = floor[y][x] < floor[y][x - 1];
            }
            if lowest && x < x_len - 1 {
                lowest = floor[y][x] < floor[y][x + 1];
            }
            if lowest {
                count += 1 + floor[y][x];
            }
        }
    }
    count
}

fn count(y: usize, x: usize, copy: &mut Vec<Vec<i32>>) -> usize {
    if copy[y][x] == -1 || copy[y][x] == 9 {
        return 0;
    }
    copy[y][x] = -1;
    let mut sum = 1;
    if y > 0 {
        sum += count(y - 1, x, copy);
    }
    if y < copy.len() - 1 {
        sum += count(y + 1, x, copy);
    }
    if x > 0 {
        sum += count(y, x - 1, copy);
    }
    if x < copy[0].len() - 1 {
        sum += count(y, x + 1, copy);
    }
    sum
}

fn count_basins(floor: &[Vec<usize>]) -> usize {
    let mut copy: Vec<Vec<i32>> = floor
        .iter()
        .map(|l| l.iter().map(|v| *v as i32).collect())
        .collect();
    let mut basins = vec![];
    for y in 0..copy.len() {
        for x in 0..copy[0].len() {
            if copy[y][x] >= 0 {
                basins.push(count(y, x, &mut copy));
            }
        }
    }
    basins.sort_unstable();
    basins[basins.len() - 3..].iter().product()
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let floor: Vec<Vec<usize>> = input
        .iter()
        .map(|l| {
            l.chars()
                .map(|n| String::from(n).parse().expect("Not a number!"))
                .collect()
        })
        .collect();
    match step {
        Step::First => count_low(&floor).to_string(),
        Step::Second => count_basins(&floor).to_string(),
    }
}
