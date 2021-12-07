use super::common::Step;
use std::convert::TryFrom;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("37"),
        Step::Second => String::from("168"),
    }
}

fn linear_cost(crabs: &Vec<i32>) -> i32 {
    let mut crabs = crabs.clone();
    crabs.sort();
    let p = crabs[((crabs.len() + 1) / 2)];
    let mut cost = 0;
    for c in crabs {
        cost += (c - p).abs();
    }
    cost
}

fn arithmetic_cost(crabs: &Vec<i32>) -> i32 {
    let mut prev = i32::MAX;
    for p in 0..crabs.len() {
        let mut cost = 0;
        for c in crabs {
            let dist = (c - i32::try_from(p).unwrap()).abs();
            cost += dist * (dist + 1) / 2;
        }
        if cost > prev {
            return prev;
        }
        prev = cost;
    }
    prev
}

pub fn solution(step: &Step, input: &Vec<String>) -> String {
    let crabs = input[0]
        .split(',')
        .map(|n| n.parse().expect("Not a number!"))
        .collect();
    match step {
        Step::First => linear_cost(&crabs).to_string(),
        Step::Second => arithmetic_cost(&crabs).to_string(),
    }
}
