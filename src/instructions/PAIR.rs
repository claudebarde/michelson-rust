use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MValue, PairValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-PAIR

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(2, Instruction::PAIR) {
        Ok(_) => {
            // TODO: make pair work with an argument
            // creates the new pair
            let new_pair: MValue = MValue::Pair(PairValue::new(
                stack[options.pos].value.clone(),
                stack[options.pos + 1].value.clone(),
            ));
            // drops the 2 elements from the stack
            let (_, new_stack) = stack.remove_at(options.pos);
            let (_, new_stack) = new_stack.remove_at(options.pos);
            // pushes the new pair to the stack
            let mut stack_with_pair: Stack = vec![StackElement::new(new_pair, Instruction::PAIR)];
            let mut old_stack = new_stack.clone();
            stack_with_pair.append(&mut old_stack);
            // updates the stack snapshots
            stack_snapshots.push(stack_with_pair.clone());

            Ok((stack_with_pair, stack_snapshots))
        }
        Err(err) => Err(err),
    }
}

/**
 * TESTS
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::{MType, MValue, PairValue};

    // PASSING
    // pairs the 2 elements on the stack
    #[test]
    fn pair_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
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
            Ok((new_stack, _)) => {
                assert!(new_stack.len() == 2);
                assert!(
                    new_stack[0].value
                        == MValue::Pair(PairValue {
                            m_type: (MType::Int, MType::Nat),
                            value: Box::new((MValue::Int(5), MValue::Nat(6)))
                        })
                )
            }
        }
    }

    // FAILING
    // stack is not deep enough
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 2 for instruction PAIR, got 1"
    )]
    fn pair_wrong_stack() {
        let initial_stack: Stack = vec![StackElement::new(MValue::Int(5), Instruction::INIT)];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 1);

        match run(initial_stack, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }
}
