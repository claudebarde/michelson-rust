use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{CollectionValue, MType, MValue};
use crate::stack::{create_stack_element, Stack, StackElement};
use serde_json::Value;

pub fn run(stack: Stack, args: Option<&Vec<Value>>, options: &RunOptions) -> Result<Stack, String> {
    // no need to check the stack, it can be empty
    let new_stack: Stack = match args {
        None => panic!("Arguments for NIL instruction cannot be empty"),
        Some(val) => {
            if val[0].is_object() {
                let new_list: StackElement = match val[0]["prim"].as_str() {
                    None => panic!("Expected string for the list element type"),
                    Some(str) => {
                        let list_type = MType::from_string(str);
                        create_stack_element(
                            MValue::List(CollectionValue {
                                m_type: list_type,
                                value: Box::new(vec![]),
                            }),
                            Instruction::NIL,
                        )
                    }
                };
                let mut new_stack: Stack = vec![new_list];
                let mut old_stack = stack.clone();
                new_stack.append(&mut old_stack);
                new_stack
            } else {
                panic!("Expected a 'serde_json::Value' of type object")
            }
        }
    };

    Ok(new_stack)
}
