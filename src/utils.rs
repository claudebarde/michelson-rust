pub mod pair {
    use crate::m_types::{ MValue };
    use crate::errors::{ ErrorCode, error_code };

    pub fn unpair(val: MValue) -> Result<(MValue, MValue), String> {
        match val {
            MValue::Pair (box_) => Ok(*box_),
            _ => Err(error_code(ErrorCode::WrongType((String::from("pair"), MValue::to_string(&val)))))
        }
    }

    /*pub fn unpair(val: &MValue) -> Result<&(MValue, MValue), String> {
        match val {
            MValue::Pair (box_) => {
                let pair = &**box_;
                Ok(pair)
            },
            _ => Err(error_code(ErrorCode::WrongType((String::from("pair"), MValue::to_string(val)))))
        }
    }*/
}
