mod common;
use common::{lines_from_file, Step};

mod day07;
use day07::{solution, test_result};
const DAY: &str = "day07";

fn main() {
    let test_input = String::from("./src/") + DAY + "/test_input.txt";
    let input = String::from("./src/") + DAY + "/input.txt";
    if let Ok(test_lines) = lines_from_file(&test_input) {
        if let Ok(lines) = lines_from_file(&input) {
            for s in &[Step::First, Step::Second] {
                let actual = solution(s, &test_lines);
                let expected = test_result(s);
                if actual == expected {
                    println!("{:?}: {}", s, solution(s, &lines));
                } else {
                    eprintln!("{:?}: Test got {}, expected {}", s, actual, expected);
                }
            }
        } else {
            eprintln!("Failed to load lines from {}", input);
        }
    } else {
        eprintln!("Failed to read lines from {}", test_input);
    }
}
