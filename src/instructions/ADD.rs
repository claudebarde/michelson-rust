use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, timestamp, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(2, Instruction::ADD) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // pattern matches the different numeric types
    let new_val: MValue = match (
        stack[options.pos].get_val(),
        stack[options.pos + 1].get_val(),
    ) {
        (MValue::Int(left), MValue::Int(right)) => MValue::Int(left + right),
        (MValue::Int(left), MValue::Nat(right)) => {
            if MValue::Nat(right).check_nat() {
                MValue::Int(left + right as int)
            } else {
                panic!("{}", display_error(ErrorCode::InvalidNat(right)))
            }
        } // int
        (MValue::Nat(left), MValue::Int(right)) => {
            if MValue::Nat(left).check_nat() {
                MValue::Int(left as int + right)
            } else {
                panic!("{}", display_error(ErrorCode::InvalidNat(left)))
            }
        } // int
        (MValue::Nat(left), MValue::Nat(right)) => {
            if MValue::Nat(left).check_nat() == false {
                panic!("{}", display_error(ErrorCode::InvalidNat(left)))
            } else if MValue::Nat(right).check_nat() == false {
                panic!("{}", display_error(ErrorCode::InvalidNat(right)))
            } else {
                MValue::Nat(left + right)
            }
        } // nat
        (MValue::Timestamp(left), MValue::Int(right)) => {
            MValue::Timestamp((left as int + right) as timestamp)
        } // timestamp
        (MValue::Int(left), MValue::Timestamp(right)) => {
            MValue::Timestamp((left + right as int) as timestamp)
        } // timestamp
        (MValue::Mutez(left), MValue::Mutez(right)) => {
            if MValue::Mutez(left).check_mutez() == false {
                panic!("{}", display_error(ErrorCode::InvalidMutez(left)))
            } else if MValue::Mutez(right).check_mutez() == false {
                panic!("{}", display_error(ErrorCode::InvalidMutez(right)))
            } else {
                MValue::Mutez(left + right)
            }
        } // mutez
        (m_val_left, m_val_right) => panic!(
            "Cannot add together values of type {} and {}",
            m_val_left.to_string(),
            m_val_right.to_string()
        ),
    };
    // updates the stack by removing the 2 elements
    let (_, new_stack) = stack.remove_at(options.pos);
    let (_, new_stack) = new_stack.remove_at(options.pos);
    // pushes the new value to the top of the stack
    let mut stack_head = vec![StackElement::new(new_val, Instruction::ADD)];
    let mut stack_tail = new_stack;
    stack_head.append(&mut stack_tail);
    // updates the stack snapshots
    stack_snapshots.push(stack_head.clone());
    // returns the stack
    Ok((stack_head, stack_snapshots))
}

/*
    TESTS
*/

#[cfg(test)]
mod tests {
    use crate::instructions::{Instruction, RunOptions, RunOptionsContext};
    use crate::m_types::MValue;
    use crate::stack::{Stack, StackElement, StackSnapshots};
    use serde_json::Value;

    // Tests ADD with 2 ints
    #[test]
    fn add_int_int() -> () {
        let args: Option<&Vec<Value>> = None;
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
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
        let (stack, _): (Stack, StackSnapshots) =
            Instruction::ADD.run(args, initial_stack, stack_snapshots, &options);

        assert!(stack.len() == 1);
        assert_eq!(stack[0].value, MValue::Int(11));
    }

    // Tests ADD with 1 int and 1 nat
    #[test]
    fn add_int_nat() -> () {
        let args: Option<&Vec<Value>> = None;
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
        let (stack, _): (Stack, StackSnapshots) =
            Instruction::ADD.run(args, initial_stack, stack_snapshots, &options);

        assert!(stack.len() == 1);
        assert_eq!(stack[0].value, MValue::Int(11));
    }

    // Tests ADD with 2 nats
    #[test]
    fn add_nat_nat() -> () {
        let args: Option<&Vec<Value>> = None;
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
        let (stack, _): (Stack, StackSnapshots) =
            Instruction::ADD.run(args, initial_stack, stack_snapshots, &options);

        assert!(stack.len() == 1);
        assert_eq!(stack[0].value, MValue::Nat(11));
    }
}
