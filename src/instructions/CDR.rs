use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-CDR

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    let this_instruction = Instruction::CDR;
    // checks the stack
    stack.check_depth(options.pos + 1, this_instruction)?;
    // checks if the stack has a pair on it
    let new_val: MValue = match stack[options.pos].get_val() {
        MValue::Pair(pair) => {
            // extracts the left field of the pair
            Ok(pair.cdr())
        }
        val => Err(display_error(ErrorCode::WrongType((
            String::from("pair"),
            val.get_type().to_string(),
            this_instruction,
        )))),
    }?;

    let new_el = StackElement::new(new_val, this_instruction);
    let new_stack = stack.replace(vec![new_el], options.pos);
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    Ok((new_stack, stack_snapshots))
}

/*
    TESTS
*/

#[cfg(test)]
mod test {
    use super::*;
    use crate::{instructions::RunOptionsContext, m_types::PairValue};

    // PASSING TESTS
    #[test]
    fn cdr_success() {
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Pair(PairValue::new(MValue::Nat(7), MValue::Mutez(5_000_000))),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Int(4), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        let initial_stack_len = initial_stack.len();

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == initial_stack_len); // initial stack and new stack must have the same lengths
                assert_eq!(stack[0].value, MValue::Mutez(5_000_000));
                assert_eq!(stack[0].instruction, Instruction::CDR);
                assert_eq!(stack[1].value, MValue::Int(4));
                assert_eq!(stack[2].value, MValue::String(String::from("taquito")));
            }
        }
    }

    // FAILING TESTS
    #[test]
    fn cdr_empty_stack() {
        let initial_stack: Stack = vec![];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert_eq!(
                err,
                String::from(
                    "Unexpected stack length, expected a length of 1 for instruction CDR, got 0"
                )
            ),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn cdr_wrong_type() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(4), Instruction::INIT),
            StackElement::new(MValue::String(String::from("taquito")), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert_eq!(
                err,
                String::from("Wrong type, expected `pair` for instruction CDR, got `int`")
            ),
            Ok(_) => assert!(false),
        }
    }
}
