use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-UNPAIR

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    let this_instruction = Instruction::UNPAIR;
    // checks the stack
    stack.check_depth(options.pos + 1, this_instruction)?;
    // unpairs the value
    let unpair_res: Result<(MValue, MValue), String> = match stack[options.pos].value.clone() {
        MValue::Pair(pair) => Ok(pair.unpair()),
        _ => Err(format!(
            "Invalid pair found at UNPAIR instruction: {:?}",
            stack[options.pos].value
        )),
    };
    let (el1, el2) = unpair_res?;
    // creates the new stack elements
    let stack_el1 = StackElement::new(el1, this_instruction);
    let stack_el2 = StackElement::new(el2, this_instruction);
    let els_to_insert = vec![stack_el1, stack_el2];
    let new_stack = stack.clone().replace(els_to_insert, options.pos);
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
    fn unpair_success() {
        let args: Option<&Vec<Value>> = None;
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Pair(PairValue {
                    m_type: (MType::Int, MType::Nat),
                    value: Box::new((MValue::Int(6), MValue::Nat(11))),
                }),
                Instruction::INIT,
            ),
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
            Ok((new_stack, _)) => {
                assert!(new_stack.len() == 4);
                assert!(new_stack[0].value == MValue::Int(6));
                assert!(new_stack[1].value == MValue::Nat(11));
            }
            Err(_) => assert!(false),
        }
    }

    // FAILING
    // stack isn't deep enough
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 1 for instruction UNPAIR, got 0"
    )]
    fn unpair_wrong_stack() {
        let args: Option<&Vec<Value>> = None;
        let initial_stack: Stack = vec![];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 0);

        match run(initial_stack, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }
}
