use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{AddressType, MValue, Or};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use std::cmp::Ordering;

// https://tezos.gitlab.io/michelson-reference/#instr-COMPARE

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 2, Instruction::COMPARE) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // creates a separate function to recursively call it for complex types
    fn compare(first_val: &MValue, last_val: &MValue) -> Result<MValue, String> {
        match (first_val, last_val) {
            // int
            (MValue::Int(first_val), MValue::Int(last_val)) => {
                if first_val < last_val {
                    Ok(MValue::Int(-1))
                } else if first_val == last_val {
                    Ok(MValue::Int(0))
                } else {
                    Ok(MValue::Int(1))
                }
            }
            // nat and mutez
            (MValue::Nat(first_val), MValue::Nat(last_val))
            | (MValue::Mutez(first_val), MValue::Mutez(last_val)) => {
                if first_val < last_val {
                    Ok(MValue::Int(-1))
                } else if first_val == last_val {
                    Ok(MValue::Int(0))
                } else {
                    Ok(MValue::Int(1))
                }
            }
            // timestamp
            (MValue::Timestamp(first_val), MValue::Timestamp(last_val)) => {
                if first_val < last_val {
                    Ok(MValue::Int(-1))
                } else if first_val == last_val {
                    Ok(MValue::Int(0))
                } else {
                    Ok(MValue::Int(1))
                }
            }
            // string values
            (MValue::String(first_val), MValue::String(last_val))
            | (MValue::Bytes(first_val), MValue::Bytes(last_val))
            | (MValue::KeyHash(first_val), MValue::KeyHash(last_val))
            | (MValue::Key(first_val), MValue::Key(last_val))
            | (MValue::Signature(first_val), MValue::Signature(last_val))
            | (MValue::ChainId(first_val), MValue::ChainId(last_val)) => {
                match first_val.cmp(&last_val) {
                    Ordering::Less => Ok(MValue::Int(-1)),
                    Ordering::Equal => Ok(MValue::Int(0)),
                    Ordering::Greater => Ok(MValue::Int(1)),
                }
            }
            // addresses
            (MValue::Address(first_val), MValue::Address(last_val)) => {
                let first_addr = first_val.clone();
                let last_addr = last_val.clone();

                match (
                    MValue::Address(first_val.to_string()).get_address_type(),
                    MValue::Address(last_val.to_string()).get_address_type(),
                ) {
                    (Ok(addr_1), Ok(addr_2)) => match (addr_1, addr_2) {
                        (AddressType::ImplicitAccount, AddressType::ImplicitAccount)
                        | (AddressType::Contract, AddressType::Contract) => {
                            match first_addr.cmp(&last_addr) {
                                Ordering::Less => Ok(MValue::Int(-1)),
                                Ordering::Equal => Ok(MValue::Int(0)),
                                Ordering::Greater => Ok(MValue::Int(1)),
                            }
                        }
                        (AddressType::ImplicitAccount, AddressType::Contract) => {
                            Ok(MValue::Int(-1))
                        }
                        (AddressType::Contract, AddressType::ImplicitAccount) => Ok(MValue::Int(1)),
                    },
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            // bool
            (MValue::Bool(first_val), MValue::Bool(last_val)) => {
                if first_val == &false && last_val == &true {
                    Ok(MValue::Int(-1))
                } else if first_val == &true && last_val == &false {
                    Ok(MValue::Int(1))
                } else {
                    Ok(MValue::Int(0))
                }
            }
            // unit
            (MValue::Unit, MValue::Unit) => Ok(MValue::Int(0)),
            // pairs
            (MValue::Pair(left_pair), MValue::Pair(right_pair)) => {
                let (left_pair_left, left_pair_right) = &*left_pair.value;
                let (right_pair_left, right_pair_right) = &*right_pair.value;

                match (
                    compare(left_pair_left, right_pair_left),
                    compare(left_pair_right, right_pair_right),
                ) {
                    (Ok(val_1), Ok(val_2)) => {
                        if let MValue::Int(_val @ 0) = val_1 {
                            Ok(val_1)
                        } else {
                            Ok(val_2)
                        }
                    }
                    (Ok(_), Err(err)) => Err(err),
                    (Err(err), Ok(_)) => Err(err),
                    (Err(err_1), Err(err_2)) => Err(format!("{} / {}", err_1, err_2)),
                }
            }
            // options
            (MValue::Option(first_val), MValue::Option(last_val)) => {
                // unwraps the values in the boxes
                let first_opt = &*first_val.value;
                let last_opt = &*last_val.value;
                // compares the values
                match (first_opt, last_opt) {
                    (None, None) => Ok(MValue::Int(0)),
                    (Some(_), None) => Ok(MValue::Int(1)),
                    (None, Some(_)) => Ok(MValue::Int(-1)),
                    (Some(val_1), Some(val_2)) => compare(&val_1, &val_2),
                }
            }
            // unions
            (MValue::Or(first_val), MValue::Or(last_val)) => {
                let first_or = &*first_val.value;
                let last_or = &*last_val.value;

                match (first_or, last_or) {
                    (Or::Left(_), Or::Right(_)) => Ok(MValue::Int(-1)),
                    (Or::Right(_), Or::Left(_)) => Ok(MValue::Int(1)),
                    (Or::Left(first_left), Or::Left(last_left)) => compare(first_left, last_left),
                    (Or::Right(first_right), Or::Right(last_right)) => {
                        compare(first_right, last_right)
                    }
                }
            }
            // never
            // TODO: implementation of comparison of never may be incorrect
            (MValue::Never, MValue::Never) => Err(String::from("Forbidden comparison of never")),
            _ => Err(String::from("{:?} and {:?} are not comparable")),
        }
    }
    // pattern match the values according to their types
    let new_val = compare(&stack[options.pos].value, &stack[options.pos + 1].value)?;
    // removes the 2 elements being compared from the stack
    let (_, new_stack) = stack.remove_at(options.pos);
    // pushes the new element to the stack
    let new_stack = new_stack.insert_instead(
        vec![StackElement::new(new_val, Instruction::COMPARE)],
        options.pos,
    );
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    // returns the stack
    Ok((stack, stack_snapshots))
}
