use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, MType, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-INT

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::INT) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // verifies that the value at options.pos is a nat
    let new_val_res: Result<MValue, String> = match stack[options.pos].value {
        MValue::Nat(val) => Ok(MValue::Int(val as int)),
        _ => Err(display_error(ErrorCode::InvalidType((
            vec![MType::Nat],
            stack[options.pos].value.get_type(),
            Instruction::INT,
        )))),
    };

    match new_val_res {
        Err(err) => Err(err),
        Ok(new_val) => {
            let new_stack = stack.replace(
                vec![StackElement::new(new_val, Instruction::INT)],
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
    // Simple test of cast nat to int
    #[test]
    fn int_success() {
        let initial_stack: Stack = vec![
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

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[options.pos].value, MValue::Int(5));
            }
        }
    }

    // Casts nat to int inside the stack
    #[test]
    fn int_success_pos() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(7_000_000), Instruction::INIT),
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
            pos: 1,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Int(5));
                assert_eq!(stack[1].value, MValue::Int(6));
                assert_eq!(stack[1].instruction, Instruction::INT);
                assert_eq!(stack[2].value, MValue::Mutez(7_000_000));
            }
        }
    }

    // FAILING
    // empty stack
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 1 for instruction INT, got 0"
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
                balance: 50_000_000,
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
        expected = "Unexpected stack length, expected a length of 2 for instruction INT, got 1"
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
                balance: 50_000_000,
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
    #[should_panic(expected = "Invalid type for `INT` expected nat, but got mutez")]
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
                balance: 50_000_000,
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
