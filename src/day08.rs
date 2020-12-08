use std::{collections::HashSet, error::Error};

use crate::try_parse_lines;

pub fn run() -> Result<(), Box<dyn Error>> {
    let instructions = try_parse_lines(8, |line| parse_instruction(&line))?;
    let mut cpu = Cpu::new(instructions);

    let mut visited = HashSet::new();
    loop {
        if !visited.insert(cpu.ip) {
            break;
        }

        cpu.next();
    }

    println!("Day 8a: {}", cpu.accumulator);

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn parse_instruction(s: &str) -> Result<Instruction, Box<dyn Error>> {
    let mut split = s.split(" ");
    let opcode = split.next().expect("opcode");
    let value = split.next().expect("value").parse::<i32>()?;
    assert!(split.next().is_none());

    let opcode = match opcode {
        "acc" => Instruction::Acc,
        "jmp" => Instruction::Jmp,
        "nop" => Instruction::Nop,
        _ => panic!("Opcode: {}", opcode),
    };

    Ok(opcode(value))
}

struct Cpu {
    instructions: Vec<Instruction>,
    ip: usize,
    accumulator: i32,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            ip: 0,
            accumulator: 0,
        }
    }

    fn next(&mut self) {
        match self.instructions[self.ip] {
            Instruction::Acc(value) => {
                self.accumulator += value;
                self.ip += 1;
            }
            Instruction::Jmp(value) => {
                self.ip = (self.ip as i32 + value) as usize;
            }
            Instruction::Nop(_) => {
                self.ip += 1;
            }
        }
    }
}
