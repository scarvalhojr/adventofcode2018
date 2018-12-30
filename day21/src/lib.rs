use regex::Regex;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

const NUM_REGS: usize = 6;

type Registers = Vec<usize>;

#[derive(Debug)]
enum Opcode {
    ADDR,
    ADDI,
    MULR,
    MULI,
    BANR,
    BANI,
    BORR,
    BORI,
    SETR,
    SETI,
    GTIR,
    GTRI,
    GTRR,
    EQIR,
    EQRI,
    EQRR,
}

#[derive(Debug)]
struct Instr {
    opcode: Opcode,
    in_a: usize,
    in_b: usize,
    out_c: usize,
}

impl Instr {
    fn execute(&self, regs: &mut Registers) {
        assert!(regs.len() > self.out_c);
        regs[self.out_c] = match self.opcode {
            Opcode::ADDR => regs[self.in_a] + regs[self.in_b],
            Opcode::ADDI => regs[self.in_a] + self.in_b,
            Opcode::MULR => regs[self.in_a] * regs[self.in_b],
            Opcode::MULI => regs[self.in_a] * self.in_b,
            Opcode::BANR => regs[self.in_a] & regs[self.in_b],
            Opcode::BANI => regs[self.in_a] & self.in_b,
            Opcode::BORR => regs[self.in_a] | regs[self.in_b],
            Opcode::BORI => regs[self.in_a] | self.in_b,
            Opcode::SETR => regs[self.in_a],
            Opcode::SETI => self.in_a,
            Opcode::GTIR => (self.in_a > regs[self.in_b]) as usize,
            Opcode::GTRI => (regs[self.in_a] > self.in_b) as usize,
            Opcode::GTRR => (regs[self.in_a] > regs[self.in_b]) as usize,
            Opcode::EQIR => (self.in_a == regs[self.in_b]) as usize,
            Opcode::EQRI => (regs[self.in_a] == self.in_b) as usize,
            Opcode::EQRR => (regs[self.in_a] == regs[self.in_b]) as usize,
        };
    }
}

pub struct Program {
    ip_reg: usize,
    inst_ptr: usize,
    regs: Registers,
    instr: Vec<Instr>,
}

impl Program {
    pub fn run(&mut self) {
        while self.step() {}
    }

    pub fn step(&mut self) -> bool {
        if let Some(instruction) = self.instr.get(self.inst_ptr) {
            self.regs[self.ip_reg] = self.inst_ptr;
            instruction.execute(&mut self.regs);
            self.inst_ptr = 1 + self.regs[self.ip_reg];
            true
        } else {
            // Halt
            false
        }
    }

    pub fn get_register(&self, reg_num: usize) -> usize {
        *self.regs.get(reg_num).unwrap_or(&0)
    }

    pub fn get_inst_ptr(&self) -> usize {
        self.inst_ptr
    }

    pub fn reset(&mut self) {
        self.inst_ptr = 0;
        for reg in self.regs.iter_mut() {
            *reg = 0;
        }
    }
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line =
            lines.next().ok_or_else(|| "Empty source".to_string())?;
        let ip_pattern = Regex::new(r"^#ip (\d+)$").unwrap();
        let captures = ip_pattern.captures(first_line).ok_or_else(|| {
            "Invalid instruction pointer declaration".to_string()
        })?;
        let ip_reg = captures
            .iter()
            .nth(1)
            .unwrap()
            .unwrap()
            .as_str()
            .parse()
            .map_err(|err: ParseIntError| err.to_string())?;
        let instr = lines.map(|line| line.parse()).collect::<Result<_, _>>()?;

        Ok(Program {
            ip_reg,
            inst_ptr: 0,
            regs: vec![0; NUM_REGS],
            instr,
        })
    }
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"(\w+) (\d+) (\d+) (\d+)").unwrap();
        let captures = pattern
            .captures(s)
            .ok_or_else(|| "Invalid instruction".to_string())?;
        let mut tokens = captures.iter().skip(1);
        let opcode = match tokens.next().unwrap().unwrap().as_str() {
            "addr" => Opcode::ADDR,
            "addi" => Opcode::ADDI,
            "mulr" => Opcode::MULR,
            "muli" => Opcode::MULI,
            "banr" => Opcode::BANR,
            "bani" => Opcode::BANI,
            "borr" => Opcode::BORR,
            "bori" => Opcode::BORI,
            "setr" => Opcode::SETR,
            "seti" => Opcode::SETI,
            "gtir" => Opcode::GTIR,
            "gtri" => Opcode::GTRI,
            "gtrr" => Opcode::GTRR,
            "eqir" => Opcode::EQIR,
            "eqri" => Opcode::EQRI,
            "eqrr" => Opcode::EQRR,
            _ => return Err("Unknown operation".to_string()),
        };
        let operand = tokens
            .map(|val| {
                val.unwrap()
                    .as_str()
                    .parse()
                    .map_err(|err: ParseIntError| err.to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Instr {
            opcode,
            in_a: operand[0],
            in_b: operand[1],
            out_c: operand[2],
        })
    }
}

pub fn part1(program: &mut Program) -> usize {
    program.reset();
    while program.step() {
        if program.get_inst_ptr() == 28 {
            // Instruction 28 halts the program when register 0
            // equals register 1
            return program.get_register(1);
        }
    }
    0
}

pub fn part2(program: &mut Program) -> usize {
    let mut values = HashSet::new();
    let mut last_val = 0;
    program.reset();
    while program.step() {
        if program.get_inst_ptr() == 28 {
            // Remember every value in register 0 that would cause a halt
            if !values.insert(program.get_register(1)) {
                // The value before a repeat is the one that
                // would halt the program after most instructions
                return last_val;
            }
            last_val = program.get_register(1);
        }
    }
    0
}
