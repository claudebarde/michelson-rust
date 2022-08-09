use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MValue, Or};
use crate::parser;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-IF_LEFT

/// runs the instruction with the provided stack and options
pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<parser::RunResult, String> {
    // checks the stack
    match stack.check_depth(1, Instruction::IF_LEFT) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // checks the arguments
    let args = match args {
        None => panic!("No argument provided for IF_LEFT instruction"),
        Some(args_) => {
            if args_.len() != 2 {
                panic!(
                    "{:?}",
                    display_error(ErrorCode::UnexpectedArgsNumber((2, args_.len())))
                )
            } else {
                args_
            }
        }
    };
    // unwraps the value
    let (or_element, stack) = stack.remove_at(options.pos);
    // processes the stack element value
    match or_element.value {
        MValue::Or(box_) => {
            // gets the corresponding arguments and m_value
            let (new_args, m_val): (String, MValue) = match *box_.value {
                Or::Left(left_val) => (args[0].to_string(), left_val),
                Or::Right(right_val) => (args[1].to_string(), right_val),
            };
            // Pushes unwrapped value to the stack
            let mut stack_head = vec![StackElement::new(m_val, Instruction::IF_LEFT)];
            let mut stack_tail = stack.clone();
            stack_head.append(&mut stack_tail);
            // updates the stack snapshots
            stack_snapshots.push(stack_head.clone());
            // runs the code inside the argument
            parser::run(new_args.as_str(), stack_head, stack_snapshots)
        }
        _ => Err(display_error(ErrorCode::WrongType((
            String::from("or"),
            stack[options.pos].value.to_string(),
            Instruction::IF_LEFT,
        )))),
    }
}
