use std::{collections::HashSet, error::Error};

use crate::try_parse_lines;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut instructions = try_parse_lines(8, |line| parse_instruction(&line))?;

    let mut cpu = Cpu::new(&instructions);
    let mut visited_forward = HashSet::new();
    loop {
        if !visited_forward.insert(cpu.ip) {
            break;
        }
        cpu.next();
    }
    println!("Day 8a: {}", cpu.accumulator);

    let patch = get_patch_address(&instructions, &visited_forward).expect("patch address");
    let patch_insn = instructions.get_mut(patch).unwrap();
    match patch_insn {
        Instruction::Acc(_) => unreachable!(),
        Instruction::Jmp(x) => *patch_insn = Instruction::Nop(*x),
        Instruction::Nop(x) => *patch_insn = Instruction::Jmp(*x),
    }

    let mut cpu = Cpu::new(&instructions);
    while cpu.ip != cpu.instructions.len() {
        cpu.next();
    }
    println!("Day 8b: {}", cpu.accumulator);

    Ok(())
}

fn get_patch_address(instructions: &[Instruction], candidates: &HashSet<usize>) -> Option<usize> {
    let mut todo = vec![{
        let mut cpu = Cpu::new(instructions);
        cpu.ip = cpu.instructions.len();
        cpu
    }];

    while let Some(current) = todo.pop() {
        for before in current.back() {
            match before {
                Ok(before) => {
                    debug_assert!(!candidates.contains(&before.ip));
                    todo.push(before);
                }
                Err(before) => {
                    if candidates.contains(&before.ip) {
                        return Some(before.ip);
                    }
                }
            }
        }
    }

    None
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

#[derive(Clone)]
struct Cpu<'a> {
    instructions: &'a [Instruction],
    ip: usize,
    accumulator: i32,
}

impl<'a> Cpu<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
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

    fn back(self) -> Vec<Result<Self, Self>> {
        let mut result = Vec::new();

        for (idx, insn) in self.instructions.iter().copied().enumerate() {
            let ip_offsets = match insn {
                Instruction::Acc(_) => vec![1],
                Instruction::Jmp(x) => vec![x, 1],
                Instruction::Nop(x) => vec![1, x],
            };

            for (offset_idx, offset) in ip_offsets.into_iter().enumerate() {
                if (idx as i32 + offset) as usize != self.ip {
                    continue;
                }

                let mut cpu = self.clone();
                cpu.ip = idx;
                result.push(if offset_idx == 0 { Ok(cpu) } else { Err(cpu) });
            }
        }

        result
    }
}
