use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("7"),
        Step::Second => String::from("5"),
    }
}

fn count_drops(floor: &Vec<i32>, range: usize) -> u32 {
    let mut count = 0;
    let mut prev = std::i32::MAX;
    for index in 0..floor.len() - range + 1 {
        let curr = floor[index..index + range].iter().sum();
        if curr > prev {
            count += 1;
        }
        prev = curr;
    }
    count
}

pub fn solution(step: &Step, input: &Vec<String>) -> String {
    let floor = input
        .iter()
        .map(|n| n.parse().expect("Not a number"))
        .collect();
    match step {
        Step::First => count_drops(&floor, 1).to_string(),
        Step::Second => count_drops(&floor, 3).to_string(),
    }
}
