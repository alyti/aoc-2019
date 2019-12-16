use super::{error::IntcodeError, DWord};
use std::{
    collections::{BTreeMap, VecDeque},
    ops::{Index, IndexMut},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Ready,
    WaitingForInput,
    Halted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intcode {
    pc: usize,
    state: State,
    base: DWord,
    pub memory: BTreeMap<usize, DWord>,
    pub inputs: VecDeque<DWord>,
}

impl Intcode {
    pub fn new(memory: Vec<DWord>, inputs: Vec<DWord>) -> Self {
        Intcode {
            memory: memory.into_iter().enumerate().collect(),
            inputs: VecDeque::from(inputs),
            pc: 0,
            base: 0,
            state: State::Ready,
        }
    }
    
    /// Returns current memory of the VM.
    pub fn memory(self) -> Vec<DWord> {
        self.memory.values().cloned().collect()
    }

    /// Returns mutable memory of the VM.
    pub fn memory_mut(&mut self) -> &mut BTreeMap<usize, DWord> {
        &mut self.memory
    }

    /// Returns current state of the VM.
    pub fn state(self) -> State {
        self.state
    }
    
    /// MAGICAL SMOKE MACHINE, read the docs @ https://adventofcode.com/2019/day/{2,5,7,9}.
    pub fn step(&mut self) -> Result<Option<DWord>, IntcodeError> {
        if self.state == State::Halted {
            return Err(IntcodeError::Halted);
        }
        let (args, op) = self.parse_opcode()?;
        let mut output = None;
        let new_pc = match op {
            1 => {
                self[args[2]] = self[args[0]] + self[args[1]];
                self.pc + 4
            }
            2 => {
                self[args[2]] = self[args[0]] * self[args[1]];
                self.pc + 4
            }
            3 => match self.inputs.pop_front() {
                Some(value) => {
                    self[args[0]] = value;
                    self.state = State::Ready;
                    self.pc + 2
                }
                None => {
                    self.state = State::WaitingForInput;
                    return Err(IntcodeError::NeedsInput);
                }
            }
            4 => {
                output = Some(self[args[0]]);
                self.pc + 2
            }
            5 => {
                if self[args[0]] != 0 {
                    self[args[1]] as usize
                } else {
                    self.pc + 3
                }
            }
            6 => {
                if self[args[0]] == 0 {
                    self[args[1]] as usize
                } else {
                    self.pc + 3
                }
            }
            7 => {
                self[args[2]] = if self[args[0]] < self[args[1]] {
                    1
                } else {
                    0
                };
                self.pc + 4
            }
            8 => {
                self[args[2]] = if self[args[0]] == self[args[1]] {
                    1
                } else {
                    0
                };
                self.pc + 4
            }
            9 => {
                self.base += self[args[0]];
                self.pc + 2
            }
            99 => {
                self.state = State::Halted;
                return Err(IntcodeError::Halted);
            }
            _ => return Err(IntcodeError::UnknownOpcode{opcode: self[self.pc]}),
        };
        self.pc = new_pc;
        Ok(output)
    }

    fn parse_opcode(&self) -> Result<(Vec<usize>, usize), IntcodeError> {
        let mut opcode = *self.memory.get(&self.pc).unwrap_or(&0) as usize;
        let op = opcode % 100;
        opcode /= 100;
        let mut modes = Vec::with_capacity(4);
        while opcode > 0 {
            modes.push(opcode % 10);
            opcode /= 10;
        }
        modes.resize(4, 0);
        let args = modes
            .into_iter()
            .enumerate()
            .map(|(mut i, mode)| {
                i += 1;
                Ok(match mode {
                    0 => self[i + self.pc] as usize,
                    1 => i + self.pc,
                    2 => (self.base + self[i + self.pc]) as usize,
                    _ => return Err(IntcodeError::InvalidMode{mode: self[self.pc]}),
                })
            })
            .collect::<Result<Vec<usize>, IntcodeError>>()?;
        Ok((args, op))
    }
}

impl Iterator for Intcode {
    type Item = Result<DWord, IntcodeError>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.step() {
                Ok(Some(o)) => return Some(Ok(o)),
                Err(IntcodeError::Halted) => return None,
                Err(e) => return Some(Err(e)),
                _ => (),
            }
        }
    }
}


impl Index<usize> for Intcode {
    type Output = DWord;

    fn index(&self, index: usize) -> &Self::Output {
        self.memory.get(&index).unwrap_or(&0)
    }
}

impl IndexMut<usize> for Intcode {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.memory.entry(index).or_default()
    }
}

use std::num::ParseIntError;

impl From<&str> for Intcode {
    fn from(s: &str) -> Self {
        let code: Vec<DWord> = s
            .split_terminator(',')
            .map(|x| x.parse())
            .collect::<Result<Vec<DWord>, ParseIntError>>().unwrap();
        Intcode::new(code.to_vec(), Vec::new())
    }
}
