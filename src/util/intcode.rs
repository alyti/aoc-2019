use std::sync::mpsc;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::num::ParseIntError;

type DWord = i64;
#[derive(Debug, Fail)]
pub enum IntcodeError {
    #[fail(display = "unknown opcode `{}` met at {}", opcode, position)]
    UnknownOpcode {
        position: usize,
        opcode: DWord,
    },

    #[fail(display = "input read failed")]
    InputReadFailed(#[fail(cause)] mpsc::RecvError),

    #[fail(display = "input read timed out")]
    InputReadTimedout(#[fail(cause)] mpsc::RecvTimeoutError),

    #[fail(display = "output write failed")]
    OutputWriteFailed(#[fail(cause)] mpsc::SendError<DWord>),

    #[fail(display = "code parse failed")]
    BadCode(#[fail(cause)] ParseIntError),

    #[fail(display = "wrapped none")]
    NoneError(std::option::NoneError),
}

impl std::convert::From<mpsc::RecvError> for IntcodeError {
    fn from(x: mpsc::RecvError) -> Self {
        IntcodeError::InputReadFailed(x)
    }
}

impl std::convert::From<mpsc::RecvTimeoutError> for IntcodeError {
    fn from(x: mpsc::RecvTimeoutError) -> Self {
        IntcodeError::InputReadTimedout(x)
    }
}

impl std::convert::From<mpsc::SendError<DWord>> for IntcodeError {
    fn from(x: mpsc::SendError<DWord>) -> Self {
        IntcodeError::OutputWriteFailed(x)
    }
}

impl std::convert::From<ParseIntError> for IntcodeError {
    fn from(x: ParseIntError) -> Self {
        IntcodeError::BadCode(x)
    }
}

impl std::convert::From<std::option::NoneError> for IntcodeError {
    fn from(x: std::option::NoneError) -> Self {
        IntcodeError::NoneError(x)
    }
}

pub struct IntcodeVM{
    state: BTreeMap<usize, DWord>,
    input: mpsc::Receiver<DWord>,
    output: mpsc::Sender<DWord>,
    timeout: Option<std::time::Duration>,
}

impl IntcodeVM {
    /// Creates an IntcodeVM with given code as the initial state.
    pub fn new(code: Vec<DWord>) -> IntcodeVM {
        let (output, input) = mpsc::channel();
        IntcodeVM{state: code.into_iter().enumerate().collect(), input, output, timeout: None}
    }

    /// Returns current state of the VM.
    pub fn state(self) -> Vec<DWord> {
        self.state.values().cloned().collect()
    }

    /// Returns current state of the VM.
    pub fn state_mut(&mut self) -> &mut BTreeMap<usize, DWord> {
        &mut self.state
    }

    /// Creates a thread that populates this VM's input with provided static dataset.
    /// Useful for when you don't need the input channel mechanic.
    pub fn simple_input<'a>(&'a mut self, vec: Vec<DWord>) -> &'a mut Self {
        let (tx, rx) = mpsc::channel();
        self.input = rx;
        // TODO: Consider keeping track of thread and joining it at op99.
        std::thread::spawn(move || {
            for i in vec {
                tx.send(i).unwrap();
            }
        });
        self
    }

    /// Set input channel to read from.
    pub fn rx<'a>(&'a mut self, rx: mpsc::Receiver<DWord>) -> &'a mut Self {
        self.input = rx;
        self
    }

    /// Set output channel to write to.
    pub fn tx<'a>(&'a mut self, tx: mpsc::Sender<DWord>) -> &'a mut Self {
        self.output = tx;
        self
    }

    /// Set timeout for input read.
    pub fn timeout<'a>(&'a mut self, dur: std::time::Duration) -> &'a mut Self {
        self.timeout = Some(dur);
        self
    }

    /// Connects this VM's output to other VM's input.
    pub fn wire<'a>(&'a mut self, other: &mut IntcodeVM) -> &'a mut Self {
        let (tx, rx) = mpsc::channel();
        self.output = tx;
        other.input = rx;
        self
    }

    /// Executes the VM and collects output into a vec.
    pub fn execute_and_collect(&mut self) -> Result<Vec<DWord>, IntcodeError> {
        let (tx, rx) = mpsc::channel();
        self.output = tx;
        self.execute()?;
        Ok(rx.try_iter().collect())
    }

    /// MAGICAL SMOKE MACHINE, read the docs @ https://adventofcode.com/2019/day/{2,5,7,9}.
    pub fn execute(&mut self) -> Result<(), IntcodeError> {
        let mut ptr = 0;
        let mut base = 0;

        loop {
            let opcode = *self.state.get(&ptr).unwrap_or(&0);

            let _mode = |offset| {
                let mut mode = opcode / 100;
                for _ in 1..offset {
                    mode /= 10;
                }
                mode % 10
            };

            let _read = |offset| -> DWord {
                let v = *self.state.get(&(ptr + offset)).unwrap_or(&0);
                match _mode(offset) {
                    // position mode
                    0 => *self.state.get(&(v as usize)).unwrap_or(&0),
                    // value mode
                    1 => v,
                    // relative position mode
                    2 => *self.state.get(&((v as DWord + base) as usize)).unwrap_or(&0),
                    _ => 0,
                }
            };

            let _write = |state: &mut BTreeMap<usize, DWord>, offset, value| {
                let pos = *state.entry(ptr + offset).or_default();
                match _mode(offset) {
                    // position mode
                    0 => *state.entry(pos as usize).or_default() = value,
                    // relative position mode
                    2 => *state.entry((base + pos) as usize).or_default() = value,
                    _ => (),
                }
            };

            match opcode % 100 {
                // handle add opcode
                1 => {
                    let c = _read(1) + _read(2);
                    _write(&mut self.state, 3, c);
                    ptr += 4;
                }
                // handle multiply opcode
                2 => {
                    let c = _read(1) * _read(2);
                    _write(&mut self.state, 3, c);
                    ptr += 4;
                }
                // receive input from channel
                3 => {
                    let dword = match self.timeout {
                        Some(dur) => self.input.recv_timeout(dur)?,
                        None => self.input.recv()?,
                    };
                    _write(&mut self.state, 1, dword);
                    ptr += 2;
                }
                // send output to channel
                4 => {
                    self.output.send(_read(1))?;
                    ptr += 2;
                }
                // jump if true
                5 => {
                    if _read(1) != 0 {
                        ptr = _read(2) as usize;
                    } else {
                        ptr += 3;
                    }
                }
                // jump if false
                6 => {
                    if _read(1) == 0 {
                        ptr = _read(2) as usize;
                    } else {
                        ptr += 3;
                    }
                }
                // less than
                7 => {
                    let result = if _read(1) < _read(2) { 1 } else { 0 };
                    _write(&mut self.state, 3, result);
                    ptr += 4;
                }
                // equals
                8 => {
                    let result = if _read(1) == _read(2) { 1 } else { 0 };
                    _write(&mut self.state, 3, result);
                    ptr += 4;
                }
                // adjust the relative base
                9 => {
                    base += _read(1);
                    ptr += 2;
                }
                // stopcode
                99 => return Ok(()),
                // behave, user.
                _ => return Err(IntcodeError::UnknownOpcode{opcode: opcode, position: ptr})
            }
        }
    }
}

impl Clone for IntcodeVM {
    /// Warning: Due to single consumer limitation, clone of the VM actually has it's own channel.
    fn clone(&self) -> Self {
        let (output, input) = mpsc::channel();
        Self{
            state: self.state.clone(), output, input, timeout: self.timeout.clone()
        }
    }
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl FromStr for IntcodeVM {
    type Err = IntcodeError;

    fn from_str(s: &str) -> Result<Self, IntcodeError> {
        let code: Vec<DWord> = s
            .split_terminator(',')
            .map(|x| x.parse())
            .collect::<Result<Vec<DWord>, ParseIntError>>()?;
        Ok(IntcodeVM::new(code.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse() -> Result<(), IntcodeError> {
        let vm = IntcodeVM::from_str("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99")?;
        assert_eq!(vm.state(), vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
        Ok(())
    }
    #[test]
    fn basic() -> Result<(), IntcodeError> {
        let x = IntcodeVM::from_str("3,9,8,9,10,9,4,9,99,-1,8")?
            .simple_input(vec![8])
            .timeout(std::time::Duration::from_secs(5))
            .execute_and_collect()?;
        assert_eq!(*x.last()?, 1);
        Ok(())
    }

    #[test]
    fn quine() -> Result<(), IntcodeError> {
        let x = IntcodeVM::from_str("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99")?
            .timeout(std::time::Duration::from_secs(5))
            .execute_and_collect()?;
        assert_eq!(x, vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
        Ok(())
    }
}