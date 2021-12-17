use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("45"),
        Step::Second => String::from("112"),
    }
}

fn hit(x: i32, y: i32, area: &[Vec<i32>]) -> Option<i32> {
    let (high_x, low_x) = (area[0][1], area[0][0]);
    let (high_y, low_y) = (area[1][1], area[1][0]);
    let (mut x_c, mut y_c) = (0, 0);
    let (mut x_v, mut y_v) = (x, y);
    let mut hight = 0;
    while x_c <= high_x && y_c >= low_y {
        if x_v == 0 && x_c < low_x {
            break;
        }
        x_c += x_v;
        y_c += y_v;
        if hight < y_c {
            hight = y_c;
        }
        if x_c >= low_x && x_c <= high_x && y_c >= low_y && y_c <= high_y {
            return Some(hight);
        }
        if x_v > 0 {
            x_v -= 1;
        }
        y_v -= 1;
    }
    None
}

fn count_hight(area: &[Vec<i32>]) -> i32 {
    let high_x = area[0][1];
    let low_y = area[1][0];
    let mut hight = 0;
    for x in 1..high_x + 1 {
        for y in low_y - 1..-low_y * 2 + 1 {
            if let Some(h) = hit(x, y, area) {
                if hight < h {
                    hight = h;
                }
            }
        }
    }
    hight
}

fn count_velocities(area: &[Vec<i32>]) -> i32 {
    let high_x = area[0][1];
    let low_y = area[1][0];
    let mut count = 0;
    for x in 1..high_x + 1 {
        for y in low_y - 1..-low_y * 2 + 1 {
            if hit(x, y, area).is_some() {
                count += 1;
            }
        }
    }
    count
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<Vec<i32>> = input[0]
        .split("target area: x=")
        .nth(1)
        .unwrap()
        .split(", y=")
        .map(|r| {
            let mut range: Vec<i32> = r
                .split("..")
                .map(|n| n.parse().expect("Not a number!"))
                .collect();
            range.sort_unstable();
            range
        })
        .collect();
    match step {
        Step::First => count_hight(&data).to_string(),
        Step::Second => count_velocities(&data).to_string(),
    }
}
