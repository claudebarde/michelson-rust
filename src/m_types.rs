#[derive(Debug, Clone)]
pub enum Or {
    Left(MValue),
    Right(MValue),
}

#[derive(Debug, Clone)]
pub enum OrType<A, B> {
    Left(A),
    Right(B),
}

pub enum Never {}

pub type unit = ();
pub type never = Never;
pub type int = i128;
pub type nat = u128;
pub type string = String;
pub type chain_id = String;
pub type bytes = String;
pub type mutez = u128;
pub type key_hash = String;
pub type key = String;
pub type signature = String;
pub type timestamp = usize;
pub type address = String;
pub type operation = String;
pub type option<T> = Option<T>;
pub type or<A, B> = (A, B);
pub type pair<A, B> = (A, B);
pub type list<T> = Vec<T>;
pub type set<T> = Vec<T>;

#[derive(Debug, Clone)]
pub enum MType {
    Unit,
    Never,
    Bool,
    Int,
    Nat,
    String,
    ChainId,
    Bytes,
    Mutez,
    KeyHash,
    Key,
    Signature,
    Timestamp,
    Address,
    Operation,
    Option(Box<MType>),
    Or(Box<(MType, MType)>),
    Pair(Box<(MType, MType)>),
    List(Box<MType>),
    Set(Box<MType>),
}

impl MType {
    pub fn from_string(str: &str) -> MType {
        match str {
            "unit" => MType::Unit,
            "never" => MType::Never,
            "bool" => MType::Bool,
            "int" => MType::Int,
            "nat" => MType::Nat,
            "string" => MType::String,
            "chain_id" => MType::ChainId,
            "bytes" => MType::Bytes,
            "mutez" => MType::Mutez,
            "key_hash" => MType::KeyHash,
            "key" => MType::Key,
            "signature" => MType::Signature,
            "timestamp" => MType::Timestamp,
            "address" => MType::Address,
            "operation" => MType::Operation,
            _ => panic!("Cannot cast string to complex type yet"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MValue {
    Unit,
    Never,
    Bool(bool),
    Int(int),
    Nat(nat),
    String(string),
    ChainId(chain_id),
    Bytes(bytes),
    Mutez(mutez),
    KeyHash(key_hash),
    Key(key),
    Signature(signature),
    Timestamp(timestamp),
    Address(address),
    Operation(operation),
    Option((MType, Box<MValue>)),
    Or(((MType, MType), Box<Or>)),
    Pair(((MType, MType), Box<(MValue, MValue)>)),
    List((MType, Box<Vec<MValue>>)),
    Set((MType, Box<Vec<MValue>>)),
}

impl MValue {
    pub fn to_string(&self) -> String {
        match self {
            MValue::Unit => String::from("unit"),
            MValue::Never => String::from("never"),
            MValue::Bool(_) => String::from("bool"),
            MValue::Int(_) => String::from("int"),
            MValue::Nat(_) => String::from("nat"),
            MValue::String(_) => String::from("string"),
            MValue::ChainId(_) => String::from("chain_id"),
            MValue::Bytes(_) => String::from("bytes"),
            MValue::Mutez(_) => String::from("mutez"),
            MValue::KeyHash(_) => String::from("key_hash"),
            MValue::Key(_) => String::from("key"),
            MValue::Signature(_) => String::from("signature"),
            MValue::Timestamp(_) => String::from("timestamp"),
            MValue::Address(_) => String::from("address"),
            MValue::Operation(_) => String::from("operation"),
            MValue::Option(_) => String::from("option"),
            MValue::Or(_) => String::from("or"),
            MValue::Pair(_) => String::from("pair"),
            MValue::List(_) => String::from("list"),
            MValue::Set(_) => String::from("set"),
        }
    }

    pub fn get_type(&self) -> MType {
        match self {
            MValue::Unit => MType::Unit,
            MValue::Never => MType::Never,
            MValue::Bool(_) => MType::Bool,
            MValue::Int(_) => MType::Int,
            MValue::Nat(_) => MType::Nat,
            MValue::String(_) => MType::String,
            MValue::ChainId(_) => MType::ChainId,
            MValue::Bytes(_) => MType::Bytes,
            MValue::Mutez(_) => MType::Mutez,
            MValue::KeyHash(_) => MType::KeyHash,
            MValue::Key(_) => MType::Key,
            MValue::Signature(_) => MType::Signature,
            MValue::Timestamp(_) => MType::Timestamp,
            MValue::Address(_) => MType::Address,
            MValue::Operation(_) => MType::Operation,
            MValue::Option((type_, _)) => MType::Option(Box::new(type_.clone())),
            MValue::Or((type_, _)) => MType::Or(Box::new(type_.clone())),
            MValue::Pair((type_, _)) => MType::Pair(Box::new(type_.clone())),
            MValue::List((type_, _)) => MType::List(Box::new(type_.clone())),
            MValue::Set((type_, _)) => MType::Set(Box::new(type_.clone())),
        }
    }

    /*fn to_michelson(&self, is_nested: bool) -> MType {
        match self {
            MValue::Unit => String::from("unit"),
            MValue::Never => String::from("never"),
            MValue::Bool (_) => String::from("bool"),
            MValue::Int (_) => String::from("int"),
            MValue::Nat (_) => String::from("nat"),
            MValue::String (_) => String::from("string"),
            MValue::ChainId (_) => String::from("chain_id"),
            MValue::Bytes (_) => String::from("bytes"),
            MValue::Mutez (_) => String::from("mutez"),
            MValue::KeyHash (_) => String::from("key_hash"),
            MValue::Key (_) => String::from("key"),
            MValue::Signature (_) => String::from("signature"),
            MValue::Timestamp (_) => String::from("timestamp"),
            MValue::Address (_) => String::from("address"),
            MValue::Operation (_) => String::from("operation"),
            // complex types
            MValue::Option (val) => {
                if is_nested {
                    String::from(format!("option ({:?})", val.to_michelson(true)))
                } else {
                    String::from(format!("option {:?}", val.to_michelson(true)))
                }
            },
            MValue::Or (val) => {
                if is_nested {
                    String::from(format!("option ({:?})", val.to_michelson(true)))
                } else {
                    String::from(format!("option {:?}", val.to_michelson(true)))
                }
            },
            MValue::Pair (_) => String::from("pair"),
            MValue::List (_) => String::from("list"),
            MValue::Set (_) => String::from("set"),
        }
    }*/

    /*pub fn from_string(poss_type: String) -> MValue {
        match poss_type.as_str() {
            "unit" => MValue::Unit,
            "never" => MValue::Never,
            "" => MValue::Bool (_),
            "" => MValue::Int (_),
            "" => MValue::Nat (_),
            "" => MValue::String (_),
            "" => MValue::ChainId (_),
            "" => MValue::Bytes (_),
            "" => MValue::Mutez (_),
            "" => MValue::KeyHash (_),
            "" => MValue::Key (_),
            "" => MValue::Signature (_),
            "" => MValue::Timestamp (_),
            "" => MValue::Address (_),
            "" => MValue::Option (_),
            "" => MValue::Or (_),
            "" => MValue::Pair (_),
            "" => MValue::List (_),
            "" => MValue::Set (_),
        }
    }*/

    /// safeguard method
    /// checks if value is a valid nat
    pub fn check_nat(&self) -> bool {
        match &self {
            MValue::Nat(m_val) => {
                if m_val >= &0 {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// safeguard method
    /// checks if value is a valid mutez
    pub fn check_mutez(&self) -> bool {
        match &self {
            MValue::Mutez(m_val) => {
                if m_val >= &0 {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
