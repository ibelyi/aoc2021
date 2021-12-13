use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("17"),
        Step::Second => String::from("16"),
    }
}

fn count(dots: &Vec<Vec<usize>>, instr: &[Vec<&str>]) -> usize {
    let mut copy: Vec<(usize, usize)> = dots.iter().map(|d| (d[0], d[1])).collect();
    for line in instr {
        let fold: usize = line[1].parse().unwrap();
        if line[0] == "x" {
            copy = copy
                .iter()
                .map(|d| (if fold > d.0 { d.0 } else { 2 * fold - d.0 }, d.1))
                .collect();
        } else {
            copy = copy
                .iter()
                .map(|d| (d.0, if fold > d.1 { d.1 } else { 2 * fold - d.1 }))
                .collect();
        }
    }
    let max_x = copy.iter().fold(0, |m, d| if m < d.0 { d.0 } else { m }) + 1;
    let max_y = copy.iter().fold(0, |m, d| if m < d.1 { d.1 } else { m }) + 1;
    let mut sheet = vec![vec!['.'; max_x]; max_y];
    for dot in copy {
        sheet[dot.1][dot.0] = '#';
    }
    if instr.len() > 1 {
        for line in &sheet {
            println!("{}", line.into_iter().collect::<String>());
        }
    }
    sheet.iter().flatten().filter(|v| **v == '#').count()
}

pub fn solution(step: &Step, input: &Vec<String>) -> String {
    let mut first = true;
    let mut dots = vec![];
    let mut instr = vec![];
    for line in input {
        if line.len() == 0 {
            first = false;
        } else if first {
            dots.push(line.split(',').map(|v| v.parse().unwrap()).collect());
        } else {
            instr.push(
                line.split("fold along ")
                    .nth(1)
                    .unwrap()
                    .split('=')
                    .collect(),
            );
        }
    }
    match step {
        Step::First => count(&dots, &instr[..1]).to_string(),
        Step::Second => count(&dots, &instr).to_string(),
    }
}
