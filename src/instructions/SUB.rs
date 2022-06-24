use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, timestamp, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(2, Instruction::SUB) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };

    let new_val_res: Result<MValue, String> = match (
        stack[options.pos].get_val(),
        stack[options.pos + 1].get_val(),
    ) {
        (MValue::Int(left), MValue::Int(right)) => Ok(MValue::Int(left - right)),
        (MValue::Int(left), MValue::Nat(right)) => Ok(MValue::Int(left - (right as int))),
        (MValue::Nat(left), MValue::Int(right)) => Ok(MValue::Int((left as int) - right)),
        (MValue::Nat(left), MValue::Nat(right)) => Ok(MValue::Int((left as int) - (right as int))),
        (MValue::Timestamp(left), MValue::Int(right)) => Ok(MValue::Timestamp(left - (right as timestamp))),
        (MValue::Timestamp(left), MValue::Timestamp(right)) => Ok(MValue::Int((left - right) as int)),
        (MValue::Mutez(_), MValue::Mutez(_)) => Err(String::from("Use the SUB_MUTEZ instruction to subtract mutez values")),
        (m_val_left, m_val_right) => Err(
            format!("Cannot subtract values of type {} and {} with the SUB instruction",
            m_val_left.to_string(),
            m_val_right.to_string())
        ),
    };
    match new_val_res {
        Err(err) => Err(err),
        Ok(new_val) => {
            // updates the stack by removing the 2 elements
            let (_, new_stack) = stack.remove_at(options.pos);
            let (_, new_stack) = new_stack.remove_at(options.pos);
            // pushes the new value to the top of the stack
            let mut stack_head = vec![StackElement::new(new_val, Instruction::SUB)];
            let mut stack_tail = new_stack;
            stack_head.append(&mut stack_tail);
            // updates the stack snapshots
            stack_snapshots.push(stack_head.clone());
            // returns the stack
            Ok((stack_head, stack_snapshots))
        }
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
    // SUB int - int = int
    #[test]
    fn sub_int_int() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(15), Instruction::INIT),
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
                assert!(new_stack.len() == 2);
                assert!(new_stack[0].value == MValue::Int(9));
            }
            Err(_) => assert!(false),
        }
    }

    // SUB int - nat = int
    #[test]
    fn sub_int_nat() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(15), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
                assert!(new_stack.len() == 2);
                assert!(new_stack[0].value == MValue::Int(9));
            }
            Err(_) => assert!(false),
        }
    }

    // SUB nat - nat = int
    #[test]
    fn sub_nat_nat() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
                assert!(new_stack.len() == 2);
                assert!(new_stack[0].value == MValue::Int(-1));
            }
            Err(_) => assert!(false),
        }
    }

    // FAILING
    // SUB string - nat
    #[test]
    #[should_panic(expected = "Cannot subtract values of type string and nat with the SUB instruction")]
    fn sub_string_nat() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("5")), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // SUB mutez - mutez
    #[test]
    #[should_panic(expected = "Use the SUB_MUTEZ instruction to subtract mutez values")]
    fn sub_mutez_mutez() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Mutez(10_000_000), Instruction::INIT),
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

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }
}