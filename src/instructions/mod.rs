use regex::Regex;
use crate::stack::{ Stack };
use crate::m_types::{ mutez, address };
mod ADD;
mod DROP;
mod IF_LEFT;
mod NIL;
mod PAIR;
mod PUSH;
mod SUB;
mod SWAP;
mod UNPAIR;

#[derive(Debug, Clone)]
pub enum Instruction {
    ADD,
    DROP,
    IF_LEFT,
    NIL,
    PAIR,
    PUSH,
    SUB,
    SWAP,
    UNPAIR,
    INIT, // used to initialize the stack
}

pub struct RunOptionsContext {
    pub amount: mutez,
    pub sender: address,
    pub source: address
}

pub struct RunOptions {
    pub context: RunOptionsContext,
    pub pos: usize
}

impl Instruction {
    /// Converts a string to an instruction type
    pub fn from_str(input: &str) -> Result<Instruction, String> {
        let format_regex = Regex::new(r"[^A-Z_]").unwrap();
        let formatted_input: &str = &format_regex.replace_all(input, "").to_string();
        match formatted_input {
            "ADD"       => Ok(Instruction::ADD),
            "DROP"      => Ok(Instruction::DROP),
            "IF_LEFT"   => Ok(Instruction::IF_LEFT),
            "NIL"       => Ok(Instruction::NIL),
            "PAIR"      => Ok(Instruction::PAIR),
            "PUSH"      => Ok(Instruction::PUSH),
            "SUB"       => Ok(Instruction::SUB),
            "SWAP"      => Ok(Instruction::SWAP),
            "UNPAIR"    => Ok(Instruction::UNPAIR),
            _           => Err(format!("Unknown instruction {}", input)),
        }
    }

    /// Runs the provided instruction against the provided stack, returns the new stack
    pub fn run(instruction: Instruction, initial_stack: Stack, options: &RunOptions) -> Stack {
        let new_stack = 
            match instruction {
                Instruction::ADD => ADD::run(initial_stack, options),
                Instruction::DROP => DROP::run(initial_stack, options),
                Instruction::IF_LEFT => IF_LEFT::run(initial_stack, options),
                Instruction::NIL => NIL::run(initial_stack, options),
                Instruction::PAIR => PAIR::run(initial_stack, options),
                Instruction::PUSH => PUSH::run(initial_stack, options),
                Instruction::SUB => SUB::run(initial_stack, options),
                Instruction::SWAP => SWAP::run(initial_stack, options),
                Instruction::UNPAIR => UNPAIR::run(initial_stack, options),
                _ => panic!("Invalid instruction {:?}", instruction)
            };
        match new_stack {
            Ok (stack) => stack,
            Err (err) => panic!("{}", err)
        }
    }
}