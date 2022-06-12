use serde_json::{Value};
use crate::stack::{ Stack };
use crate::instructions::{ RunOptions };

pub fn run(stack: Stack, args: Option<&Vec<Value>>, options: &RunOptions) -> Result<Stack, String> {
    Ok(stack)
}