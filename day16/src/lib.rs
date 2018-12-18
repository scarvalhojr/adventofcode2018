use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

struct Instr {
    opcode: Opcode,
    in_a: usize,
    in_b: usize,
    out_c: usize,
}

impl Instr {
    fn new(opcode: Opcode, in_a: usize, in_b: usize, out_c: usize) -> Self {
        Instr {
            opcode,
            in_a,
            in_b,
            out_c,
        }
    }

    fn execute(&self, regs: &mut [u64]) {
        assert!(regs.len() > self.out_c);
        regs[self.out_c] = match self.opcode {
            Opcode::ADDR => regs[self.in_a] + regs[self.in_b],
            Opcode::ADDI => regs[self.in_a] + self.in_b as u64,
            Opcode::MULR => regs[self.in_a] * regs[self.in_b],
            Opcode::MULI => regs[self.in_a] * self.in_b as u64,
            Opcode::BANR => regs[self.in_a] & regs[self.in_b],
            Opcode::BANI => regs[self.in_a] & self.in_b as u64,
            Opcode::BORR => regs[self.in_a] | regs[self.in_b],
            Opcode::BORI => regs[self.in_a] | self.in_b as u64,
            Opcode::SETR => regs[self.in_a],
            Opcode::SETI => self.in_a as u64,
            Opcode::GTIR => (self.in_a as u64 > regs[self.in_b]) as u64,
            Opcode::GTRI => (regs[self.in_a] > self.in_b as u64) as u64,
            Opcode::GTRR => (regs[self.in_a] > regs[self.in_b]) as u64,
            Opcode::EQIR => (self.in_a as u64 == regs[self.in_b]) as u64,
            Opcode::EQRI => (regs[self.in_a] == self.in_b as u64) as u64,
            Opcode::EQRR => (regs[self.in_a] == regs[self.in_b]) as u64,
        };
    }
}

type Registers = Vec<u64>;

#[derive(Debug)]
pub struct Sample {
    before: Registers,
    opcode: usize,
    in_a: usize,
    in_b: usize,
    out_c: usize,
    after: Registers,
}

impl Sample {
    fn match_instr(&self, instr: &Instr) -> bool {
        let mut regs = self.before.clone();
        instr.execute(&mut regs);
        self.after == regs
    }

    fn get_matching_opcodes(&self) -> Vec<Opcode> {
        [
            Opcode::ADDR,
            Opcode::ADDI,
            Opcode::MULR,
            Opcode::MULI,
            Opcode::BANR,
            Opcode::BANI,
            Opcode::BORR,
            Opcode::BORI,
            Opcode::SETR,
            Opcode::SETI,
            Opcode::GTIR,
            Opcode::GTRI,
            Opcode::GTRR,
            Opcode::EQIR,
            Opcode::EQRI,
            Opcode::EQRR,
        ]
        .into_iter()
        .filter(|&opcode| {
            self.match_instr(&Instr::new(
                opcode.clone(),
                self.in_a,
                self.in_b,
                self.out_c,
            ))
        })
        .cloned()
        .collect()
    }
}

struct InstrMap {
    mapping: HashMap<usize, Opcode>,
}

impl InstrMap {
    fn build(samples: &[Sample]) -> Option<Self> {
        let mut mapping = HashMap::new();
        let mut possible: HashMap<usize, HashSet<Opcode>> = HashMap::new();

        for sample in samples {
            let opcodes = sample.get_matching_opcodes();
            possible
                .entry(sample.opcode)
                .and_modify(|entry| {
                    *entry = entry
                        .intersection(&opcodes.iter().cloned().collect())
                        .cloned()
                        .collect();
                })
                .or_insert(opcodes.into_iter().collect());
        }

        while let Some((&byte, opcodes)) =
            possible.iter().find(|(_, opcodes)| opcodes.len() == 1)
        {
            let opcode = opcodes.iter().nth(0).unwrap().clone();
            possible.remove(&byte);
            for (_, opcodes) in possible.iter_mut() {
                opcodes.remove(&opcode);
            }
            mapping.insert(byte, opcode);
        }

        if possible.is_empty() {
            Some(InstrMap { mapping })
        } else {
            // A full mapping could not be found
            None
        }
    }

    fn get_opcode(&self, byte: usize) -> Option<Opcode> {
        self.mapping.get(&byte).cloned()
    }
}

pub struct Code {
    bytes: Vec<usize>,
}

struct Program {
    instr: Vec<Instr>,
}

impl Program {
    fn compile(source: &[Code], mapping: &InstrMap) -> Result<Self, String> {
        let instr: Vec<Instr> = source
            .iter()
            .map(|code| match mapping.get_opcode(code.bytes[0]) {
                None => Err("Invalid opcode"),
                Some(opcode) => Ok(Instr::new(
                    opcode,
                    code.bytes[1],
                    code.bytes[2],
                    code.bytes[3],
                )),
            })
            .collect::<Result<_, _>>()?;

        Ok(Program { instr })
    }

    fn execute(&self) -> Registers {
        let mut regs = vec![0; 4];
        for instr in self.instr.iter() {
            instr.execute(&mut regs);
        }
        regs
    }
}

pub fn part1(samples: &[Sample]) -> usize {
    samples
        .iter()
        .filter(|sample| sample.get_matching_opcodes().len() >= 3)
        .count()
}

pub fn part2(samples: &[Sample], source: &[Code]) -> u64 {
    let mapping = match InstrMap::build(samples) {
        Some(mapping) => mapping,
        None => {
            println!("Could not resolve opcodes");
            return 0;
        }
    };
    let prog = match Program::compile(source, &mapping) {
        Ok(prog) => prog,
        Err(err) => {
            println!("Compilation failed: {}", err);
            return 0;
        }
    };
    let regs = prog.execute();
    regs[0]
}

impl FromStr for Sample {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(concat!(
            r"^Before:\s+\[(\d+), (\d+), (\d+), (\d+)\]",
            r"(\d+) (\d+) (\d+) (\d+)",
            r"After:\s+\[(\d+), (\d+), (\d+), (\d+)\]",
        ))
        .unwrap();

        let groups = pattern
            .captures(s)
            .ok_or(Error::new(ErrorKind::InvalidData, "Invalid format"))?;

        let values: Vec<usize> = groups
            .iter()
            .skip(1)
            .map(|val| {
                val.unwrap()
                    .as_str()
                    .parse()
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))
            })
            .collect::<Result<_, _>>()?;

        Ok(Sample {
            before: values[0..=3].iter().map(|v| *v as u64).collect(),
            opcode: values[4],
            in_a: values[5],
            in_b: values[6],
            out_c: values[7],
            after: values[8..=11].iter().map(|v| *v as u64).collect(),
        })
    }
}

impl FromStr for Code {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();

        let groups = pattern
            .captures(s)
            .ok_or(Error::new(ErrorKind::InvalidData, "Invalid format"))?;

        let bytes: Vec<usize> = groups
            .iter()
            .skip(1)
            .map(|val| {
                val.unwrap()
                    .as_str()
                    .parse()
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))
            })
            .collect::<Result<_, _>>()?;

        Ok(Code { bytes })
    }
}
