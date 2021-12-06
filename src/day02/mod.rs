use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("150"),
        Step::Second => String::from("900"),
    }
}

enum Dir {
    Forward,
    Down,
    Up,
}

fn parse(dir: &str) -> (Dir, i32) {
    let dir: Vec<&str> = dir.split(' ').collect();
    let count: i32 = dir[1].parse().expect("Not a number!");
    if dir[0] == "forward" {
        return (Dir::Forward, count);
    } else if dir[0] == "down" {
        return (Dir::Down, count);
    } else if dir[0] == "up" {
        return (Dir::Up, count);
    }
    panic!("Unknown direction!");
}

fn simple_rules(dirs: &Vec<(Dir, i32)>) -> i32 {
    let mut pos = (0, 0);
    for dir in dirs {
        match dir.0 {
            Dir::Forward => pos.0 += dir.1,
            Dir::Down => pos.1 += dir.1,
            Dir::Up => pos.1 -= dir.1,
        }
    }
    pos.0 * pos.1
}

fn advanced_rules(dirs: &Vec<(Dir, i32)>) -> i32 {
    let mut pos = (0, 0, 0);
    for dir in dirs {
        match dir.0 {
            Dir::Forward => {
                pos.0 += dir.1;
                pos.1 += pos.2 * dir.1;
            }
            Dir::Down => pos.2 += dir.1,
            Dir::Up => pos.2 -= dir.1,
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
