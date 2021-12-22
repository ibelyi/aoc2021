use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("739785"),
        Step::Second => String::from("444356092776315"),
    }
}

fn count_100(starts: &[usize]) -> u32 {
    let mut positions: Vec<u32> = starts.iter().map(|v| (*v - 1) as u32).collect();
    let mut counts = vec![0; starts.len()];
    let (mut rolls, mut die, mut index) = (0, 0, 0);
    loop {
        let mut step = 0;
        for _ in 0..3 {
            step += die + 1;
            die = (die + 1) % 100;
        }
        rolls += 3;
        positions[index] = (positions[index] + step) % 10;
        counts[index] += positions[index] + 1;
        if counts[index] > 999 {
            break;
        }
        index = (index + 1) % counts.len();
    }
    rolls * counts[(index + 1) % counts.len()]
}

const DIES: [(usize, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
const LOC: usize = 10;
const WIN: usize = 21;

fn count_quantum(start: &[usize]) -> u64 {
    let mut counts = [0u64, 0u64];
    let mut universes = vec![vec![vec![vec![vec![0; LOC]; LOC]; WIN]; WIN]; 2];
    universes[0][0][0][start[0] - 1][start[1] - 1] = 1;
    loop {
        let mut done = true;
        for i in 0..2 {
            for s1 in 0..WIN {
                for s2 in 0..WIN {
                    for p1 in 0..LOC {
                        for p2 in 0..LOC {
                            let curr = universes[i][s1][s2][p1][p2];
                            if curr == 0 {
                                continue;
                            }
                            done = false;
                            universes[i][s1][s2][p1][p2] = 0;
                            for (c, u) in DIES {
                                let val = u * curr;
                                let mut v: [usize; 4] = [s1, s2, p1, p2];
                                v[i + 2] = (v[i + 2] + c) % 10;
                                v[i] += v[i + 2] + 1;
                                if v[i] >= WIN {
                                    counts[i] += val;
                                    continue;
                                }
                                universes[(i + 1) & 1][v[0]][v[1]][v[2]][v[3]] += val;
                            }
                        }
                    }
                }
            }
        }
        if done {
            break;
        }
    }
    if counts[0] > counts[1] {
        counts[0]
    } else {
        counts[1]
    }
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<usize> = input
        .iter()
        .map(|line| line.split("position: ").nth(1).unwrap().parse().unwrap())
        .collect();
    match step {
        Step::First => count_100(&data).to_string(),
        Step::Second => count_quantum(&data).to_string(),
    }
}
