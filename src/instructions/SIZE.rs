use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-SIZE

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::SIZE) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // SIZE can be used only with string, list, set, map or bytes
    // match &stack[option.pos].value {
    //     &Mvalue::String(val) => ,
    //     &Mvalue::List(val) => ,
    //     &Mvalue::Set(val) => ,
    //     &Mvalue::Map(val) => ,
    //     &Mvalue::Bytes(val) => ,
    //     _ => Err(display_error(ErrorCode::InvalidType(())))
    // }
    Err(String::from("test"))
}
