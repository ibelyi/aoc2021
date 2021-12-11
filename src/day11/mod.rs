use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("1656"),
        Step::Second => String::from("195"),
    }
}

fn inc_flash(y: usize, x: usize, energy: &mut Vec<Vec<usize>>) {
    if y < energy.len() && x < energy[0].len() && energy[y][x] > 0 {
        energy[y][x] += 1;
        flash(y, x, energy);
    }
}

fn flash(y: usize, x: usize, energy: &mut Vec<Vec<usize>>) {
    if energy[y][x] < 10 {
        return;
    }
    energy[y][x] = 0;
    inc_flash(y + 1, x + 1, energy);
    inc_flash(y + 1, x, energy);
    inc_flash(y, x + 1, energy);
    if y > 0 {
        inc_flash(y - 1, x, energy);
        inc_flash(y - 1, x + 1, energy);
    }
    if x > 0 {
        inc_flash(y, x - 1, energy);
        inc_flash(y + 1, x - 1, energy);
        if y > 0 {
            inc_flash(y - 1, x - 1, energy);
        }
    }
}

fn one_step(energy: &mut Vec<Vec<usize>>) {
    for line in energy.into_iter() {
        for oct in line {
            *oct += 1;
        }
    }
    for y in 0..energy.len() {
        for x in 0..energy[0].len() {
            flash(y, x, energy);
        }
    }
}

fn count_flashes(data: &Vec<Vec<usize>>) -> usize {
    let mut energy = data.clone();
    let mut total = 0;
    for _ in 0..100 {
        one_step(&mut energy);
        total += energy.iter().flatten().filter(|v| **v == 0).count()
    }
    total
}

fn count_first(data: &Vec<Vec<usize>>) -> i32 {
    let mut energy = data.clone();
    let total = energy[0].len() * energy.len();
    for step in 0..i32::MAX {
        one_step(&mut energy);
        if energy.iter().flatten().filter(|v| **v == 0).count() == total {
            return step + 1;
        }
    }
    0
}

pub fn solution(step: &Step, input: &Vec<String>) -> String {
    let data = input
        .iter()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();
    match step {
        Step::First => count_flashes(&data).to_string(),
        Step::Second => count_first(&data).to_string(),
    }
}
