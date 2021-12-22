use super::common::Step;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryInto;
use std::io::{self, Write};

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("79"),
        Step::Second => String::from("3621"),
    }
}

/// Rotate beacons detected by a scanner given rotation id
///
/// # Example
/// ```
/// let scan = vec![vec![649, 640, 665],vec![682, -795, 504]];
/// let answer1 = aoc2021::day19::rotate(&scan, 2);
/// let answer2 = aoc2021::day19::rotate(&scan, 6);
///
/// assert_eq!(&answer1[0], &[-649, 640, -665]);
/// assert_eq!(&answer1[1], &[-682,-795, -504]);
/// assert_eq!(&answer2[0], &[-649, 665,  640]);
/// assert_eq!(&answer2[1], &[-682, 504, -795]);
///
/// ```
pub fn rotate(scan: &[Vec<i32>], r: i32) -> Vec<Vec<i32>> {
    if r == 0 {
        return scan.to_owned();
    }
    let xi: usize = (r / 8).try_into().unwrap();
    let yi: usize = (((r / 4 + 3) / 2) % 3).try_into().unwrap();
    let zi: usize = (2 - (r / 4) % 3).try_into().unwrap();
    let (xs, ys) = (1 - (r & 2), 1 - ((r << 1) & 2));
    let zs = xs * ys * (1 - ((r >> 1) & 2));
    scan.iter()
        .map(|p| vec![xs * p[xi], ys * p[yi], zs * p[zi]])
        .collect()
}

/*
fn rotate_in_place(scan: &mut Vec<Vec<i32>>, r: i32) {
    if r == 0 {
        return;
    }
    let xi = if r % 8 == 0 { 2 } else { 0 };
    let yi = if (r + 4) % 8 == 0 { 2 } else { 1 };
    let zi = if r % 4 == 0 {
        if r % 8 == 0 {
            0
        } else {
            1
        }
    } else {
        2
    };
    let xs = ((r << 1) & 2) - 1;
    let ys = -1;
    let zs = if (r + 2) % 4 == 0 { 1 } else { -1 };

    for p in scan.iter_mut() {
        let (x, y, z) = (xs * p[xi], ys * p[yi], zs * p[zi]);
        p[0] = x;
        p[1] = y;
        p[2] = z;
    }
}
*/

fn equals(scan1: &[Vec<i32>], scan2: &[Vec<i32>]) -> bool {
    let mut count = 0;
    let (mut i1, mut i2) = (0, 0);
    while i1 < scan1.len() && i2 < scan2.len() {
        match (0..3).fold(Ordering::Equal, |v, i| {
            if v == Ordering::Equal {
                scan1[i1][i].cmp(&scan2[i2][i])
            } else {
                v
            }
        }) {
            Ordering::Greater => i2 += 1,
            Ordering::Less => i1 += 1,
            Ordering::Equal => {
                i1 += 1;
                i2 += 1;
                count += 1;
                if count > 11 {
                    return true;
                }
            }
        }
    }
    false
}

fn match_with_rotate(scan1: &[Vec<i32>], scan: &[Vec<i32>]) -> Option<(Vec<Vec<i32>>, Vec<i32>)> {
    //let mut scan2 = scan.to_owned();
    for r in 0..24 {
        //rotate_in_place(&mut scan2, r);
        let mut scan2 = rotate(&scan, r);
        scan2.sort();
        for beac1 in scan1 {
            for i2 in 0..scan2.len() {
                let shift: Vec<i32> = (0..3).map(|i| beac1[i] - scan2[i2][i]).collect();
                scan2
                    .iter_mut()
                    .for_each(|b| (0..3).for_each(|i| b[i] += shift[i]));
                if equals(&scan1, &scan2) {
                    return Some((scan2, shift));
                }
                scan2
                    .iter_mut()
                    .for_each(|b| (0..3).for_each(|i| b[i] -= shift[i]));
            }
        }
    }
    None
}

type Result = (Vec<Vec<Vec<i32>>>, Vec<Option<Vec<i32>>>);
fn count(data: &[Vec<Vec<i32>>]) -> Result {
    //println!("Counting... {} to go", data.len());
    let mut first = data[0].to_owned();
    first.sort();
    let mut done = Vec::with_capacity(data.len());
    done.push(first);
    let mut locations = vec![None; data.len()];
    locations[0] = Some(vec![0; 3]);
    for i in 0..data.len() {
        for j in 0..data.len() {
            if locations[j].is_some() {
                continue;
            }
            if let Some((scan, shift)) = match_with_rotate(&done[i], &data[j]) {
                print!("{}/{}\r", done.len(), data.len());
                io::stdout().flush().unwrap();
                locations[j] = Some(shift);
                done.push(scan);
            }
        }
    }
    println!("Done!   ");
    (done, locations)
}

fn count_beacons(data: &[Vec<Vec<i32>>]) -> usize {
    let (done, _) = count(&data);
    done.iter()
        .fold(HashSet::new(), |mut bs, s| {
            s.iter().for_each(|b| {
                bs.insert(b);
            });
            bs
        })
        .len()
}

fn count_distance(data: &[Vec<Vec<i32>>]) -> i32 {
    let (_, locations) = count(&data);
    let mut distance = 0;
    for i1 in 0..locations.len() - 1 {
        if let Some(p1) = &locations[i1] {
            for p2 in locations.iter().skip(i1).flatten() {
                let curr = (0..3).map(|i| (p1[i] - p2[i]).abs()).sum();
                if curr > distance {
                    distance = curr;
                }
            }
        }
    }
    distance
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<Vec<Vec<i32>>> = input.iter().fold(Vec::new(), |mut scans, line| {
        if line.contains(" scanner ") {
            scans.push(Vec::new());
        } else if !line.is_empty() {
            scans
                .last_mut()
                .unwrap()
                .push(line.split(',').map(|n| n.parse().unwrap()).collect());
        };
        scans
    });
    match step {
        Step::First => count_beacons(&data).to_string(),
        Step::Second => count_distance(&data).to_string(),
    }
}
