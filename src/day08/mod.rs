use super::common::Step;
use std::collections::{HashMap, HashSet};

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("26"),
        Step::Second => String::from("61229"),
    }
}

#[derive(Debug)]
struct Signal {
    sig: String,
    set: HashSet<char>,
}

fn parse(signal: &str) -> Signal {
    let mut chars: Vec<char> = signal.chars().collect();
    chars.sort_unstable();
    let set = signal.chars().collect();
    let sig = chars.iter().collect();
    Signal { sig, set }
}

fn count_1478(signals: &[Vec<Vec<Signal>>]) -> usize {
    signals
        .iter()
        .map(|line| line[1].iter())
        .flatten()
        .filter(|signal| {
            let len = signal.sig.len();
            len < 5 || len == 7
        })
        .count()
}

fn count_all(signals: &[Vec<Vec<Signal>>]) -> i32 {
    let mut count = 0;
    for line in signals {
        let mut map = HashMap::<i32, &Signal>::new();
        for signal in &line[0] {
            let len = signal.sig.len();
            if len == 2 {
                map.insert(1, &signal);
            } else if len == 3 {
                map.insert(7, &signal);
            } else if len == 4 {
                map.insert(4, &signal);
            } else if len == 7 {
                map.insert(8, &signal);
            }
        }
        for signal in &line[0] {
            let len = signal.sig.len();
            let one_set = &map.get(&1).unwrap().set;
            if len == 5 && one_set.difference(&signal.set).count() == 0 {
                map.insert(3, &signal);
            } else if len == 6 && one_set.difference(&signal.set).count() > 0 {
                map.insert(6, &signal);
            }
        }
        for signal in &line[0] {
            if signal.sig.len() == 6 && map.get(&6).unwrap().set != signal.set {
                let mut union_set = map.get(&3).unwrap().set.clone();
                for c in &map.get(&4).unwrap().set {
                    union_set.insert(*c);
                }
                if signal.set.difference(&union_set).count() == 0 {
                    map.insert(9, &signal);
                } else {
                    map.insert(0, &signal);
                }
            }
        }
        for signal in &line[0] {
            if signal.sig.len() == 5 && map.get(&3).unwrap().set != signal.set {
                let nine_set = &map.get(&9).unwrap().set;
                if signal.set.difference(nine_set).count() > 0 {
                    map.insert(2, &signal);
                } else {
                    map.insert(5, &signal);
                }
            }
        }
        let mut numbers = HashMap::<String, i32>::new();
        for (numb, &signal) in &map {
            numbers.insert(signal.sig.clone(), *numb);
        }
        count += line[1]
            .iter()
            .fold(0, |sum, v| sum * 10 + numbers.get(&v.sig).unwrap());
    }
    count
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let signals: Vec<Vec<Vec<Signal>>> = input
        .iter()
        .map(|n| {
            n.split(" | ")
                .map(|s| s.split(' ').map(|v| parse(v)).collect())
                .collect()
        })
        .collect();
    match step {
        Step::First => count_1478(&signals).to_string(),
        Step::Second => count_all(&signals).to_string(),
    }
}
