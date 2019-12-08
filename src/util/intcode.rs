use std::sync::mpsc;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Fail)]
pub enum IntcodeError {
    #[fail(display = "unknown opcode `{}` met at {}", opcode, position)]
    UnknownOpcode {
        position: usize,
        opcode: i32,
    },

    #[fail(display = "input read failed")]
    InputReadFailed(#[fail(cause)] mpsc::RecvError),

    #[fail(display = "input read timed out")]
    InputReadTimedout(#[fail(cause)] mpsc::RecvTimeoutError),

    #[fail(display = "output write failed")]
    OutputWriteFailed(#[fail(cause)] mpsc::SendError<i32>),

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

impl std::convert::From<mpsc::SendError<i32>> for IntcodeError {
    fn from(x: mpsc::SendError<i32>) -> Self {
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
    state: Vec<i32>,
    input: mpsc::Receiver<i32>,
    output: mpsc::Sender<i32>,
    timeout: Option<std::time::Duration>,
}

impl IntcodeVM {
    // Creates an IntcodeVM with given code as the initial state.
    pub fn new(code: &mut Vec<i32>) -> IntcodeVM {
        let (output, input) = mpsc::channel();
        IntcodeVM{state: code.to_vec(), input, output, timeout: None}
    }

    // Returns current state of the VM.
    pub fn state(self) -> Vec<i32> {
        self.state
    }

    // Returns current state of the VM.
    pub fn state_mut(&mut self) -> &mut Vec<i32> {
        &mut self.state
    }

    // Creates a thread that populates this VM's input with provided static dataset.
    // Useful for when you don't need the input channel mechanic.
    pub fn simple_input<'a>(&'a mut self, vec: Vec<i32>) -> &'a mut Self {
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

    // Set input channel to read from.
    pub fn rx<'a>(&'a mut self, rx: mpsc::Receiver<i32>) -> &'a mut Self {
        self.input = rx;
        self
    }

    // Set output channel to write to.
    pub fn tx<'a>(&'a mut self, tx: mpsc::Sender<i32>) -> &'a mut Self {
        self.output = tx;
        self
    }

    pub fn timeout<'a>(&'a mut self, dur: std::time::Duration) -> &'a mut Self {
        self.timeout = Some(dur);
        self
    }

    // Connects this VM's output to other VM's input.
    pub fn wire<'a>(&'a mut self, other: &mut IntcodeVM) -> &'a mut Self {
        let (tx, rx) = mpsc::channel();
        self.output = tx;
        other.input = rx;
        self
    }

    // Executes the VM and collects output into a vec.
    pub fn execute_and_collect(&mut self) -> Result<Vec<i32>, IntcodeError> {
        let (tx, rx) = mpsc::channel();
        self.output = tx;
        self.execute()?;
        Ok(rx.try_iter().collect())
    }

    // MAGICAL SMOKE MACHINE, read the docs @ https://adventofcode.com/2019/day/{2,5,7}.
    pub fn execute(&mut self) -> Result<(), IntcodeError> {
        let mut ptr = 0;

        loop {
            let opcode = self.state[ptr];
            
            let _read = |offset| -> i32 {
                let mut mode = opcode / 100;
                for _ in 1..offset {
                    mode /= 10;
                }
                let value = self.state[ptr + offset];
                if mode % 10 > 0  { value } else { self.state[value as usize] }
            };

            let _write = |code: &mut Vec<i32>, offset, value| {
                let pos = code[ptr + offset] as usize;
                code[pos] = value
            };

            match opcode % 100 {
                // handle add opcode
                1 => {
                    let arg1 = _read(1);
                    let arg2 = _read(2);
                    _write(&mut self.state, 3, arg1 + arg2);
                    ptr += 4;
                }
                // handle multiply opcode
                2 => {
                    let arg1 = _read(1);
                    let arg2 = _read(2);
                    _write(&mut self.state, 3, arg1 * arg2);
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
                5 => {
                    if _read(1) != 0 {
                        ptr = _read(2) as usize;
                    } else {
                        ptr += 3;
                    }
                }
                6 => {
                    if _read(1) == 0 {
                        ptr = _read(2) as usize;
                    } else {
                        ptr += 3;
                    }
                }
                7 => {
                    let result = if _read(1) < _read(2) { 1 } else { 0 };
                    _write(&mut self.state, 3, result);
                    ptr += 4;
                }
                8 => {
                    let result = if _read(1) == _read(2) { 1 } else { 0 };
                    _write(&mut self.state, 3, result);
                    ptr += 4;
                }
                // handle stopcode
                99 => return Ok(()),
                // behave, user.
                _ => return Err(IntcodeError::UnknownOpcode{opcode: self.state[ptr], position: ptr})
            }
        }
    }
}

impl Clone for IntcodeVM {
    // Warning: Due to single consumer limitation, clone of the VM actually has it's own channel.
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
        let code: Vec<i32> = s
            .split_terminator(',')
            .map(|x| x.parse())
            .collect::<Result<Vec<i32>, ParseIntError>>()?;
        Ok(IntcodeVM::new(&mut code.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basic() -> Result<(), IntcodeError> {
        let x = IntcodeVM::from_str("3,9,8,9,10,9,4,9,99,-1,8")?
            .simple_input(vec![8])
            .timeout(std::time::Duration::from_secs(5))
            .execute_and_collect()?;
        assert_eq!(*x.last()?, 1);
        Ok(())
    }
}