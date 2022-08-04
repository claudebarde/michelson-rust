use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-EQ

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::LT) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };

    // the element on the stack must be an int
    match stack[options.pos].value {
        MValue::Int(val) => {
            let new_val = {
                if val < 0 {
                    MValue::Bool(true)
                } else {
                    MValue::Bool(false)
                }
            };
            // creates the new stack
            let new_stack = stack.replace(
                vec![StackElement::new(new_val, Instruction::LT)],
                options.pos,
            );
            stack_snapshots.push(new_stack.clone());

            Ok((new_stack, stack_snapshots))
        }
        _ => Err(display_error(ErrorCode::InvalidType((
            vec![MType::Int],
            stack[options.pos].value.get_type(),
            Instruction::LT,
        )))),
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
    #[test]
    fn eq_success() {
        // should output true
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(-22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Bool(true));
                assert_eq!(stack[0].instruction, Instruction::LT);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // should output false
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(3), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Bool(false));
                assert_eq!(stack[0].instruction, Instruction::LT);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // FAILING
    // empty stack
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 1 for instruction LT, got 0"
    )]
    fn eq_empty_stack() {
        let initial_stack: Stack = vec![];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 0);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }

    // invalid type on the stack
    #[test]
    #[should_panic(expected = "Invalid type for `LT` expected int, but got nat")]
    fn eq_invalid_type() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(0), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }
}
