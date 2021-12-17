use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("26397"),
        Step::Second => String::from("288957"),
    }
}

fn count(data: &[String], get_corr: bool) -> i64 {
    let mut corr = 0;
    let mut scores = vec![];
    for line in data {
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                _ => {
                    if stack.pop() != Some(c) {
                        corr += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("Unexpected char"),
                        };
                        stack.clear();
                        break;
                    }
                }
            }
        }
        if !stack.is_empty() && !get_corr {
            scores.push(stack.into_iter().rev().fold(0i64, |r, c| {
                r * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("Unexpected char!"),
                    }
            }));
        }
    }
    if get_corr {
        corr
    } else {
        scores.sort_unstable();
        scores[scores.len() >> 1]
    }
}

pub fn solution(step: &Step, input: &[String]) -> String {
    match step {
        Step::First => count(&input, true).to_string(),
        Step::Second => count(&input, false).to_string(),
    }
}
