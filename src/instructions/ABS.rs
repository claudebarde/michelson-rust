use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{nat, MType, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-ABS

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::ABS) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // verifies that the value at options.pos is an int
    let new_val_res: Result<MValue, String> = match stack[options.pos].value {
        MValue::Int(val) => {
            let new_nat = val.abs() as nat;
            Ok(MValue::Nat(new_nat))
        }
        _ => Err(display_error(ErrorCode::InvalidType((
            MType::Int,
            stack[options.pos].value.get_type(),
            Instruction::ABS,
        )))),
    };

    match new_val_res {
        Err(err) => Err(err),
        Ok(new_val) => {
            let new_stack = stack.replace(
                vec![StackElement::new(new_val, Instruction::ABS)],
                options.pos,
            );
            stack_snapshots.push(new_stack.clone());
            Ok((new_stack, stack_snapshots))
        }
    }
}

/**
 * TESTS
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;

    // PASSING
    // Simple test of cast int to nat
    #[test]
    fn abs_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(-5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[options.pos].value, MValue::Nat(5));
            }
        }
    }

    // Casts int to nat inside the stack
    #[test]
    fn abs_success_pos() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Int(-6), Instruction::INIT),
            StackElement::new(MValue::Mutez(7_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                level: 11,
            },
            pos: 1,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Int(5));
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[2].value, MValue::Mutez(7_000_000));
            }
        }
    }

    // Casts positive int to nat
    #[test]
    fn abs_positive_int() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(7_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(5));
            }
        }
    }

    // FAILING
    // empty stack
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 1 for instruction ABS, got 0"
    )]
    fn abs_empty_stack() {
        let initial_stack: Stack = vec![];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 0);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }

    // stack not deep enough
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 2 for instruction ABS, got 1"
    )]
    fn abs_stack_not_deep_enough() {
        let initial_stack: Stack = vec![StackElement::new(
            MValue::Mutez(7_000_000),
            Instruction::INIT,
        )];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                level: 11,
            },
            pos: 1,
        };

        assert!(initial_stack.len() == 1);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }

    // wrong type
    #[test]
    #[should_panic(expected = "Invalid type for `ABS` expected Int, but got Mutez")]
    fn abs_wrong_type() {
        let initial_stack: Stack = vec![StackElement::new(
            MValue::Mutez(7_000_000),
            Instruction::INIT,
        )];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 1);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }
}
