use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, mutez, nat, MType, MValue, OptionValue, PairValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-EDIV

fn divide_with_int(dividend: int, divisor: int) -> Result<MValue, String> {
    let result_type = MType::Pair(Box::new((MType::Int, MType::Nat)));
    if divisor == 0 {
        Ok(MValue::Option(OptionValue::new(None, result_type)))
    } else {
        let quotient = MValue::Int(dividend / divisor);
        let remainder = MValue::Nat((dividend % divisor) as nat);
        // checks that remainder is a nat
        if remainder.check_nat() {
            Ok(MValue::Option(OptionValue::new(
                Some(MValue::Pair(PairValue::new(quotient, remainder))),
                result_type,
            )))
        } else {
            Err(format!(
                "The division of {} and {} doesn't yield a remainder compatible with type `nat`",
                dividend, divisor
            ))
        }
    }
}

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    let this_instruction = Instruction::EDIV;
    // checks the stack
    stack.check_depth(options.pos + 2, this_instruction)?;
    // pattern matches the different possible numeric types
    let new_val: MValue = match (
        stack[options.pos].get_val(),
        stack[options.pos + 1].get_val(),
    ) {
        (MValue::Int(dividend), MValue::Int(divisor)) => divide_with_int(dividend, divisor),
        (MValue::Int(dividend), MValue::Nat(divisor)) => divide_with_int(dividend, divisor as int),
        (MValue::Nat(dividend), MValue::Int(divisor)) => divide_with_int(dividend as int, divisor),
        (MValue::Nat(dividend), MValue::Nat(divisor)) => {
            let result_type = MType::Pair(Box::new((MType::Nat, MType::Nat)));
            if divisor == 0 {
                Ok(MValue::Option(OptionValue::new(None, result_type)))
            } else {
                let quotient = MValue::Nat(dividend / divisor);
                let remainder = MValue::Nat((dividend % divisor) as nat);
                // checks that remainder is a nat
                if remainder.check_nat() {
                    Ok(MValue::Option(OptionValue::new(
                        Some(MValue::Pair(PairValue::new(quotient, remainder))),
                        result_type,
                    )))
                } else {
                    Err(format!("The division of {} and {} doesn't yield a remainder compatible with type `nat`", dividend, divisor))
                }
            }
        }
        (MValue::Mutez(dividend), MValue::Nat(divisor)) => {
            let result_type = MType::Pair(Box::new((MType::Mutez, MType::Mutez)));
            if divisor == 0 {
                Ok(MValue::Option(OptionValue::new(None, result_type)))
            } else {
                let quotient = MValue::Mutez((dividend / divisor) as mutez);
                let remainder = MValue::Mutez((dividend % divisor) as mutez);
                // checks that remainder is a nat
                if remainder.check_mutez() {
                    Ok(MValue::Option(OptionValue::new(
                        Some(MValue::Pair(PairValue::new(quotient, remainder))),
                        result_type,
                    )))
                } else {
                    Err(format!("The division of {} and {} doesn't yield a remainder compatible with type `mutez`", dividend, divisor))
                }
            }
        }
        (MValue::Mutez(dividend), MValue::Mutez(divisor)) => {
            let result_type = MType::Pair(Box::new((MType::Nat, MType::Mutez)));
            if divisor == 0 {
                Ok(MValue::Option(OptionValue::new(None, result_type)))
            } else {
                let quotient = MValue::Nat((dividend / divisor) as nat);
                let remainder = MValue::Mutez((dividend % divisor) as mutez);
                // checks that remainder is a nat
                if remainder.check_mutez() {
                    Ok(MValue::Option(OptionValue::new(
                        Some(MValue::Pair(PairValue::new(quotient, remainder))),
                        result_type,
                    )))
                } else {
                    Err(format!("The division of {} and {} doesn't yield a remainder compatible with type `mutez`", dividend, divisor))
                }
            }
        }
        (val, MValue::Int(_) | MValue::Nat(_) | MValue::Mutez(_)) => {
            Err(display_error(ErrorCode::InvalidType((
                vec![MType::Int, MType::Nat, MType::Mutez],
                val.get_type(),
                this_instruction,
            ))))
        }
        (MValue::Int(_) | MValue::Nat(_) | MValue::Mutez(_), val) => {
            Err(display_error(ErrorCode::InvalidType((
                vec![MType::Int, MType::Nat, MType::Mutez],
                val.get_type(),
                this_instruction,
            ))))
        }
        _ => Err(String::from("Invalid stack for EDIV instruction")),
    }?;

    // removes the first element of the addition
    let (_, new_stack) = stack.remove_at(options.pos);
    // replaces the second element of the addition with the new value
    let new_stack = new_stack.replace(
        vec![StackElement::new(new_val, this_instruction)],
        options.pos,
    );
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    // returns the stack
    Ok((new_stack, stack_snapshots))
}

/*
    TESTS
*/
#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;

    // PASSING TESTS
    // divide an int with an int
    #[test]
    fn ediv_int_int() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(MValue::Int(4), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(
                        Some(MValue::Pair(PairValue::new(MValue::Int(1), MValue::Nat(2)))),
                        MType::Pair(Box::new((MType::Int, MType::Nat)))
                    ))
                );
                assert_eq!(stack[0].instruction, Instruction::EDIV);
                assert_eq!(stack[1].value, MValue::String(String::from("taquito")));
            }
        }
    }

    // divide an int with a nat
    #[test]
    fn ediv_int_nat() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(MValue::Nat(4), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(
                        Some(MValue::Pair(PairValue::new(MValue::Int(1), MValue::Nat(2)))),
                        MType::Pair(Box::new((MType::Int, MType::Nat)))
                    ))
                );
                assert_eq!(stack[0].instruction, Instruction::EDIV);
                assert_eq!(stack[1].value, MValue::String(String::from("taquito")));
            }
        }
    }

    // divide a nat with an int
    #[test]
    fn ediv_nat_int() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Int(4), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(
                        Some(MValue::Pair(PairValue::new(MValue::Int(1), MValue::Nat(2)))),
                        MType::Pair(Box::new((MType::Int, MType::Nat)))
                    ))
                );
                assert_eq!(stack[0].instruction, Instruction::EDIV);
                assert_eq!(stack[1].value, MValue::String(String::from("taquito")));
            }
        }
    }

    // divide a nat with a nat
    #[test]
    fn ediv_nat_nat() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Nat(4), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(
                        Some(MValue::Pair(PairValue::new(MValue::Nat(1), MValue::Nat(2)))),
                        MType::Pair(Box::new((MType::Nat, MType::Nat)))
                    ))
                );
                assert_eq!(stack[0].instruction, Instruction::EDIV);
                assert_eq!(stack[1].value, MValue::String(String::from("taquito")));
            }
        }
    }

    // divide a mutez with a nat
    #[test]
    fn ediv_mutez_nat() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Mutez(6), Instruction::INIT),
            StackElement::new(MValue::Nat(4), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(
                        Some(MValue::Pair(PairValue::new(
                            MValue::Mutez(1),
                            MValue::Mutez(2)
                        ))),
                        MType::Pair(Box::new((MType::Mutez, MType::Mutez)))
                    ))
                );
                assert_eq!(stack[0].instruction, Instruction::EDIV);
                assert_eq!(stack[1].value, MValue::String(String::from("taquito")));
            }
        }
    }

    // divide a mutez with a nat
    #[test]
    fn ediv_mutez_mutez() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Mutez(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(4), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(
                        Some(MValue::Pair(PairValue::new(
                            MValue::Nat(1),
                            MValue::Mutez(2)
                        ))),
                        MType::Pair(Box::new((MType::Nat, MType::Mutez)))
                    ))
                );
                assert_eq!(stack[0].instruction, Instruction::EDIV);
                assert_eq!(stack[1].value, MValue::String(String::from("taquito")));
            }
        }
    }

    // divide an int by zero
    #[test]
    fn ediv_int_zero() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(MValue::Int(0), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(
                        None,
                        MType::Pair(Box::new((MType::Int, MType::Nat)))
                    ))
                );
                assert_eq!(stack[0].instruction, Instruction::EDIV);
                assert_eq!(stack[1].value, MValue::String(String::from("taquito")));
            }
        }
    }

    // FAILING TESTS
    // wrong stack
    #[test]
    fn ediv_wrong_stack() {
        let initial_stack: Stack = vec![StackElement::new(MValue::Int(0), Instruction::INIT)];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert!(
                err == String::from(
                    "Unexpected stack length, expected a length of 2 for instruction EDIV, got 1"
                )
            ),
            Ok(_) => assert!(false),
        }
    }

    // wrong stack values
    #[test]
    fn ediv_wrong_stack_values() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("tezos")), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
            StackElement::new(MValue::Int(0), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert!(err == String::from("Invalid stack for EDIV instruction")),
            Ok(_) => assert!(false),
        }
    }

    // wrong value types
    #[test]
    fn ediv_string() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
            StackElement::new(MValue::Int(0), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert!(
                err == String::from(
                    "Invalid type for `EDIV` expected int | nat | mutez, but got string"
                )
            ),
            Ok(_) => assert!(false),
        }
    }
}
