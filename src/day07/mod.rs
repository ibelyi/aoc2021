use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("37"),
        Step::Second => String::from("168"),
    }
}

fn fuel_cost<F>(crabs: &Vec<i32>, cost: F) -> i32
where
    F: Fn(i32) -> i32,
{
    let mut min = i32::MAX;
    for p in 0..crabs.len() {
        let curr = crabs.iter().map(|v| cost((v - p as i32).abs())).sum();
        if curr < min {
            min = curr;
        }
    }
    min
}

pub fn solution(step: &Step, input: &Vec<String>) -> String {
    let crabs = input[0]
        .split(',')
        .map(|n| n.parse().expect("Not a number!"))
        .collect();
    match step {
        Step::First => fuel_cost(&crabs, |d| d).to_string(),
        Step::Second => fuel_cost(&crabs, |d| d * (d + 1) / 2).to_string(),
    }
}
