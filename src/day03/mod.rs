use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("198"),
        Step::Second => String::from("230"),
    }
}

fn vote(report: &Vec<u32>, mask: u32) -> i32 {
    report.iter().fold(0, |v, r| if r & mask > 0 { v + 1 } else { v - 1})
}

fn power_consumption(report: &Vec<u32>, size: usize) -> i32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut mask = 1 << (size - 1);
    while mask > 0 {
        gamma <<= 1;
        epsilon <<= 1;
        if vote(report, mask) > 0 {
            gamma += 1
        } else {
            epsilon += 1
        }
        mask >>= 1;
    }
    gamma * epsilon
}

fn life_support(report: &Vec<u32>, size: usize) -> u32 {
    let mut oxy = report.clone();
    let mut co2 = report.clone();
    let mut mask = 1 << (size - 1);
    while mask > 0 {
        if oxy.len() > 1 {
            let crit = if vote(&oxy, mask) >= 0 { mask } else { 0 };
            oxy.retain(|n| n & mask == crit);
        }
        if co2.len() > 1 {
            let crit = if vote(&co2, mask) < 0 { mask } else { 0 };
            co2.retain(|n| n & mask == crit);
        }
        mask >>= 1;
    }
    oxy[0] * co2[0]
}

pub fn solution(step: &Step, input: &Vec<String>) -> String {
    let size = input[0].len();
    let report = input
        .iter()
        .map(|n| u32::from_str_radix(n, 2).expect("Not a binary number"))
        .collect();
    match step {
        Step::First => power_consumption(&report, size).to_string(),
        Step::Second => life_support(&report, size).to_string(),
    }
}
