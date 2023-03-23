use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue, OptionValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-SUB_MUTEZ

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    let this_instruction = Instruction::SUB_MUTEZ;
    // checks the stack
    stack.check_depth(options.pos + 2, this_instruction)?;
    // elements on the stack must be two mutez values
    let new_stack_el = match (
        stack[options.pos].get_val(),
        stack[options.pos + 1].get_val(),
    ) {
        (MValue::Mutez(minuend), MValue::Mutez(subtrahend)) => {
            // gets the result of the subtraction
            let result = if minuend < subtrahend {
                None
            } else {
                Some(MValue::Mutez(minuend - subtrahend))
            };
            // creates an optional value
            let new_val = MValue::Option(OptionValue {
                m_type: MType::Mutez,
                value: Box::new(result),
            });
            // creates a new stack element
            Ok(StackElement::new(new_val, this_instruction))
        }
        (MValue::Mutez(_), val) | (val, MValue::Mutez(_)) => {
            Err(display_error(ErrorCode::WrongType((
                String::from("mutez"),
                val.get_type().to_string(),
                this_instruction,
            ))))
        }
        _ => Err(String::from(
            "SUB_MUTEZ instruction requires 2 mutez values on the stack",
        )),
    }?;

    let (_, new_stack) = stack.remove_at(options.pos);
    let new_stack = new_stack.replace(vec![new_stack_el], options.pos);
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    Ok((new_stack, stack_snapshots))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;

    // PASSING
    #[test]
    fn sub_mutez_success_some() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Mutez(15), Instruction::INIT),
            StackElement::new(MValue::Mutez(6), Instruction::INIT),
            StackElement::new(MValue::Int(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Ok((new_stack, _)) => {
                assert!(new_stack.len() == 2);
                assert!(
                    new_stack[0].value
                        == MValue::Option(OptionValue::new(Some(MValue::Mutez(9)), MType::Mutez))
                );
                assert!(new_stack[0].instruction == Instruction::SUB_MUTEZ);
                assert!(new_stack[1].value == MValue::Int(6_000_000));
                assert!(new_stack[1].instruction == Instruction::INIT)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn sub_mutez_success_none() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Mutez(5), Instruction::INIT),
            StackElement::new(MValue::Mutez(16), Instruction::INIT),
            StackElement::new(MValue::Int(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Ok((new_stack, _)) => {
                assert!(new_stack.len() == 2);
                assert!(new_stack[0].value == MValue::Option(OptionValue::new(None, MType::Mutez)));
                assert!(new_stack[0].instruction == Instruction::SUB_MUTEZ);
                assert!(new_stack[1].value == MValue::Int(6_000_000));
                assert!(new_stack[1].instruction == Instruction::INIT)
            }
            Err(_) => assert!(false),
        }
    }

    // FAILING
    #[test]
    fn sub_mutez_fail_1() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Mutez(5), Instruction::INIT),
            StackElement::new(MValue::Nat(16), Instruction::INIT),
            StackElement::new(MValue::Int(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => assert!(
                err == String::from(
                    "Wrong type, expected `mutez` for instruction SUB_MUTEZ, got `nat`"
                )
            ),
        }
    }

    #[test]
    fn sub_mutez_fail_2() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Mutez(16), Instruction::INIT),
            StackElement::new(MValue::Int(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => assert!(
                err == String::from(
                    "Wrong type, expected `mutez` for instruction SUB_MUTEZ, got `int`"
                )
            ),
        }
    }

    #[test]
    fn sub_mutez_fail_3() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(16), Instruction::INIT),
            StackElement::new(MValue::Int(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => assert!(
                err == String::from("SUB_MUTEZ instruction requires 2 mutez values on the stack")
            ),
        }
    }

    #[test]
    fn sub_mutez_fail_4() {
        let initial_stack: Stack = vec![StackElement::new(MValue::Int(5), Instruction::INIT)];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 1);

        match run(initial_stack, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => assert!(
                err == String::from("Unexpected stack length, expected a length of 2 for instruction SUB_MUTEZ, got 1")
            ),
        }
    }
}
