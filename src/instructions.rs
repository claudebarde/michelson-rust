use crate::errors::{display_error, ErrorCode};
use crate::m_types::{address, mutez, nat};
use crate::stack::{Stack, StackSnapshots};
use regex::Regex;
use serde_json::Value;
mod ABS;
mod ADD;
mod AMOUNT;
mod BALANCE;
mod COMPARE;
mod DIG;
mod DROP;
mod DUG;
mod DUP;
mod EQ;
mod GE;
mod GT;
mod IF;
mod IF_LEFT;
mod INT;
mod LE;
mod LT;
mod MUL;
mod NEQ;
mod NIL;
mod PAIR;
mod PUSH;
mod SELF_ADDRESS;
mod SENDER;
mod SOME;
mod SOURCE;
mod SUB;
mod SWAP;
mod UNIT;
mod UNPAIR;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    ADD,
    ABS,
    BALANCE,
    AMOUNT,
    COMPARE,
    DIG,
    DROP,
    DUG,
    DUP,
    EQ,
    FAILWITH,
    IF,
    IF_LEFT,
    INT,
    GE,
    GT,
    LE,
    LT,
    MUL,
    NEQ,
    NIL,
    PAIR,
    PUSH,
    SELF_ADDRESS,
    SENDER,
    SOME,
    SOURCE,
    SUB,
    SWAP,
    UNIT,
    UNPAIR,
    INIT, // used to initialize the stack
}

pub struct RunOptionsContext {
    pub amount: mutez,
    pub sender: address,
    pub source: address,
    pub self_address: address,
    pub balance: mutez,
    pub level: nat,
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
            "BALANCE" => Ok(Instruction::BALANCE),
            "COMPARE" => Ok(Instruction::COMPARE),
            "DIG" => Ok(Instruction::DIG),
            "DROP" => Ok(Instruction::DROP),
            "DUG" => Ok(Instruction::DUG),
            "DUP" => Ok(Instruction::DUP),
            "EQ" => Ok(Instruction::EQ),
            "IF" => Ok(Instruction::IF),
            "IF_LEFT" => Ok(Instruction::IF_LEFT),
            "INT" => Ok(Instruction::INT),
            "GE" => Ok(Instruction::GE),
            "GT" => Ok(Instruction::GT),
            "LE" => Ok(Instruction::LE),
            "LT" => Ok(Instruction::LT),
            "MUL" => Ok(Instruction::MUL),
            "NEQ" => Ok(Instruction::NEQ),
            "NIL" => Ok(Instruction::NIL),
            "PAIR" => Ok(Instruction::PAIR),
            "PUSH" => Ok(Instruction::PUSH),
            "SELF_ADDRESS" => Ok(Instruction::SELF_ADDRESS),
            "SENDER" => Ok(Instruction::SENDER),
            "SOME" => Ok(Instruction::SOME),
            "SOURCE" => Ok(Instruction::SOURCE),
            "SUB" => Ok(Instruction::SUB),
            "SWAP" => Ok(Instruction::SWAP),
            "UNIT" => Ok(Instruction::UNIT),
            "UNPAIR" => Ok(Instruction::UNPAIR),
            _ => Err(format!("Unknown instruction {}", input)),
        }
    }

    /// Checks if the provided argument is correct
    /// Returns the numeric value from the argument
    pub fn check_num_arg(&self, arg: &Option<&Vec<Value>>) -> Result<usize, String> {
        // instruction argument type
        enum ArgType {
            Required,
            Optional,
            None,
        }
        // checks if the instruction can have a numeric argument
        let arg_type = match self {
            Instruction::DIG | Instruction::DUG => ArgType::Required,
            Instruction::DUP | Instruction::DROP => ArgType::Optional,
            _ => ArgType::None,
        };

        match (arg, arg_type) {
            (_, ArgType::None) => Err(String::from(
                "The {:?} instruction doesn't need a numeric argument",
            )),
            (None, ArgType::Required) => Err(display_error(ErrorCode::NoArgument(self.clone()))),
            (None, ArgType::Optional) => Ok(1),
            (Some(arg), _) => {
                if arg.len() == 1 {
                    let arg = &arg[0];
                    if arg.is_object() && arg.get("int").is_some() {
                        // gets the int value that will be stored as a string
                        match arg.get("int").unwrap().as_str() {
                            None => Err(String::from(format!(
                                "Expected a string in JSON value for {:?}",
                                self
                            ))),
                            Some(str) =>
                            // parse the string into a number
                            {
                                match str.parse::<usize>() {
                                    Err(_) => Err(format!(
                                        "JSON value for {:?} argument is not a valid number: {}",
                                        self, str
                                    )),
                                    Ok(val) => {
                                        // INSTRUCTION 0 is a noop
                                        if val == 0 {
                                            Err(format!(
                                                "{:?}",
                                                ErrorCode::Noop(String::from(format!(
                                                    "{:?} 0 is a noop",
                                                    self
                                                )))
                                            ))
                                        } else {
                                            Ok(val)
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        Err(format!(
                            "Unexpected format for {:?} argument: {:?}",
                            self, arg
                        ))
                    }
                } else {
                    Err(format!(
                        "{:?}",
                        display_error(ErrorCode::UnexpectedArgsNumber((1, arg.len())))
                    ))
                }
            }
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
            Instruction::BALANCE => BALANCE::run(initial_stack, options, stack_snapshots),
            Instruction::COMPARE => COMPARE::run(initial_stack, options, stack_snapshots),
            Instruction::DIG => DIG::run(initial_stack, args, options, stack_snapshots),
            Instruction::DROP => DROP::run(initial_stack, args, options, stack_snapshots),
            Instruction::DUG => DUG::run(initial_stack, args, options, stack_snapshots),
            Instruction::DUP => DUP::run(initial_stack, args, options, stack_snapshots),
            Instruction::EQ => EQ::run(initial_stack, options, stack_snapshots),
            Instruction::IF => {
                match IF::run(initial_stack, args, options, stack_snapshots) {
                    // the boolean value in RunResult is not necessary here
                    Ok(run_result) => Ok((run_result.stack, run_result.stack_snapshots)),
                    Err(err) => Err(err),
                }
            }
            Instruction::IF_LEFT => {
                match IF_LEFT::run(initial_stack, args, options, stack_snapshots) {
                    // the boolean value in RunResult is not necessary here
                    Ok(run_result) => Ok((run_result.stack, run_result.stack_snapshots)),
                    Err(err) => Err(err),
                }
            }
            Instruction::INT => INT::run(initial_stack, options, stack_snapshots),
            Instruction::GE => GE::run(initial_stack, options, stack_snapshots),
            Instruction::GT => GT::run(initial_stack, options, stack_snapshots),
            Instruction::LE => LE::run(initial_stack, options, stack_snapshots),
            Instruction::LT => LT::run(initial_stack, options, stack_snapshots),
            Instruction::MUL => MUL::run(initial_stack, options, stack_snapshots),
            Instruction::NEQ => NEQ::run(initial_stack, options, stack_snapshots),
            Instruction::NIL => NIL::run(initial_stack, args, options, stack_snapshots),
            Instruction::PAIR => PAIR::run(initial_stack, options, stack_snapshots),
            Instruction::PUSH => PUSH::run(initial_stack, args, options, stack_snapshots),
            Instruction::SELF_ADDRESS => SELF_ADDRESS::run(initial_stack, options, stack_snapshots),
            Instruction::SENDER => SENDER::run(initial_stack, options, stack_snapshots),
            Instruction::SOME => SOME::run(initial_stack, options, stack_snapshots),
            Instruction::SOURCE => SOURCE::run(initial_stack, options, stack_snapshots),
            Instruction::SUB => SUB::run(initial_stack, options, stack_snapshots),
            Instruction::SWAP => SWAP::run(initial_stack, options, stack_snapshots),
            Instruction::UNIT => UNIT::run(initial_stack, options, stack_snapshots),
            Instruction::UNPAIR => UNPAIR::run(initial_stack, args, options, stack_snapshots),
            _ => panic!("Invalid instruction {:?}", self),
        };
        match result {
            Ok(res) => res,
            Err(err) => panic!("{}", err),
        }
    }
}
