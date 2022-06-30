use crate::m_types::{address, mutez};
use crate::stack::{Stack, StackSnapshots};
use regex::Regex;
use serde_json::Value;
mod ABS;
mod ADD;
mod AMOUNT;
mod COMPARE;
mod DROP;
mod EQ;
mod GE;
mod GT;
mod IF_LEFT;
mod LE;
mod LT;
mod NEQ;
mod NIL;
mod PAIR;
mod PUSH;
mod SOME;
mod SUB;
mod SWAP;
mod UNPAIR;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    ADD,
    ABS,
    AMOUNT,
    COMPARE,
    DROP,
    EQ,
    IF_LEFT,
    GE,
    GT,
    LE,
    LT,
    NEQ,
    NIL,
    PAIR,
    PUSH,
    SOME,
    SUB,
    SWAP,
    UNPAIR,
    INIT, // used to initialize the stack
}

pub struct RunOptionsContext {
    pub amount: mutez,
    pub sender: address,
    pub source: address,
}

pub struct RunOptions {
    pub context: RunOptionsContext,
    pub pos: usize,
}

impl Instruction {
    /// Converts a string to an instruction type
    pub fn from_str(input: &str) -> Result<Instruction, String> {
        let format_regex = Regex::new(r"[^A-Z_]").unwrap();
        let formatted_input: &str = &format_regex.replace_all(input, "").to_string();
        match formatted_input {
            "ADD" => Ok(Instruction::ADD),
            "ABS" => Ok(Instruction::ABS),
            "AMOUNT" => Ok(Instruction::AMOUNT),
            "COMPARE" => Ok(Instruction::COMPARE),
            "DROP" => Ok(Instruction::DROP),
            "EQ" => Ok(Instruction::EQ),
            "IF_LEFT" => Ok(Instruction::IF_LEFT),
            "GE" => Ok(Instruction::GE),
            "GT" => Ok(Instruction::GT),
            "LE" => Ok(Instruction::LE),
            "LT" => Ok(Instruction::LT),
            "NEQ" => Ok(Instruction::NEQ),
            "NIL" => Ok(Instruction::NIL),
            "PAIR" => Ok(Instruction::PAIR),
            "PUSH" => Ok(Instruction::PUSH),
            "SOME" => Ok(Instruction::SOME),
            "SUB" => Ok(Instruction::SUB),
            "SWAP" => Ok(Instruction::SWAP),
            "UNPAIR" => Ok(Instruction::UNPAIR),
            _ => Err(format!("Unknown instruction {}", input)),
        }
    }

    /// Runs the provided instruction against the provided stack, returns the new stack
    pub fn run(
        &self,
        args: Option<&Vec<Value>>,
        initial_stack: Stack,
        stack_snapshots: StackSnapshots,
        options: &RunOptions,
    ) -> (Stack, StackSnapshots) {
        let result = match self {
            Instruction::ABS => ABS::run(initial_stack, options, stack_snapshots),
            Instruction::ADD => ADD::run(initial_stack, options, stack_snapshots),
            Instruction::AMOUNT => AMOUNT::run(initial_stack, options, stack_snapshots),
            Instruction::COMPARE => COMPARE::run(initial_stack, options, stack_snapshots),
            Instruction::DROP => DROP::run(initial_stack, args, options, stack_snapshots),
            Instruction::EQ => EQ::run(initial_stack, options, stack_snapshots),
            Instruction::IF_LEFT => IF_LEFT::run(initial_stack, args, options, stack_snapshots),
            Instruction::GE => GE::run(initial_stack, options, stack_snapshots),
            Instruction::GT => GT::run(initial_stack, options, stack_snapshots),
            Instruction::LE => LE::run(initial_stack, options, stack_snapshots),
            Instruction::LT => LT::run(initial_stack, options, stack_snapshots),
            Instruction::NEQ => NEQ::run(initial_stack, options, stack_snapshots),
            Instruction::NIL => NIL::run(initial_stack, args, options, stack_snapshots),
            Instruction::PAIR => PAIR::run(initial_stack, options, stack_snapshots),
            Instruction::PUSH => PUSH::run(initial_stack, args, options, stack_snapshots),
            Instruction::SOME => SOME::run(initial_stack, options, stack_snapshots),
            Instruction::SUB => SUB::run(initial_stack, options, stack_snapshots),
            Instruction::SWAP => SWAP::run(initial_stack, options, stack_snapshots),
            Instruction::UNPAIR => UNPAIR::run(initial_stack, args, options, stack_snapshots),
            _ => panic!("Invalid instruction {:?}", self),
        };
        match result {
            Ok(res) => res,
            Err(err) => panic!("{}", err),
        }
    }
}
