use super::common::Step;
use std::collections::HashMap;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("1588"),
        Step::Second => String::from("2188189693529"),
    }
}

fn count(polimer: &str, rules: &[Vec<&str>], steps: usize) -> i64 {
    // Create a HashMap entry out of a rule.
    // Example: CH -> B becomes {"CH": (["CB", "BH"], 'B')}
    let d_rules: HashMap<&str, (Vec<String>, char)> = rules
        .iter()
        .map(|v| {
            let t = v[1].chars().next().unwrap();
            (
                v[0],
                (
                    v[0].chars()
                        .enumerate()
                        .map(|(i, c)| {
                            if i == 0 {
                                [c, t].iter().collect()
                            } else {
                                [t, c].iter().collect()
                            }
                        })
                        .collect(),
                    t,
                ),
            )
        })
        .collect();
    // Keep count of pairs in the polimer
    let mut pair_counts = HashMap::<String, i64>::new();
    // Keep count of elements in the polimer
    let mut single_counts = HashMap::new();
    let mut prev = polimer.chars().next().unwrap();
    // Make original counts from the polimer
    single_counts.insert(prev, 1);
    for c in polimer.chars().skip(1) {
        *single_counts.entry(c).or_insert(0) += 1;
        *pair_counts.entry([prev, c].iter().collect()).or_insert(0) += 1;
        prev = c;
    }
    for _ in 0..steps {
        // Pair count updates made just in this step
        let mut new_counts = HashMap::<String, i64>::new();
        for (key, value) in &pair_counts {
            if let Some(rule) = d_rules.get(&key as &str) {
                // Increase counts for the newly created pairs
                for pair in &rule.0 {
                    *new_counts.entry(pair.to_string()).or_insert(0) += value;
                }
                // Increase count for the newly inserted element
                *single_counts.entry(rule.1).or_insert(0) += value;
                // Decrease count for the currently split pair
                *new_counts.entry(key.to_string()).or_insert(0) -= value;
            }
        }
        // Update total pair counts using counts of this step
        for (key, value) in new_counts {
            *pair_counts.entry(key.to_string()).or_insert(0) += value;
        }
    }
    // Collect and sort single counts to get the result
    let mut result: Vec<&i64> = single_counts.values().collect();
    result.sort();
    *result.last().unwrap() - *result[0]
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let polimer = &input[0];
    let rules: Vec<Vec<&str>> = input[2..]
        .iter()
        .map(|v| v.split(" -> ").collect())
        .collect();
    match step {
        Step::First => count(&polimer, &rules, 10).to_string(),
        Step::Second => count(&polimer, &rules, 40).to_string(),
    }
}
