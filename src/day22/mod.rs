use super::common::Step;
use std::collections::HashSet;
use std::io::{self, Write};

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("474140"),
        Step::Second => String::from("2758514936282235"),
    }
}

fn count_boot(data: &[(u32, Vec<Vec<i32>>)]) -> u32 {
    let mut engine: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; 101]; 101]; 101];
    for prod in data {
        if (0..3).any(|i| (0..2).any(|j| prod.1[i][j].abs() > 50)) {
            continue;
        }
        for x in prod.1[0][0]..=prod.1[0][1] {
            for y in prod.1[1][0]..=prod.1[1][1] {
                for z in prod.1[2][0]..=prod.1[2][1] {
                    engine[(z + 50) as usize][(y + 50) as usize][(x + 50) as usize] = prod.0;
                }
            }
        }
    }
    engine.iter().flatten().flatten().sum()
}

fn count(data: &[(u32, Vec<Vec<i32>>)]) -> u64 {
    let mut heap = vec![HashSet::new(); data[0].1.len()];
    for cube in data {
        for (i, range) in cube.1.iter().enumerate() {
            for edge in range {
                heap[i].insert(*edge);
                heap[i].insert(*edge + 1);
            }
        }
    }
    let mut edges: Vec<Vec<i32>> = heap.into_iter().map(|s| s.into_iter().collect()).collect();
    print!("Sorting edges...");
    io::stdout().flush().unwrap();
    for edge in edges.iter_mut() {
        edge.sort_unstable();
    }
    let mut engine: Vec<Vec<Vec<u64>>> =
        vec![vec![vec![0; edges[0].len() - 1]; edges[1].len() - 1]; edges[2].len() - 1];
    let mut prev = 0;
    for (n, cube) in data.iter().enumerate() {
        let edge: Vec<Vec<usize>> = (0..3)
            .map(|i| {
                (0..2)
                    .map(|j| {
                        edges[i]
                            .binary_search(&cube.1[i][j])
                            .expect("Unexpected boundary")
                    })
                    .collect()
            })
            .collect();
        let curr = n * 100 / data.len();
        if curr > prev {
            print!("\rFilling engine with cubes: {}%  ", curr);
            io::stdout().flush().unwrap();
            prev = curr;
        }
        for row in engine.iter_mut().take(edge[2][1] + 1).skip(edge[2][0]) {
            for col in row.iter_mut().take(edge[1][1] + 1).skip(edge[1][0]) {
                for dot in col.iter_mut().take(edge[0][1] + 1).skip(edge[0][0]) {
                    *dot = cube.0 as u64;
                }
            }
        }
    }

    let mut result = 0;
    prev = 0;
    for (z, row) in engine.iter().enumerate() {
        let curr = z * 100 / engine.len();
        if curr > prev {
            print!("\rCounting engine: {}%               ", curr);
            io::stdout().flush().unwrap();
            prev = curr;
        }
        let dz = (edges[2][z + 1] - edges[2][z]) as u64;
        for (y, col) in row.iter().enumerate() {
            let dydz = dz * ((edges[1][y + 1] - edges[1][y]) as u64);
            for (x, dot) in col.iter().enumerate() {
                result += *dot * dydz * ((edges[0][x + 1] - edges[0][x]) as u64);
            }
        }
    }
    println!("\rDone!                              ");
    result
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<(u32, Vec<Vec<i32>>)> = input
        .iter()
        .map(|line| {
            let v: Vec<&str> = line.split(' ').collect();
            (
                if v[0] == "on" { 1 } else { 0 },
                v[1].split(',')
                    .map(|c| {
                        c.split('=')
                            .nth(1)
                            .unwrap()
                            .split("..")
                            .map(|n| n.parse().unwrap())
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect();
    match step {
        Step::First => count_boot(&data).to_string(),
        Step::Second => count(&data).to_string(),
    }
}
