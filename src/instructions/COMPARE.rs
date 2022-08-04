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
                            Ok(val_2)
                        } else {
                            Ok(val_1)
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
            _ => Err(format!(
                "{:?} and {:?} are not comparable",
                first_val, last_val
            )),
        }
    }
    // pattern match the values according to their types
    let new_val = compare(&stack[options.pos].value, &stack[options.pos + 1].value)?;
    // removes the 2 elements being compared from the stack
    let (_, new_stack) = stack.remove_at(options.pos);
    // pushes the new element to the stack
    let new_stack = new_stack.replace(
        vec![StackElement::new(new_val, Instruction::COMPARE)],
        options.pos,
    );
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    // returns the stack
    Ok((new_stack, stack_snapshots))
}

/**
 * TESTS
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::{MType, timestamp, OptionValue, PairValue, OrValue, Or};
    use std::time::{SystemTime, UNIX_EPOCH};

    // PASSING
    // COMPARES INTS
    #[test]
    fn compare_int_int() {
        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 0
        let new_stack: Stack = vec![
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(new_stack.len() == 3);

        match run(new_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let new_stack: Stack = vec![
            StackElement::new(MValue::Int(16), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(new_stack.len() == 3);

        match run(new_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // COMPARES NATS
    #[test]
    fn compare_nat_nat() {
        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 0
        let new_stack: Stack = vec![
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(new_stack.len() == 3);

        match run(new_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let new_stack: Stack = vec![
            StackElement::new(MValue::Nat(16), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(new_stack.len() == 3);

        match run(new_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // COMPARES MUTEZ
    #[test]
    fn compare_mutez_mutez() {
        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Mutez(5_000_000), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 0
        let new_stack: Stack = vec![
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
            StackElement::new(MValue::Nat(7), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(new_stack.len() == 3);

        match run(new_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(7));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let new_stack: Stack = vec![
            StackElement::new(MValue::Mutez(50_000_000), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(new_stack.len() == 3);

        match run(new_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // COMPARES TIMESTAMPS
    #[test]
    fn compare_timestamp_timestamp() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as timestamp;
        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Timestamp(now - 100), Instruction::INIT),
            StackElement::new(MValue::Timestamp(now), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 0
        let new_stack: Stack = vec![
            StackElement::new(MValue::Timestamp(now), Instruction::INIT),
            StackElement::new(MValue::Timestamp(now), Instruction::INIT),
            StackElement::new(MValue::Nat(7), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(new_stack.len() == 3);

        match run(new_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(7));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let new_stack: Stack = vec![
            StackElement::new(MValue::Timestamp(now + 100), Instruction::INIT),
            StackElement::new(MValue::Timestamp(now), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(new_stack.len() == 3);

        match run(new_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // COMPARES STRINGS
    #[test]
    fn compare_string_string() {
        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
            StackElement::new(MValue::String(String::from("tezos")), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 0
        let new_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("tezos")), Instruction::INIT),
            StackElement::new(MValue::String(String::from("tezos")), Instruction::INIT),
            StackElement::new(MValue::Nat(7), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(new_stack.len() == 3);

        match run(new_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(7));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let new_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("tezos")), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(new_stack.len() == 3);

        match run(new_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // COMPARES ADDRESSES
    #[test]
    fn compare_address_address() {
        // comparing implicit account and contract addresses
        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Address(String::from("tz1Me1MGhK7taay748h4gPnX2cXvbgL6xsYL")),
                Instruction::INIT,
            ),
            StackElement::new(
                MValue::Address(String::from("KT1DrZokUnBg35YANi5sQxGfyWgDSAJRfJqY")),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Address(String::from("KT1DrZokUnBg35YANi5sQxGfyWgDSAJRfJqY")),
                Instruction::INIT,
            ),
            StackElement::new(
                MValue::Address(String::from("tz1Me1MGhK7taay748h4gPnX2cXvbgL6xsYL")),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // comparing 2 implicit account addresses
        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Address(String::from("tz1Me1MGhK7taay748h4gPnX2cXvbgL6xsYL")),
                Instruction::INIT,
            ),
            StackElement::new(
                MValue::Address(String::from("tz1NhNv9g7rtcjyNsH8Zqu79giY5aTqDDrzB")),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 0
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Address(String::from("tz1Me1MGhK7taay748h4gPnX2cXvbgL6xsYL")),
                Instruction::INIT,
            ),
            StackElement::new(
                MValue::Address(String::from("tz1Me1MGhK7taay748h4gPnX2cXvbgL6xsYL")),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Address(String::from("tz1NhNv9g7rtcjyNsH8Zqu79giY5aTqDDrzB")),
                Instruction::INIT,
            ),
            StackElement::new(
                MValue::Address(String::from("tz1Me1MGhK7taay748h4gPnX2cXvbgL6xsYL")),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // comparing 2 contract addresses
        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Address(String::from("KT1DrZokUnBg35YANi5sQxGfyWgDSAJRfJqY")),
                Instruction::INIT,
            ),
            StackElement::new(
                MValue::Address(String::from("KT1X1LgNkQShpF9nRLYw3Dgdy4qp38MX617z")),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 0
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Address(String::from("KT1X1LgNkQShpF9nRLYw3Dgdy4qp38MX617z")),
                Instruction::INIT,
            ),
            StackElement::new(
                MValue::Address(String::from("KT1X1LgNkQShpF9nRLYw3Dgdy4qp38MX617z")),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Address(String::from("KT1X1LgNkQShpF9nRLYw3Dgdy4qp38MX617z")),
                Instruction::INIT,
            ),
            StackElement::new(
                MValue::Address(String::from("KT1DrZokUnBg35YANi5sQxGfyWgDSAJRfJqY")),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // COMPARES BOOLEANS
    #[test]
    fn compare_bool_bool() {
        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Bool(false), Instruction::INIT),
            StackElement::new(MValue::Bool(true), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 0
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Bool(false), Instruction::INIT),
            StackElement::new(MValue::Bool(false), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Bool(true), Instruction::INIT),
            StackElement::new(MValue::Bool(false), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // COMPARES UNITS
    #[test]
    fn compare_unit_unit() {
        // should output 0
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Unit, Instruction::INIT),
            StackElement::new(MValue::Unit, Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // COMPARES OPTIONS
    #[test]
    fn compare_option_option() {
        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Option(OptionValue { m_type: MType::Int, value: Box::new(None) }), Instruction::INIT),
            StackElement::new(MValue::Option(OptionValue { m_type: MType::Int, value: Box::new(Some(MValue::Int(9))) }), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Option(OptionValue { m_type: MType::Int, value: Box::new(Some(MValue::Int(9))) }), Instruction::INIT),
            StackElement::new(MValue::Option(OptionValue { m_type: MType::Int, value: Box::new(None) }), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 0
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Option(OptionValue { m_type: MType::Int, value: Box::new(None) }), Instruction::INIT),
            StackElement::new(MValue::Option(OptionValue { m_type: MType::Int, value: Box::new(None) }), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 0
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Option(OptionValue { m_type: MType::Int, value: Box::new(Some(MValue::Int(9))) }), Instruction::INIT),
            StackElement::new(MValue::Option(OptionValue { m_type: MType::Int, value: Box::new(Some(MValue::Int(9))) }), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Option(OptionValue { m_type: MType::Int, value: Box::new(Some(MValue::Int(19))) }), Instruction::INIT),
            StackElement::new(MValue::Option(OptionValue { m_type: MType::Int, value: Box::new(Some(MValue::Int(9))) }), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // COMPARES PAIRS
    #[test]
    fn compare_pair_pair() {
        // should output 0
        let initial_stack: Stack = vec![
            StackElement::new(PairValue::new(MValue::Int(9), MValue::String(String::from("tezos"))), Instruction::INIT),
            StackElement::new(PairValue::new(MValue::Int(9), MValue::String(String::from("tezos"))), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(PairValue::new(MValue::Int(8), MValue::String(String::from("tezos"))), Instruction::INIT),
            StackElement::new(PairValue::new(MValue::Int(9), MValue::String(String::from("tezos"))), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let initial_stack: Stack = vec![
            StackElement::new(PairValue::new(MValue::Int(18), MValue::String(String::from("tezos"))), Instruction::INIT),
            StackElement::new(PairValue::new(MValue::Int(9), MValue::String(String::from("tezos"))), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let initial_stack: Stack = vec![
            StackElement::new(PairValue::new(MValue::Int(9), MValue::String(String::from("tezos"))), Instruction::INIT),
            StackElement::new(PairValue::new(MValue::Int(9), MValue::String(String::from("taquito"))), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(PairValue::new(MValue::Int(9), MValue::String(String::from("taquito"))), Instruction::INIT),
            StackElement::new(PairValue::new(MValue::Int(9), MValue::String(String::from("tezos"))), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // COMPARES UNIONS
    #[test]
    fn compare_union_union() {
        // should output 0
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Or(OrValue::new(Or::Left(MValue::Int(3)), (MType::Int, MType::Nat))), Instruction::INIT),
            StackElement::new(MValue::Or(OrValue::new(Or::Left(MValue::Int(3)), (MType::Int, MType::Nat))), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(0));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Or(OrValue::new(Or::Left(MValue::Int(3)), (MType::Int, MType::Nat))), Instruction::INIT),
            StackElement::new(MValue::Or(OrValue::new(Or::Right(MValue::Nat(3)), (MType::Int, MType::Nat))), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Or(OrValue::new(Or::Right(MValue::Nat(3)), (MType::Int, MType::Nat))), Instruction::INIT),
            StackElement::new(MValue::Or(OrValue::new(Or::Left(MValue::Int(3)), (MType::Int, MType::Nat))), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output 1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Or(OrValue::new(Or::Left(MValue::Int(33)), (MType::Int, MType::Nat))), Instruction::INIT),
            StackElement::new(MValue::Or(OrValue::new(Or::Left(MValue::Int(3)), (MType::Int, MType::Nat))), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output -1
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Or(OrValue::new(Or::Left(MValue::Int(3)), (MType::Int, MType::Nat))), Instruction::INIT),
            StackElement::new(MValue::Or(OrValue::new(Or::Left(MValue::Int(33)), (MType::Int, MType::Nat))), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::COMPARE);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // FAILING
    // COMPARES DIFFERENT TYPES
    #[test]
    fn compare_different_types() {
        // should generate an error
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Unit, Instruction::INIT),
            StackElement::new(MValue::Bool(true), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert_eq!(err, String::from("Unit and Bool(true) are not comparable")),
            Ok(_) => assert!(false),
        }

        // should generate an error
        let initial_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("KT1X1LgNkQShpF9nRLYw3Dgdy4qp38MX617z")), Instruction::INIT),
            StackElement::new(MValue::Address(String::from("KT1X1LgNkQShpF9nRLYw3Dgdy4qp38MX617z")), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => 
                assert_eq!(err, String::from("String(\"KT1X1LgNkQShpF9nRLYw3Dgdy4qp38MX617z\") and Address(\"KT1X1LgNkQShpF9nRLYw3Dgdy4qp38MX617z\") are not comparable")),
            Ok(_) => assert!(false),
        }

        // should generate an error
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(MValue::Nat(667), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => 
                assert_eq!(err, String::from("Nat(6) and Int(6) are not comparable")),
            Ok(_) => assert!(false),
        }
    }

    // COMPARES UNCOMPARABLE TYPES
    #[test]
    #[should_panic(expected = "Operation(\"test\") and Operation(\"test2\") are not comparable")]
    fn compare_operation_operation() {
        // TODO: operation value hasn't been implemented yet
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Operation(String::from("test")), Instruction::INIT),
            StackElement::new(MValue::Operation(String::from("test2")), Instruction::INIT),
            StackElement::new(MValue::Nat(667), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }
}
