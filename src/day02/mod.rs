use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("150"),
        Step::Second => String::from("900"),
    }
}

fn parse(dir: &str) -> (&str, i32) {
    let dir: Vec<&str> = dir.split(' ').collect();
    (dir[0], dir[1].parse().expect("Not a number!"))
}

fn simple_rules(dirs: &Vec<(&str, i32)>) -> i32 {
    let mut pos = (0, 0);
    for dir in dirs {
        match dir.0 {
            "forward" => pos.0 += dir.1,
            "down" => pos.1 += dir.1,
            "up" => pos.1 -= dir.1,
            _ => panic!("Unknown direction!"),
        }
    }
    pos.0 * pos.1
}

fn advanced_rules(dirs: &Vec<(&str, i32)>) -> i32 {
    let mut pos = (0, 0, 0);
    for dir in dirs {
        match dir.0 {
            "forward" => {
                pos.0 += dir.1;
                pos.1 += pos.2 * dir.1;
            }
            "down" => pos.2 += dir.1,
            "up" => pos.2 -= dir.1,
            _ => panic!("Unknown direction!"),
        }
    }
    pos.0 * pos.1
}

pub fn solution(step: &Step, input: &Vec<String>) -> String {
    let dirs = input.iter().map(|n| parse(n)).collect();
    match step {
        Step::First => simple_rules(&dirs).to_string(),
        Step::Second => advanced_rules(&dirs).to_string(),
    }
}
