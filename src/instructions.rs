use crate::errors::{display_error, ErrorCode};
use crate::m_types::{address, mutez, nat};
use crate::stack::{Stack, StackSnapshots};
use regex::Regex;
use serde_json::Value;
mod ABS;
mod ADD;
mod ADDRESS;
mod AMOUNT;
mod AND;
mod BALANCE;
mod CAR;
mod CDR;
mod CHAIN_ID;
mod COMPARE;
mod CONCAT;
mod CONS;
mod DIG;
mod DROP;
mod DUG;
mod DUP;
mod EDIV;
mod EMPTY_COLLECTION;
mod EQ;
mod GE;
mod GET;
mod GT;
mod IF;
mod IF_LEFT;
mod INT;
mod ISNAT;
mod KECCAK;
mod LE;
mod LEFT_RIGHT;
mod LEVEL;
mod LT;
mod MAP;
mod MEM;
mod MUL;
mod NEG;
mod NEQ;
mod NEVER;
mod NIL;
mod NONE;
mod NOT;
mod NOW;
mod OR;
mod PAIR;
mod PUSH;
mod SELF_ADDRESS;
mod SENDER;
mod SIZE;
mod SLICE;
mod SOME;
mod SOURCE;
mod SUB;
mod SUB_MUTEZ;
mod SWAP;
mod TICKET;
mod UNIT;
mod UNPAIR;
mod UPDATE;
mod XOR;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Instruction {
    ABS,
    ADD,
    ADDRESS,
    AND,
    BALANCE,
    AMOUNT,
    CAR,
    CDR,
    CHAIN_ID,
    COMPARE,
    CONCAT,
    CONS,
    DIG,
    DROP,
    DUG,
    DUP,
    EDIV,
    EMPTY_BIG_MAP,
    EMPTY_MAP,
    EMPTY_SET,
    EQ,
    FAILWITH,
    GE,
    GET,
    GT,
    IF,
    IF_LEFT,
    INT,
    ISNAT,
    KECCAK,
    LE,
    LEFT,
    LEVEL,
    LT,
    MAP,
    MEM,
    MUL,
    NEG,
    NEQ,
    NEVER,
    NIL,
    NONE,
    NOT,
    NOW,
    OR,
    PAIR,
    PUSH,
    RIGHT,
    SELF_ADDRESS,
    SENDER,
    SIZE,
    SLICE,
    SOME,
    SOURCE,
    SUB,
    SUB_MUTEZ,
    SWAP,
    TICKET,
    UNIT,
    UNPAIR,
    UPDATE,
    XOR,
    INIT, // used to initialize the stack
}

pub struct RunOptionsContext {
    pub amount: mutez,
    pub sender: address,
    pub source: address,
    pub self_address: address,
    pub balance: mutez,
    pub level: nat,
    pub chain_id: String,
}

impl RunOptionsContext {
    pub fn mock() -> RunOptionsContext {
        RunOptionsContext {
            amount: 0,
            sender: String::from("test_sender"),
            source: String::from("test_source"),
            self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
            balance: 50_000_000,
            level: 11,
            chain_id: String::from("chain_id"),
        }
    }
}

pub struct RunOptions {
    pub context: RunOptionsContext,
    pub pos: usize,
}

// for LEFT and RIGHT instructions
pub enum LeftOrRight {
    Left,
    Right,
}

// for EMPTY_SET, EMPTY_MAP and EMPTY_BIGMAP instructions
pub enum EmptyCollection {
    Set,
    Map,
    Bigmap,
}

impl Instruction {
    /// Converts a string to an instruction type
    pub fn from_str(input: &str) -> Result<Instruction, String> {
        let format_regex = Regex::new(r"[^A-Z_]").unwrap();
        let formatted_input: &str = &format_regex.replace_all(input, "").to_string();
        match formatted_input {
            "ABS" => Ok(Instruction::ABS),
            "ADD" => Ok(Instruction::ADD),
            "ADDRESS" => Ok(Instruction::ADDRESS),
            "AND" => Ok(Instruction::AND),
            "AMOUNT" => Ok(Instruction::AMOUNT),
            "BALANCE" => Ok(Instruction::BALANCE),
            "CAR" => Ok(Instruction::CAR),
            "CDR" => Ok(Instruction::CDR),
            "CHAIN_ID" => Ok(Instruction::CHAIN_ID),
            "COMPARE" => Ok(Instruction::COMPARE),
            "CONCAT" => Ok(Instruction::CONCAT),
            "CONS" => Ok(Instruction::CONS),
            "DIG" => Ok(Instruction::DIG),
            "DROP" => Ok(Instruction::DROP),
            "DUG" => Ok(Instruction::DUG),
            "DUP" => Ok(Instruction::DUP),
            "EDIV" => Ok(Instruction::EDIV),
            "EMPTY_BIG_MAP" => Ok(Instruction::EMPTY_BIG_MAP),
            "EMPTY_MAP" => Ok(Instruction::EMPTY_MAP),
            "EMPTY_SET" => Ok(Instruction::EMPTY_SET),
            "EQ" => Ok(Instruction::EQ),
            "IF" => Ok(Instruction::IF),
            "IF_LEFT" => Ok(Instruction::IF_LEFT),
            "INT" => Ok(Instruction::INT),
            "ISNAT" => Ok(Instruction::ISNAT),
            "KECCAK" => Ok(Instruction::KECCAK),
            "GE" => Ok(Instruction::GE),
            "GET" => Ok(Instruction::GET),
            "GT" => Ok(Instruction::GT),
            "LE" => Ok(Instruction::LE),
            "LEFT" => Ok(Instruction::LEFT),
            "LEVEL" => Ok(Instruction::LEVEL),
            "LT" => Ok(Instruction::LT),
            "MAP" => Ok(Instruction::MAP),
            "MEM" => Ok(Instruction::MEM),
            "MUL" => Ok(Instruction::MUL),
            "NEG" => Ok(Instruction::NEG),
            "NEQ" => Ok(Instruction::NEQ),
            "NEVER" => Ok(Instruction::NEVER),
            "NIL" => Ok(Instruction::NIL),
            "NONE" => Ok(Instruction::NONE),
            "NOT" => Ok(Instruction::NOT),
            "NOW" => Ok(Instruction::NOW),
            "OR" => Ok(Instruction::OR),
            "PAIR" => Ok(Instruction::PAIR),
            "PUSH" => Ok(Instruction::PUSH),
            "RIGHT" => Ok(Instruction::RIGHT),
            "SELF_ADDRESS" => Ok(Instruction::SELF_ADDRESS),
            "SENDER" => Ok(Instruction::SENDER),
            "SIZE" => Ok(Instruction::SIZE),
            "SLICE" => Ok(Instruction::SLICE),
            "SOME" => Ok(Instruction::SOME),
            "SOURCE" => Ok(Instruction::SOURCE),
            "SUB" => Ok(Instruction::SUB),
            "SUB_MUTEZ" => Ok(Instruction::SUB_MUTEZ),
            "SWAP" => Ok(Instruction::SWAP),
            "UNIT" => Ok(Instruction::UNIT),
            "TICKET" => Ok(Instruction::TICKET),
            "UNPAIR" => Ok(Instruction::UNPAIR),
            "UPDATE" => Ok(Instruction::UPDATE),
            "XOR" => Ok(Instruction::XOR),
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
            Instruction::ADDRESS => ADDRESS::run(initial_stack, options, stack_snapshots),
            Instruction::AND => AND::run(initial_stack, options, stack_snapshots),
            Instruction::AMOUNT => AMOUNT::run(initial_stack, options, stack_snapshots),
            Instruction::BALANCE => BALANCE::run(initial_stack, options, stack_snapshots),
            Instruction::CAR => CAR::run(initial_stack, options, stack_snapshots),
            Instruction::CDR => CDR::run(initial_stack, options, stack_snapshots),
            Instruction::CHAIN_ID => CHAIN_ID::run(initial_stack, options, stack_snapshots),
            Instruction::COMPARE => COMPARE::run(initial_stack, options, stack_snapshots),
            Instruction::CONCAT => CONCAT::run(initial_stack, options, stack_snapshots),
            Instruction::CONS => CONS::run(initial_stack, options, stack_snapshots),
            Instruction::DIG => DIG::run(initial_stack, args, options, stack_snapshots),
            Instruction::DROP => DROP::run(initial_stack, args, options, stack_snapshots),
            Instruction::DUG => DUG::run(initial_stack, args, options, stack_snapshots),
            Instruction::DUP => DUP::run(initial_stack, args, options, stack_snapshots),
            Instruction::EDIV => EDIV::run(initial_stack, options, stack_snapshots),
            Instruction::EMPTY_BIG_MAP => EMPTY_COLLECTION::run(
                initial_stack,
                args,
                options,
                stack_snapshots,
                EmptyCollection::Bigmap,
            ),
            Instruction::EMPTY_MAP => EMPTY_COLLECTION::run(
                initial_stack,
                args,
                options,
                stack_snapshots,
                EmptyCollection::Map,
            ),
            Instruction::EMPTY_SET => EMPTY_COLLECTION::run(
                initial_stack,
                args,
                options,
                stack_snapshots,
                EmptyCollection::Set,
            ),
            Instruction::EQ => EQ::run(initial_stack, options, stack_snapshots),
            Instruction::GE => GE::run(initial_stack, options, stack_snapshots),
            Instruction::GET => GET::run(initial_stack, args, options, stack_snapshots),
            Instruction::GT => GT::run(initial_stack, options, stack_snapshots),
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
            Instruction::ISNAT => ISNAT::run(initial_stack, options, stack_snapshots),
            Instruction::KECCAK => KECCAK::run(initial_stack, options, stack_snapshots),
            Instruction::LE => LE::run(initial_stack, options, stack_snapshots),
            Instruction::LEFT => LEFT_RIGHT::run(
                initial_stack,
                args,
                options,
                stack_snapshots,
                LeftOrRight::Left,
            ),
            Instruction::LEVEL => LEVEL::run(initial_stack, options, stack_snapshots),
            Instruction::LT => LT::run(initial_stack, options, stack_snapshots),
            Instruction::MAP => MAP::run(initial_stack, args, options, stack_snapshots),
            Instruction::MEM => MEM::run(initial_stack, options, stack_snapshots),
            Instruction::MUL => MUL::run(initial_stack, options, stack_snapshots),
            Instruction::NEG => NEG::run(initial_stack, options, stack_snapshots),
            Instruction::NEQ => NEQ::run(initial_stack, options, stack_snapshots),
            Instruction::NEVER => NEVER::run(initial_stack, options, stack_snapshots),
            Instruction::NIL => NIL::run(initial_stack, args, options, stack_snapshots),
            Instruction::NONE => NONE::run(initial_stack, args, options, stack_snapshots),
            Instruction::NOT => NOT::run(initial_stack, options, stack_snapshots),
            Instruction::NOW => NOW::run(initial_stack, options, stack_snapshots),
            Instruction::OR => OR::run(initial_stack, options, stack_snapshots),
            Instruction::PAIR => PAIR::run(initial_stack, options, stack_snapshots),
            Instruction::PUSH => PUSH::run(initial_stack, args, options, stack_snapshots),
            Instruction::RIGHT => LEFT_RIGHT::run(
                initial_stack,
                args,
                options,
                stack_snapshots,
                LeftOrRight::Right,
            ),
            Instruction::SELF_ADDRESS => SELF_ADDRESS::run(initial_stack, options, stack_snapshots),
            Instruction::SENDER => SENDER::run(initial_stack, options, stack_snapshots),
            Instruction::SIZE => SIZE::run(initial_stack, options, stack_snapshots),
            Instruction::SLICE => SLICE::run(initial_stack, options, stack_snapshots),
            Instruction::SOME => SOME::run(initial_stack, options, stack_snapshots),
            Instruction::SOURCE => SOURCE::run(initial_stack, options, stack_snapshots),
            Instruction::SUB => SUB::run(initial_stack, options, stack_snapshots),
            Instruction::SUB_MUTEZ => SUB_MUTEZ::run(initial_stack, options, stack_snapshots),
            Instruction::SWAP => SWAP::run(initial_stack, options, stack_snapshots),
            Instruction::TICKET => TICKET::run(initial_stack, options, stack_snapshots),
            Instruction::UNIT => UNIT::run(initial_stack, options, stack_snapshots),
            Instruction::UNPAIR => UNPAIR::run(initial_stack, options, stack_snapshots),
            Instruction::UPDATE => UPDATE::run(initial_stack, args, options, stack_snapshots),
            Instruction::XOR => XOR::run(initial_stack, options, stack_snapshots),
            _ => panic!("Invalid instruction {:?}", self),
        };
        match result {
            Ok(res) => res,
            Err(err) => panic!("{}", err),
        }
    }
}
