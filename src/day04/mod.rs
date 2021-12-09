use super::common::Step;
use regex::Regex;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("4512"),
        Step::Second => String::from("1924"),
    }
}

fn parse(input: &[String]) -> Vec<Vec<i32>> {
    let re = Regex::new("\\s+").unwrap();
    let mut board = vec![];
    for l in input {
        board.push(
            re.split(l.trim())
                .map(|n| n.parse().expect("Not a number!"))
                .collect(),
        );
    }
    board
}

struct Board<'a> {
    orig: &'a Vec<Vec<i32>>,
    curr: Vec<Vec<i32>>,
}

impl<'a> Board<'a> {
    fn new(board: &Vec<Vec<i32>>) -> Board {
        Board {
            orig: board,
            curr: board.clone(),
        }
    }
    fn count(&self) -> i32 {
        self.curr.iter().flatten().filter(|v| **v >= 0).sum()
    }

    fn check(&mut self, n: i32) -> bool {
        for y in 0..self.orig.len() {
            for x in 0..self.orig[0].len() {
                if self.orig[y][x] == n {
                    self.curr[y][x] = -1;
                    let x_left = (0..self.orig.len())
                        .filter(|v| self.curr[*v][x] >= 0)
                        .count();
                    let y_left = (0..self.orig[0].len())
                        .filter(|v| self.curr[y][*v] >= 0)
                        .count();
                    return x_left == 0 || y_left == 0;
                }
            }
        }
        false
    }
}

fn first_win(numbers: &Vec<i32>, boards: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut copy: Vec<Board> = boards.iter().map(|x| Board::new(x)).collect();
    for n in numbers {
        for board in &mut copy {
            if board.check(*n) {
                return *n * board.count();
            }
        }
    }
    -1
}

fn last_win(numbers: &Vec<i32>, boards: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut copy: Vec<Board> = boards.iter().map(|x| Board::new(x)).collect();
    let mut winner = vec![false; boards.len()];
    let mut winners = 0;
    for n in numbers {
        for b in 0..copy.len() {
            if winner[b] {
                continue;
            }
            if copy[b].check(*n) {
                winner[b] = true;
                winners += 1;
            }
            if winners == boards.len() {
                return *n * copy[b].count();
            }
        }
    }
    -1
}

pub fn solution(step: &Step, input: &Vec<String>) -> String {
    let numbers = input[0]
        .split(',')
        .map(|n| n.parse().expect("Not a number!"))
        .collect();
    let mut boards = vec![];
    for i in 0..input.len() / 6 {
        boards.push(parse(&input[i * 6 + 2..i * 6 + 7]));
    }
    match step {
        Step::First => first_win(&numbers, &boards).to_string(),
        Step::Second => last_win(&numbers, &boards).to_string(),
    }
}
