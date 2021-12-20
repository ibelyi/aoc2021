use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("4140"),
        Step::Second => String::from("3993"),
    }
}

fn add(numb1: &mut Vec<String>, numb2: &[String]) {
    numb1.insert(0, "[".to_string());
    numb1.push(",".to_string());
    numb1.extend_from_slice(numb2);
    numb1.push("]".to_string());
}

fn reduce(numb: &mut Vec<String>) {
    loop {
        let mut done = true;
        let mut depth = 0;
        let mut i = 0;
        while i < numb.len() - 2 {
            match numb[i].chars().next().unwrap() {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => (),
                _ => {
                    if depth > 4 && numb[i + 2].chars().next().unwrap().is_digit(10) {
                        done = false;
                        for j in (1..i - 1).rev() {
                            if numb[j].chars().next().unwrap().is_digit(10) {
                                numb[j] = (numb[j].parse::<i32>().unwrap()
                                    + numb[i].parse::<i32>().unwrap())
                                .to_string();
                                break;
                            }
                        }
                        for j in i + 3..numb.len() {
                            if numb[j].chars().next().unwrap().is_digit(10) {
                                numb[j] = (numb[j].parse::<i32>().unwrap()
                                    + numb[i + 2].parse::<i32>().unwrap())
                                .to_string();
                                break;
                            }
                        }
                        numb.splice(i - 1..=i + 3, ["0".to_string()]);
                        depth -= 1;
                        continue;
                    }
                }
            };
            i += 1;
        }

        for i in 0..numb.len() {
            if numb[i].chars().next().unwrap().is_digit(10) {
                let val = numb[i].parse::<i32>().unwrap();
                if val > 9 {
                    done = false;
                    let l = val / 2;
                    let r = val - l;
                    numb.splice(
                        i..=i,
                        [
                            "[".to_string(),
                            l.to_string(),
                            ",".to_string(),
                            r.to_string(),
                            "]".to_string(),
                        ],
                    );
                    break;
                }
            }
        }

        if done {
            break;
        }
    }
}

fn magn(numb: &mut Vec<String>) -> i32 {
    while numb.len() > 1 {
        let mut reduced = false;
        for i in 1..numb.len() - 3 {
            if numb[i].chars().next().unwrap().is_digit(10)
                && numb[i + 1] == ","
                && numb[i + 2].chars().next().unwrap().is_digit(10)
            {
                let val = (numb[i].parse::<i32>().unwrap() * 3
                    + numb[i + 2].parse::<i32>().unwrap() * 2)
                    .to_string();
                numb.splice(i - 1..=i + 3, [val]);
                reduced = true;
                break;
            }
        }
        if !reduced {
            panic!("unreducible!");
        }
    }
    numb[0].parse().unwrap()
}

fn count(data: &[Vec<String>]) -> i32 {
    let mut result = data[0].to_owned();
    for numb in data.iter().skip(1) {
        add(&mut result, &numb);
        reduce(&mut result);
    }
    magn(&mut result)
}

fn count_max(data: &[Vec<String>]) -> i32 {
    let mut result = 0;
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i == j {
                continue;
            }
            let mut numb1 = data[i].to_owned();
            let numb2 = &data[j];
            add(&mut numb1, &numb2);
            reduce(&mut numb1);
            let curr = magn(&mut numb1);
            if curr > result {
                result = curr;
            }
        }
    }
    result
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<Vec<String>> = input
        .iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();
    match step {
        Step::First => count(&data).to_string(),
        Step::Second => count_max(&data).to_string(),
    }
}
