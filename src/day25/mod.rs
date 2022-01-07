use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("58"),
        Step::Second => String::from("No problem"),
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    East,
    South,
}

impl Cell {
    fn parse(cell: char) -> Cell {
        match cell {
            '.' => Cell::Empty,
            '>' => Cell::East,
            'v' => Cell::South,
            _ => panic!("Invalid cell value!"),
        }
    }
}

struct Field {
    ylen: usize,
    xlen: usize,
}

impl Field {
    fn new(data: &[Vec<Cell>]) -> Field {
        Field {
            ylen: data.len(),
            xlen: data[0].len(),
        }
    }
    fn next(&self, dir: Cell, y: usize, x: usize) -> (usize, usize) {
        match dir {
            Cell::East => (y, (x + 1) % self.xlen),
            Cell::South => ((y + 1) % self.ylen, x),
            Cell::Empty => panic!("Invalid cell for 'next'"),
        }
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.ylen)
            .into_iter()
            .map(move |y| (0..self.xlen).into_iter().map(move |x| (y, x)))
            .flatten()
    }
}

fn count(data: &[Vec<Cell>]) -> i32 {
    let mut cucumbers = data.to_owned();
    let field = Field::new(&data);
    for count in 1.. {
        let mut moved = false;
        for dir in [Cell::East, Cell::South] {
            let mut newc = cucumbers.to_owned();
            for (y, x) in field.iter().filter(|(y, x)| cucumbers[*y][*x] == dir) {
                let (y1, x1) = field.next(dir, y, x);
                if cucumbers[y1][x1] == Cell::Empty {
                    moved = true;
                    newc[y][x] = Cell::Empty;
                    newc[y1][x1] = dir;
                }
            }
            cucumbers = newc;
        }
        if !moved {
            return count;
        }
    }
    i32::MAX
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<Vec<Cell>> = input
        .iter()
        .map(|l| l.chars().map(Cell::parse).collect())
        .collect();
    match step {
        Step::First => count(&data).to_string(),
        Step::Second => "No problem".to_string(),
    }
}
