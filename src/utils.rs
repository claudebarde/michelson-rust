pub mod pair {
    use crate::errors::{error_code, ErrorCode};
    use crate::m_types::{MType, MValue};

    pub fn unpair(val: MValue) -> Result<(MValue, MValue), String> {
        match val {
            MValue::Pair(box_) => Ok(*box_.1),
            _ => Err(error_code(ErrorCode::WrongType((
                String::from("pair"),
                MValue::to_string(&val),
            )))),
        }
    }

    pub fn pair(val1: MValue, val2: MValue) -> MValue {
        MValue::Pair(((val1.get_type(), val2.get_type()), Box::new((val1, val2))))
    }
}
