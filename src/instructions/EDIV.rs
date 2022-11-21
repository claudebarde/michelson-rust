use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, nat, MType, MValue, OptionValue, PairValue};
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
    // checks the stack
    match stack.check_depth(options.pos + 2, Instruction::EDIV) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // pattern matches the different possible numeric types
    let new_val: Result<MValue, String> = match (
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
                let quotient = MValue::Mutez(dividend / divisor);
                let remainder = MValue::Mutez((dividend % divisor) as nat);
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
                let quotient = MValue::Nat(dividend / divisor);
                let remainder = MValue::Mutez((dividend % divisor) as nat);
                // checks that remainder is a nat
                if remainder.check_nat() {
                    Ok(MValue::Option(OptionValue::new(
                        Some(MValue::Pair(PairValue::new(quotient, remainder))),
                        result_type,
                    )))
                } else {
                    Err(format!("The division of {} and {} doesn't yield a remainder compatible with type `mutez`", dividend, divisor))
                }
            }
        }
        (val, MValue::Int(_) | MValue::Nat(_)) => Err(display_error(ErrorCode::InvalidType((
            vec![MType::Int, MType::Nat],
            val.get_type(),
            Instruction::EDIV,
        )))),
        (MValue::Int(_) | MValue::Nat(_), val) => Err(display_error(ErrorCode::InvalidType((
            vec![MType::Int, MType::Nat],
            val.get_type(),
            Instruction::EDIV,
        )))),
        _ => Err(String::from("Invalid stack for EDIV instruction")),
    };

    let new_stack = stack;

    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    // returns the stack
    Ok((new_stack, stack_snapshots))
}
