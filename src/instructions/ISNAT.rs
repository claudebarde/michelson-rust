use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{nat, MType, MValue, OptionValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-ISNAT

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    stack.check_depth(options.pos + 1, Instruction::ISNAT)?;
    // value on the stack must be an int
    match stack[options.pos].value {
        MValue::Int(val) => {
            let new_val = if val < 0 {
                MValue::Option(OptionValue::new(None, MType::Nat))
            } else {
                MValue::Option(OptionValue::new(Some(MValue::Nat(val as nat)), MType::Nat))
            };
            // updates the stack
            let new_stack = stack.replace(
                vec![StackElement::new(new_val, Instruction::ISNAT)],
                options.pos,
            );
            // updates the stack snapshots
            stack_snapshots.push(new_stack.clone());

            Ok((new_stack, stack_snapshots))
        }
        _ => Err(display_error(ErrorCode::InvalidType((
            vec![MType::Int],
            stack[options.pos].value.get_type(),
            Instruction::ISNAT,
        )))),
    }
}

/*
    TESTS
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::OptionValue;

    // PASSING
    // Simple test of casting a positive int to nat
    #[test]
    fn isnat_positive_int_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
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
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(Some(MValue::Nat(5)), MType::Nat))
                );
                assert_eq!(stack[0].instruction, Instruction::ISNAT);
                assert_eq!(stack[1].value, MValue::Int(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // Simple test of casting a negative int to nat
    #[test]
    fn isnat_negative_int_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(-5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
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
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(None, MType::Nat))
                );
                assert_eq!(stack[0].instruction, Instruction::ISNAT);
                assert_eq!(stack[1].value, MValue::Int(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // FAILING
    // wrong type on the stack
    #[test]
    fn isnat_wrong_stack() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::new_string("test"), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert_eq!(
                err,
                String::from("Invalid type for `ISNAT` expected int, but got string")
            ),
            Ok(_) => assert!(false),
        }
    }
}
