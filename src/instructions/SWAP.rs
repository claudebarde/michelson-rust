use crate::instructions::{Instruction, RunOptions};
use crate::stack::{Stack, StackFuncs, StackSnapshots};

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(2, Instruction::SWAP) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };

    let mut new_stack: Stack = stack.clone();
    new_stack.swap(options.pos, options.pos + 1);
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
    use crate::stack::StackElement;
    use crate::m_types::MValue;
    use crate::instructions::RunOptionsContext;

    // PASSING
    #[test]
    fn swap_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("test")), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Ok((new_stack, _)) => {
                assert!(new_stack.len() == 3);
                assert!(new_stack[0].value == MValue::Int(6));
                assert!(new_stack[1].value == MValue::String(String::from("test")));
            }
            Err(_) => assert!(false),
        }
    }
}