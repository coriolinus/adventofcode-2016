use std::{
    ops::{Index, IndexMut},
    thread::JoinHandle,
};

pub type Integer = i32;

#[derive(Clone, Copy, PartialEq, Eq, Debug, parse_display::Display, parse_display::FromStr)]
#[display(style = "lowercase")]
pub enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, parse_display::Display, parse_display::FromStr)]
pub enum Value {
    #[display("{0}")]
    Register(Register),
    #[display("{0}")]
    Value(Integer),
}

impl From<Register> for Value {
    fn from(r: Register) -> Self {
        Self::Register(r)
    }
}

impl From<Integer> for Value {
    fn from(i: Integer) -> Self {
        Self::Value(i)
    }
}

impl Value {
    fn as_register(&self) -> Option<Register> {
        match self {
            Self::Register(register) => Some(*register),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, parse_display::Display, parse_display::FromStr)]
pub enum Instruction {
    #[display("cpy {0} {1}")]
    Copy(Value, Value),
    #[display("inc {0}")]
    Increase(Value),
    #[display("dec {0}")]
    Decrease(Value),
    #[display("jnz {0} {1}")]
    Jnz(Value, Value),
    #[display("tgl {0}")]
    Toggle(Value),
    #[display("out {0}")]
    Out(Value),
}

impl Instruction {
    fn toggle(&mut self) {
        *self = match *self {
            Self::Increase(value) => Self::Decrease(value),
            Self::Decrease(value) => Self::Increase(value),
            Self::Toggle(value) => Self::Increase(value),
            Self::Out(value) => Self::Increase(value),
            Self::Jnz(value, qty) => Self::Copy(value, qty),
            Self::Copy(value, qty) => Self::Jnz(value, qty),
        }
    }
}

#[derive(Default)]
pub struct Computer {
    a: Integer,
    b: Integer,
    c: Integer,
    d: Integer,
    ip: usize,
    program: Vec<Instruction>,
    sender: Option<std::sync::mpsc::SyncSender<Integer>>,
}

impl Computer {
    pub fn from_program(program: Vec<Instruction>) -> Self {
        Self {
            program,
            ..Self::default()
        }
    }

    pub fn set_sender(&mut self, sender: impl Into<Option<std::sync::mpsc::SyncSender<Integer>>>) {
        self.sender = sender.into();
    }

    pub fn value(&self, value: Value) -> Integer {
        match value {
            Value::Register(register) => self[register],
            Value::Value(value) => value,
        }
    }

    fn instruction_offset(&mut self, value: Value) -> Option<&mut Instruction> {
        let next_ip = self.ip as Integer + self.value(value);
        self.program.get_mut(next_ip as usize)
    }

    // `true` when the program should continue; `false` when it should halt
    fn step(&mut self) -> bool {
        match self.program[self.ip] {
            Instruction::Copy(value, register) => {
                register
                    .as_register()
                    .map(|register| self[register] = self.value(value));
            }
            Instruction::Increase(register) => {
                register.as_register().map(|register| self[register] += 1);
            }
            Instruction::Decrease(register) => {
                register.as_register().map(|register| self[register] -= 1);
            }
            Instruction::Jnz(_, _) => {}
            Instruction::Toggle(value) => {
                self.instruction_offset(value)
                    .map(|instruction| instruction.toggle());
            }
            Instruction::Out(value) => {
                let value = self.value(value);
                let sender = match self.sender.as_mut() {
                    Some(sender) => sender,
                    None => return false,
                };
                if sender.send(value).is_err() {
                    return false;
                }
            }
        }

        let next_ip = self.ip as Integer
            + match self.program[self.ip] {
                Instruction::Jnz(value, distance) if self.value(value) != 0 => self.value(distance),
                _ => 1,
            };
        self.ip = if (0..self.program.len()).contains(&(next_ip as usize)) {
            next_ip as usize
        } else {
            !0
        };
        self.ip != !0
    }

    /// Run this computer until the program terminates naturally.
    pub fn run(&mut self) {
        while self.step() {}
    }

    /// Run this computer in its own thread until the program terminates naturally.
    ///
    /// Note that this consumes `self`. Ensure you've `set_sender` before calling this
    /// if you want to receive output!
    pub fn launch(mut self) -> JoinHandle<()> {
        std::thread::spawn(move || self.run())
    }
}

impl Index<Register> for Computer {
    type Output = Integer;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::A => &self.a,
            Register::B => &self.b,
            Register::C => &self.c,
            Register::D => &self.d,
        }
    }
}

impl IndexMut<Register> for Computer {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
        }
    }
}
