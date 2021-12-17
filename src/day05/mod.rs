use super::common::Step;
use std::cmp::Ordering;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("5"),
        Step::Second => String::from("12"),
    }
}

struct Point {
    x: usize,
    y: usize,
}

fn parse(input: &str) -> (Point, Point) {
    let points: Vec<Vec<usize>> = input
        .split(" -> ")
        .map(|p| {
            p.split(',')
                .map(|i| i.parse().expect("Not a number!"))
                .collect()
        })
        .collect();
    (
        Point {
            x: points[0][0],
            y: points[0][1],
        },
        Point {
            x: points[1][0],
            y: points[1][1],
        },
    )
}

fn count_overlaps(vents: &[(Point, Point)], count_diag: bool) -> usize {
    let mut max = Point { x: 0, y: 0 };
    for (p1, p2) in vents {
        for p in [p1, p2] {
            if p.x > max.x {
                max.x = p.x;
            }
            if p.y > max.y {
                max.y = p.y;
            }
        }
    }
    let mut map = vec![vec![0; max.x + 1]; max.y + 1];
    for (p1, p2) in vents {
        let dx = match p1.x.cmp(&p2.x) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };
        let dy = match p1.y.cmp(&p2.y) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };
        if dx != 0 && dy != 0 && !count_diag {
            continue;
        }
        let mut p = Point { x: p1.x, y: p1.y };
        while p.x != p2.x || p.y != p2.y {
            map[p.y][p.x] += 1;
            p.x = (p.x as i32 + dx) as usize;
            p.y = (p.y as i32 + dy) as usize;
        }
        map[p2.y][p2.x] += 1;
    }
    map.iter().flatten().filter(|v| **v > 1).count()
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let vents: Vec<(Point, Point)> = input.iter().map(|n| parse(n)).collect();
    match step {
        Step::First => count_overlaps(&vents, false).to_string(),
        Step::Second => count_overlaps(&vents, true).to_string(),
    }
}
