use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("5934"),
        Step::Second => String::from("26984457539"),
    }
}

fn count_fish(fish: &Vec<usize>, days: i32) -> u64 {
    let mut curr = vec![0; 9];
    for f in fish {
        curr[*f] += 1;
    }
    for _ in 0..days {
        let spawn = curr.remove(0);
        curr[6] += spawn;
        curr.push(spawn);
    }
    curr.iter().sum()
}

pub fn solution(step: &Step, input: &Vec<String>) -> String {
    let fish = input[0]
        .split(',')
        .map(|n| n.parse().expect("Not a number!"))
        .collect();
    match step {
        Step::First => count_fish(&fish, 80).to_string(),
        Step::Second => count_fish(&fish, 256).to_string(),
    }
}
