use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => String::from("Valid"),
        Step::Second => String::from("Invalid"),
    }
}

enum Val {
    Const(i32),
    Reg(usize),
}

enum Inst {
    Inp(usize),
    Add(usize, Val),
    Mul(usize, Val),
    Div(usize, Val),
    Mod(usize, Val),
    Eql(usize, Val),
}

const REG: [&str; 4] = ["w", "x", "y", "z"];
fn registry(reg: &str) -> usize {
    REG.iter().position(|&r| r == reg).expect("Not a registry!")
}
fn value(val: &str) -> Val {
    if let Some(p) = REG.iter().position(|&r| r == val) {
        Val::Reg(p)
    } else {
        Val::Const(val.parse().expect("Not a number!"))
    }
}

impl Inst {
    fn parse(line: &str) -> Inst {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[0] {
            "inp" => Inst::Inp(registry(&parts[1])),
            "add" => Inst::Add(registry(&parts[1]), value(&parts[2])),
            "mul" => Inst::Mul(registry(&parts[1]), value(&parts[2])),
            "div" => Inst::Div(registry(&parts[1]), value(&parts[2])),
            "mod" => Inst::Mod(registry(&parts[1]), value(&parts[2])),
            "eql" => Inst::Eql(registry(&parts[1]), value(&parts[2])),
            _ => panic!("Unknown instruction!"),
        }
    }
}

struct Alu {
    input: Vec<i32>,
    index: usize,
    registers: [i32; REG.len()],
}

impl Alu {
    fn value(&self, val: &Val) -> i32 {
        match val {
            Val::Const(val) => *val,
            Val::Reg(reg) => self.registers[*reg],
        }
    }
    fn execute(&mut self, call: &Inst) {
        match call {
            Inst::Inp(reg) => {
                self.registers[*reg] = self.input[self.index];
                self.index += 1;
            }
            Inst::Add(reg, val) => {
                self.registers[*reg] += self.value(val);
            }
            Inst::Mul(reg, val) => {
                self.registers[*reg] *= self.value(val);
            }
            Inst::Div(reg, val) => {
                self.registers[*reg] /= self.value(val);
            }
            Inst::Mod(reg, val) => {
                self.registers[*reg] %= self.value(val);
            }
            Inst::Eql(reg, val) => {
                self.registers[*reg] = if self.registers[*reg] == self.value(val) {
                    1
                } else {
                    0
                };
            }
        }
    }
}

fn count(data: &[Inst], input: &str) -> String {
    let mut alu = Alu {
        input: input
            .chars()
            .map(|c| c.to_digit(10).expect("Not a digit!") as i32)
            .collect(),
        index: 0,
        registers: [0; 4],
    };
    for inst in data {
        alu.execute(&inst);
    }
    if alu.registers[3] == 0 {
        "Valid".to_string()
    } else {
        "Invalid".to_string()
    }
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<Inst> = input.iter().map(|line| Inst::parse(line)).collect();
    match step {
        Step::First => count(&data, "89959794919939"),
        Step::Second => count(&data, "17115131916112"),
    }
}
