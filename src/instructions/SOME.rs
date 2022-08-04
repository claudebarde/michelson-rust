use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MValue, OptionValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-SOME

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::SOME) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // wraps the value at options.pos
    let val_to_wrap = stack[options.pos].value.clone();
    let new_val: MValue = MValue::Option(OptionValue {
        m_type: val_to_wrap.get_type(),
        value: Box::new(Option::Some(val_to_wrap)),
    });
    // updates the stack
    let new_stack = stack.replace(
        vec![StackElement::new(new_val, Instruction::SOME)],
        options.pos,
    );
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
    use crate::m_types::MType;

    // PASSING
    #[test]
    fn some_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
                    MValue::Option(OptionValue {
                        m_type: MType::Int,
                        value: Box::new(Option::Some(MValue::Int(5)))
                    })
                );
                assert_eq!(stack[0].instruction, Instruction::SOME);
                assert_eq!(stack[1].value, MValue::Nat(6));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // FAILING
    // empty stack
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 1 for instruction SOME, got 0"
    )]
    fn abs_empty_stack() {
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
}
