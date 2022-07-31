use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-MEM

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    stack.check_depth(options.pos + 2, Instruction::MEM)?;

    let el_to_find = &stack[options.pos].value;
    // second element on the stack must be a set, a map or a bigmap
    let result = match &stack[options.pos + 1].value {
        MValue::Set(set) => {
            // finds if value is in the set
            match set.value.iter().find(|&x| x == el_to_find) {
                None => Ok(false),
                Some(_) => Ok(true),
            }
        }
        MValue::Big_map(map) | MValue::Map(map) => match map.value.get(el_to_find) {
            None => Ok(false),
            Some(_) => Ok(true),
        },
        _ => Err(format!(
            "Invalid type for `MEM` expected set, map or big_map, but got {:?}",
            stack[options.pos].value.get_type()
        )),
    }?;
    // creates the new element to insert
    let new_el = StackElement::new(MValue::Bool(result), Instruction::MEM);
    // drops the 2 elements from the stack
    let (_, new_stack) = stack.remove_at(options.pos);
    let (_, new_stack) = new_stack.remove_at(options.pos);
    // inserts the new value
    let new_stack = new_stack.insert_at(vec![new_el], options.pos);
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    Ok((new_stack, stack_snapshots))
}

/**
 * TESTS
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::{MType, PairValue};

    // PASSING
    #[test]
    fn mem_set_success() {
        // if the element is in the set
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(8), Instruction::INIT),
            StackElement::new(
                MValue::new_set(
                    vec![MValue::Nat(7), MValue::Nat(6), MValue::Nat(8)],
                    MType::Nat,
                ),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                balance: 50_000_000,
                level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 4);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Bool(true));
                assert_eq!(stack[0].instruction, Instruction::MEM);
            }
        }

        // if the element is not in the set
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(9), Instruction::INIT),
            StackElement::new(
                MValue::new_set(
                    vec![MValue::Nat(7), MValue::Nat(6), MValue::Nat(8)],
                    MType::Nat,
                ),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 4);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Bool(false));
                assert_eq!(stack[0].instruction, Instruction::MEM);
            }
        }
    }

    #[test]
    fn mem_map_success() {
        // if the element is in the set
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(8), Instruction::INIT),
            StackElement::new(
                MValue::new_map(
                    MType::Nat,
                    MType::Int,
                    vec![
                        (MValue::Nat(7), MValue::Int(7)),
                        (MValue::Nat(6), MValue::Int(6)),
                        (MValue::Nat(8), MValue::Int(8)),
                    ],
                ),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                balance: 50_000_000,
                level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 4);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Bool(true));
                assert_eq!(stack[0].instruction, Instruction::MEM);
            }
        }

        // if the element is not in the set
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(9), Instruction::INIT),
            StackElement::new(
                MValue::new_map(
                    MType::Nat,
                    MType::Int,
                    vec![
                        (MValue::Nat(7), MValue::Int(7)),
                        (MValue::Nat(6), MValue::Int(6)),
                        (MValue::Nat(8), MValue::Int(8)),
                    ],
                ),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 4);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Bool(false));
                assert_eq!(stack[0].instruction, Instruction::MEM);
            }
        }
    }

    #[test]
    fn mem_set_complex_type() {
        // if the element is in the set
        let initial_stack: Stack = vec![
            StackElement::new(
                PairValue::new(MValue::Nat(7), MValue::Int(11)),
                Instruction::INIT,
            ),
            StackElement::new(
                MValue::new_set(
                    vec![
                        PairValue::new(MValue::Nat(8), MValue::Int(11)),
                        PairValue::new(MValue::Nat(7), MValue::Int(11)),
                        PairValue::new(MValue::Nat(7), MValue::Int(12)),
                    ],
                    MType::Pair(Box::new((MType::Nat, MType::Int))),
                ),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Nat(5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                balance: 50_000_000,
                level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 4);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Bool(true));
                assert_eq!(stack[0].instruction, Instruction::MEM);
            }
        }
    }
}
