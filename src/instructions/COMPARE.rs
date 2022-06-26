use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{AddressType, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use std::cmp::Ordering;

// https://tezos.gitlab.io/michelson-reference/#instr-COMPARE

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 2, Instruction::COMPARE) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // creates a separate function to recursively call it for complex types
    fn compare(first_val: MValue, last_val: MValue) -> Result<MValue, String> {
        match (first_val, last_val) {
            // numeric values
            (MValue::Int(first_val), MValue::Int(last_val))
            | (MValue::Nat(first_val), MValue::Nat(last_val))
            | (MValue::Mutez(first_val), MValue::Mutez(last_val))
            | (MValue::Timestamp(first_val), MValue::Nat(Timestamp)) => {
                if first_val < last_val {
                    Ok(MValue::Int(-1))
                } else if first_val == last_val {
                    Ok(MValue::Int(0))
                } else {
                    Ok(MValue::Int(1))
                }
            }
            // string values
            (MValue::String(first_val), MValue::String(last_val))
            | (MValue::Bytes(first_val), MValue::Bytes(last_val))
            | (MValue::KeyHash(first_val), MValue::KeyHash(last_val))
            | (MValue::Key(first_val), MValue::Key(last_val))
            | (MValue::Signature(first_val), MValue::Signature(last_val))
            | (MValue::ChainId(first_val), MValue::ChainId(last_val)) => {
                match first_val.cmp(&last_val) {
                    Ordering::Less => Ok(MValue::Int(-1)),
                    Ordering::Equal => Ok(MValue::Int(0)),
                    Ordering::Greater => Ok(MValue::Int(1)),
                }
            }
            // addresses
            (MValue::Address(first_val), MValue::Address(last_val)) => {
                match ((first_val.get_address_type(), last_val.get_address_type())) {
                    (Ok(addr_1), Ok(addr_2)) => match (addr_1, addr_2) {
                        (AddressType::ImplicitAccount, AddressType::ImplicitAccount)
                        | (AddressType::Contract, AddressType::Contract) => {
                            match addr_1.cmp(&addr_2) {
                                Ordering::Less => Ok(MValue::Int(-1)),
                                Ordering::Equal => Ok(MValue::Int(0)),
                                Ordering::Greater => Ok(MValue::Int(1)),
                            }
                        }
                        (AddressType::ImplicitAccount, AddressType::Contract) => MValue::Int(-1),
                        (AddressType::Contract, AddressType::ImplicitAccount) => MValue::Int(1),
                    },
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            // bool
            (MValue::Bool(first_val), MValue::Bool(last_val)) => {
                if first_val == false {
                    MValue::Int(-1)
                } else {
                    MValue::Int(1)
                }
            }
            (MValue::Unit, MValue::Unit) => MValue::Int(0),
            // pairs
            // options
            // unions
            // unit
            _ => Err(String::from("{:?} and {:?} are not comparable")),
        };
    }
    // pattern match the values according to their types
    let new_val_res = compare(stack[options.pos].value, stack[options.pos + 1].value);
}
