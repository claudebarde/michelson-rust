#[derive(Debug, Clone)]
pub enum Or {
    Left (MValue),
    Right (MValue)
}

pub enum OrType<A, B> {
    Left (A),
    Right (B)
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
pub type option<T> = Option<T>;
pub type or<A, B> = OrType<A, B>;
pub type pair<A, B> = (A, B);
pub type list<T> = Vec<T>;
pub type set<T> = Vec<T>;

#[derive(Debug, Clone)]
pub enum MValue {
    Unit,
    Never,
    Bool (bool),
    Int (int),
    Nat (nat),
    String (string),
    ChainId (chain_id),
    Bytes (bytes),
    Mutez (mutez),
    KeyHash (key_hash),
    Key (key),
    Signature (signature),
    Timestamp (timestamp),
    Address (address),
    Option (Box<MValue>),
    Or (Box<Or>),
    Pair (Box<(MValue, MValue)>),
    List (Box<MValue>),
    Set (Box<MValue>)
}

impl MValue {
    pub fn to_string(&self) -> String {
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
            MValue::Option (_) => String::from("option"),
            MValue::Or (_) => String::from("or"),
            MValue::Pair (_) => String::from("pair"),
            MValue::List (_) => String::from("list"),
            MValue::Set (_) => String::from("set"),
        }
    }
}