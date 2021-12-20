use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("35"),
        Step::Second => String::from("3351"),
    }
}

fn count(algorithm: &[usize], base: &[Vec<usize>], steps: i32) -> usize {
    let mut image = base.to_owned();
    let mut background = 0;
    for _ in 0..steps {
        let mut image2 = vec![vec![0; image[0].len() + 2]; image.len() + 2];
        for y in 0..image2.len() {
            for x in 0..image2[0].len() {
                let mut index = 0;
                for dy in 0..3 {
                    for dx in 0..3 {
                        index = (index << 1)
                            + if y + dy < 2
                                || y + dy >= image.len() + 2
                                || x + dx < 2
                                || x + dx >= image[0].len() + 2
                            {
                                background
                            } else {
                                image[y + dy - 2][x + dx - 2]
                            };
                    }
                }
                image2[y][x] = algorithm[index];
            }
        }
        background = algorithm[background * ((1 << 9) - 1)];
        image = image2;
    }
    image.iter().flatten().sum()
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let algorithm: Vec<usize> = input[0]
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();
    let image: Vec<Vec<usize>> = input
        .iter()
        .skip(2)
        .map(|line| line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();
    match step {
        Step::First => count(&algorithm, &image, 2).to_string(),
        Step::Second => count(&algorithm, &image, 50).to_string(),
    }
}
