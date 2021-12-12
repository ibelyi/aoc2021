use super::common::Step;
use std::collections::{HashMap, HashSet};

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("226"),
        Step::Second => String::from("3509"),
    }
}

fn visit<'a>(
    end: &'a str,
    visited: &mut Vec<&'a str>,
    cave_map: &HashMap<&str, Vec<&'a str>>,
    twice: bool,
) -> i32 {
    let mut allow_again = !visited.contains(&end);
    if !allow_again && (!twice || end == "start") {
        return 0;
    }
    if end == "end" {
        return 1;
    }
    let lowcase = end.to_lowercase() == end;
    if lowcase {
        allow_again = twice && allow_again;
        visited.push(end);
    } else {
        allow_again = twice;
    }
    let mut sum = 0;
    for go in &cave_map[end] {
        sum += visit(go, visited, cave_map, allow_again);
    }
    if lowcase {
        visited.pop();
    }
    sum
}

fn count(caves: &Vec<Vec<&str>>, twice: bool) -> i32 {
    let mut cave_map = HashMap::<&str, Vec<&str>>::new();
    for path in caves {
        cave_map.entry(path[0]).or_insert(vec![]).push(path[1]);
        cave_map.entry(path[1]).or_insert(vec![]).push(path[0]);
    }
    visit("start", &mut vec![], &cave_map, twice)
}

pub fn solution(step: &Step, input: &Vec<String>) -> String {
    let data = input.iter().map(|l| l.split('-').collect()).collect();
    match step {
        Step::First => count(&data, false).to_string(),
        Step::Second => count(&data, true).to_string(),
    }
}
