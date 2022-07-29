use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-MEM

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    stack.check_depth(options.pos + 2, Instruction::MEM)?;
    let el_to_find = &stack[options.pos + 1].value;
    // second element on the stack must be a set, a map or a bigmap
    let result = match &stack[options.pos].value {
        MValue::Set(set) => {
            // finds if value is in the set
            match set.value.iter().find(|&x| x == el_to_find) {
                None => Ok(false),
                Some(_) => Ok(true),
            }
        }
        MValue::Big_map(map) | MValue::Map(map) => match map.value.get(el_to_find) {
            None => Ok(false),
            Some(_) => Ok(true),
        },
        _ => Err(format!(
            "Invalid type for `MEM` expected set, map or big_map, but got {:?}",
            stack[options.pos].value.get_type()
        )),
    }?;
    // creates the new element to insert
    let new_el = StackElement::new(MValue::Bool(result), Instruction::MEM);
    // drops the 2 elements from the stack
    let (_, new_stack) = stack.remove_at(options.pos);
    let (_, new_stack) = new_stack.remove_at(options.pos);
    // inserts the new value
    let new_stack = new_stack.insert_at(vec![new_el], options.pos);
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    Ok((new_stack, stack_snapshots))
}
