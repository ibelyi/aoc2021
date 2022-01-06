use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("58"),
        Step::Second => String::from("No problem"),
    }
}

struct Field {
    ylen: usize,
    xlen: usize,
}

impl Field {
    fn new(data: &[Vec<usize>]) -> Field {
        Field {
            ylen: data.len(),
            xlen: data[0].len(),
        }
    }
    fn next(&self, dir: usize, y: usize, x: usize) -> (usize, usize) {
        if dir == 1 {
            (y, (x + 1) % self.xlen)
        } else {
            ((y + 1) % self.ylen, x)
        }
    }

    fn iter(&self) -> FieldIter {
        FieldIter {
            field: &self,
            y: 0,
            x: 0,
        }
    }
}

struct FieldIter<'a> {
    field: &'a Field,
    y: usize,
    x: usize,
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.field.ylen {
            return None;
        }
        let result = (self.y, self.x);
        if self.x < self.field.xlen - 1 {
            self.x += 1;
        } else {
            self.x = 0;
            self.y += 1;
        }
        Some(result)
    }
}

fn count(data: &[Vec<usize>]) -> i32 {
    let mut count = 0;
    let mut cucumbers = data.to_owned();
    let field = Field::new(&data);
    loop {
        let mut moved = false;
        for dir in [1, 2] {
            let mut newc = cucumbers.to_owned();
            for (y, x) in field.iter() {
                let (y1, x1) = field.next(dir, y, x);
                if cucumbers[y][x] == dir && cucumbers[y1][x1] == 0 {
                    moved = true;
                    newc[y][x] = 0;
                    newc[y1][x1] = dir;
                }
            }
            cucumbers = newc;
        }
        count += 1;
        if !moved {
            return count;
        }
    }
}

const DISP: [char; 3] = ['.', '>', 'v'];

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<Vec<usize>> = input
        .iter()
        .map(|l| {
            l.chars()
                .filter_map(|c| DISP.iter().position(|&r| r == c))
                .collect()
        })
        .collect();
    match step {
        Step::First => count(&data).to_string(),
        Step::Second => "No problem".to_string(),
    }
}
