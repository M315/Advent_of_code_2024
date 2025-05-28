advent_of_code::solution!(17);

#[derive(Debug, Copy, Clone)]
pub struct Register {
    a: u64,
    b: u64,
    c: u64,
}

impl Register {
    pub fn compute(&mut self, opcode: u64, operand: u64) -> (Option<u64>, Option<usize>) {
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => unreachable!()
        }
    }

    pub fn combo_value(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!()
        }
    }

    fn adv(&mut self, operand: u64) -> (Option<u64>, Option<usize>) {
        self.a = self.a / (1 << self.combo_value(operand));
        (None, None)
    }

    fn bxl(&mut self, operand: u64) -> (Option<u64>, Option<usize>) {
        self.b ^= operand;
        (None, None)
    }

    fn bst(&mut self, operand: u64) -> (Option<u64>, Option<usize>) {
        self.b = self.combo_value(operand).rem_euclid(8);
        (None, None)
    }

    fn jnz(&mut self, operand: u64) -> (Option<u64>, Option<usize>) {
        let jump = match self.a {
            0 => None,
            _ => Some(operand as usize),
        };
        (None, jump)
    }

    fn bxc(&mut self, _: u64) -> (Option<u64>, Option<usize>) {
        self.b ^= self.c;
        (None, None)
    }

    fn out(&mut self, operand: u64) -> (Option<u64>, Option<usize>) {
        (Some(self.combo_value(operand).rem_euclid(8)), None)
    }

    fn bdv(&mut self, operand: u64) -> (Option<u64>, Option<usize>) {
        self.b = self.a / (1 << self.combo_value(operand));
        (None, None)
    }

    fn cdv(&mut self, operand: u64) -> (Option<u64>, Option<usize>) {
        self.c = self.a / (1 << self.combo_value(operand));
        (None, None)
    }
}

fn parse(input: &str) -> (Register, Vec<u64>) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers: Vec<_> = registers.lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<u64>().unwrap())
        .collect();
    let program = program.split_once(": ").unwrap().1.split(',').map(|s| s.trim().parse::<u64>().unwrap()).collect();
    (
        Register {
            a: registers[0],
            b: registers[1],
            c: registers[2],
        },
        program
    )
}

fn solve(reg: &mut Register, program: &[u64]) -> Vec<u64> {
    let mut pos = 0;
    let mut output = Vec::new();

    while pos < program.len() - 1 {
        match reg.compute(program[pos], program[pos + 1]) {
            (Some(_), Some(_)) => unreachable!(),
            (None, None) => pos += 2,
            (Some(out), None) => {
                output.push(out);
                pos += 2;
            },
            (None, Some(jump)) => pos = jump,
        };
    }

    output
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut reg, program) = parse(input);
    Some(solve(&mut reg, &program).into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
    )
}

fn bfs_inv(program: Vec<u64>) -> Option<u64> {
    let mut candidates: Vec<u64> = (0..8).collect();
    let n = program.len();
    for k in 1..program.len() {
        let mut new_candidates = Vec::new();
        for a in candidates {
            let mut reg = Register { a, b: 0, c: 0 };
            let res = solve(&mut reg, &program);
            if res == program[n - k..n] {
                new_candidates.extend_from_slice((0..8).map(|x| x + a * 8).collect::<Vec<_>>().as_slice());
            }
        }
        candidates = new_candidates;
    }
    candidates.into_iter()
        .filter(|&a| solve(&mut Register { a, b: 0, c: 0 }, &program) == program)
        .next()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, program) = parse(input);
    bfs_inv(program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("5,7,3,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
