use bs58;

#[derive(Debug, Clone, PartialEq)]
pub enum Or {
    Left(MValue),
    Right(MValue),
}

#[derive(Debug, Clone)]
pub enum OrType<A, B> {
    Left(A),
    Right(B),
}

pub enum AddressType {
    ImplicitAccount,
    Contract,
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

#[derive(Debug, Clone, PartialEq)]
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
    pub fn from_string(str: &str) -> Result<MType, String> {
        match str {
            "unit" => Ok(MType::Unit),
            "never" => Ok(MType::Never),
            "bool" => Ok(MType::Bool),
            "int" => Ok(MType::Int),
            "nat" => Ok(MType::Nat),
            "string" => Ok(MType::String),
            "chain_id" => Ok(MType::ChainId),
            "bytes" => Ok(MType::Bytes),
            "mutez" => Ok(MType::Mutez),
            "key_hash" => Ok(MType::KeyHash),
            "key" => Ok(MType::Key),
            "signature" => Ok(MType::Signature),
            "timestamp" => Ok(MType::Timestamp),
            "address" => Ok(MType::Address),
            "operation" => Ok(MType::Operation),
            _ => Err(String::from(format!("Unknown type '{}'", str))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionValue {
    pub m_type: MType,
    pub value: Box<Option<MValue>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrValue {
    pub m_type: (MType, MType),
    pub value: Box<Or>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PairValue {
    pub m_type: (MType, MType),
    pub value: Box<(MValue, MValue)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CollectionValue {
    pub m_type: MType,
    pub value: Box<Vec<MValue>>,
}

#[derive(Debug, Clone, PartialEq)]
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
    Option(OptionValue),
    Or(OrValue),
    Pair(PairValue),
    List(CollectionValue),
    Set(CollectionValue),
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
            MValue::Option(val) => MType::Option(Box::new(val.m_type.clone())),
            MValue::Or(val) => MType::Or(Box::new(val.m_type.clone())),
            MValue::Pair(val) => MType::Pair(Box::new(val.m_type.clone())),
            MValue::List(val) => MType::List(Box::new(val.m_type.clone())),
            MValue::Set(val) => MType::Set(Box::new(val.m_type.clone())),
        }
    }

    /// Checks if an address is a valid implicit account address
    pub fn is_account_address(&self) -> bool {
        match self {
            MValue::Address(address) => {
                // validates string as base58 string
                match bs58::decode(address).into_vec() {
                    Err(_) => false,
                    Ok(_) => {
                        if address.len() == 36 {
                            let prefix = &address[..3];
                            if prefix == "tz1" || prefix == "tz2" || prefix == "tz3" {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                }
            }
            _ => false,
        }
    }

    /// Checks if an address is a valid contract address
    pub fn is_contract_address(&self) -> bool {
        match self {
            MValue::Address(address) => {
                // validates string as base58 string
                match bs58::decode(address).into_vec() {
                    Err(_) => false,
                    Ok(_) => {
                        if address.len() == 36 {
                            let prefix = &address[..3];
                            if prefix == "KT1" {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                }
            }
            _ => false,
        }
    }

    /// Figures out the type of the address
    pub fn get_address_type(&self) -> Result<AddressType, String> {
        match self {
            MValue::Address(address) => {
                // validates string as base58 string
                match bs58::decode(address).into_vec() {
                    Err(_) => Err(String::from("The value is not a valid Tezos address")),
                    Ok(_) => {
                        if address.len() == 36 {
                            let prefix = &address[..3];
                            if prefix == "KT1" {
                                Ok(AddressType::Contract)
                            } else if prefix == "tz1" || prefix == "tz2" || prefix == "tz3" {
                                Ok(AddressType::ImplicitAccount)
                            } else {
                                Err(String::from("The value is not a valid Tezos address"))
                            }
                        } else {
                            Err(String::from("The value is not a valid Tezos address"))
                        }
                    }
                }
            }
            _ => Err(String::from("The value is not a valid Tezos address")),
        }
    }

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

    /// safeguard method
    /// checks if the types of the pair fields match their values
    pub fn check_pair(&self) -> bool {
        match &self {
            MValue::Pair(pair_val) => {
                let (left_type, right_type) = &pair_val.m_type;
                let (left_val, right_val) = &*pair_val.value;

                &left_val.get_type() == left_type && &right_val.get_type() == right_type
            }
            _ => false,
        }
    }
}
