use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-NOT

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::NOT) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // pattern matches the different numeric types
    let new_val: MValue = match (stack[options.pos].get_val()) {
        // boolean NOT
        MValue::Bool(val) => Ok(MValue::Bool(!val)),
        // bitwise NOT
        MValue::Nat(val) => Ok(MValue::Int(!val as int)),
        MValue::Int(val) => Ok(MValue::Int(!val)),
        _ => Err(format!(
            "Invalid types for `NOT` expected `bool`, `int` or `nat`, but got `{}`",
            stack[options.pos].value.get_type().to_string()
        )),
    }?;
    // pushes the new element to the stack
    let new_stack = stack.replace(
        vec![StackElement::new(new_val, Instruction::NOT)],
        options.pos,
    );
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    // returns the stack
    Ok((new_stack, stack_snapshots))
}

/*
    TESTS
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;

    // PASSING
    // Tests NOT with 1 bool
    #[test]
    fn not_bool() -> () {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Bool(true), Instruction::INIT),
            StackElement::new(MValue::Int(2), Instruction::INIT),
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
                assert_eq!(stack[0].instruction, Instruction::NOT);
                assert_eq!(stack[1].value, MValue::Int(2));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // Tests NOT with 1 nat
    #[test]
    fn not_nat() -> () {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(3), Instruction::INIT),
            StackElement::new(MValue::Nat(44), Instruction::INIT),
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
                assert_eq!(stack[0].value, MValue::Int(-4));
                assert_eq!(stack[0].instruction, Instruction::NOT);
                assert_eq!(stack[1].value, MValue::Nat(44));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }

        // example from the Michelson reference
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(0), Instruction::INIT),
            StackElement::new(MValue::Nat(44), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Int(-1));
                assert_eq!(stack[0].instruction, Instruction::NOT);
                assert_eq!(stack[1].value, MValue::Nat(44));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // FAILING
    // wrong types
    #[test]
    #[should_panic(
        expected = "Invalid types for `NOT` expected `bool`, `int` or `nat`, but got `string`"
    )]
    fn not_wrong_types() -> () {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("test")), Instruction::INIT),
            StackElement::new(MValue::Nat(3), Instruction::INIT),
            StackElement::new(MValue::Nat(44), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }

    // wrong stack depth
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 1 for instruction NOT, got 0"
    )]
    fn not_wrong_stack_depth() -> () {
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
