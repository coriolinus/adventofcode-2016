use std::ops::{Index, IndexMut};

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

#[derive(Clone, Copy, PartialEq, Eq, Debug, parse_display::Display, parse_display::FromStr)]
pub enum Instruction {
    #[display("cpy {0} {1}")]
    Copy(Value, Register),
    #[display("inc {0}")]
    Increase(Register),
    #[display("dec {0}")]
    Decrease(Register),
    #[display("jnz {0} {1}")]
    Jnz(Value, Integer),
}

#[derive(Default)]
pub struct Computer {
    a: Integer,
    b: Integer,
    c: Integer,
    d: Integer,
    ip: usize,
    program: Vec<Instruction>,
}

impl Computer {
    pub fn from_program(program: Vec<Instruction>) -> Self {
        Self {
            program,
            ..Self::default()
        }
    }

    pub fn value(&self, value: Value) -> Integer {
        match value {
            Value::Register(register) => self[register],
            Value::Value(value) => value,
        }
    }

    // `true` when the program should continue; `false` when it should halt
    fn step(&mut self) -> bool {
        match self.program[self.ip] {
            Instruction::Copy(value, register) => self[register] = self.value(value),
            Instruction::Increase(register) => self[register] += 1,
            Instruction::Decrease(register) => self[register] -= 1,
            Instruction::Jnz(_, _) => {}
        }

        let next_ip = self.ip as Integer
            + match self.program[self.ip] {
                Instruction::Jnz(value, distance) if self.value(value) != 0 => distance,
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
