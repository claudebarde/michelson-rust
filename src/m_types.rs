use crate::instructions::Instruction;
use bs58;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ticket {
    amount: nat,
    value: MValue,
    ticketer: address,
}

impl Ticket {
    pub fn new(amount: nat, value: MValue, ticketer: address) -> Ticket {
        Ticket {
            amount,
            value,
            ticketer,
        }
    }
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
pub type map<K, V> = HashMap<K, V>;
pub type big_map<K, V> = HashMap<K, V>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    Ticket,
    Contract(Box<(MType, MType)>),
    Option(Box<MType>),
    Or(Box<(MType, MType)>),
    Pair(Box<(MType, MType)>),
    List(Box<MType>),
    Set(Box<MType>),
    Map(Box<(MType, MType)>),
    Big_map(Box<(MType, MType)>),
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
            "ticket" => Ok(MType::Ticket),
            _ => Err(String::from(format!("Unknown type '{}'", str))),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            MType::Unit => String::from("unit"),
            MType::Never => String::from("never"),
            MType::Bool => String::from("bool"),
            MType::Int => String::from("int"),
            MType::Nat => String::from("nat"),
            MType::String => String::from("string"),
            MType::ChainId => String::from("chain_id"),
            MType::Bytes => String::from("bytes"),
            MType::Mutez => String::from("mutez"),
            MType::KeyHash => String::from("key_hash"),
            MType::Key => String::from("key"),
            MType::Signature => String::from("signature"),
            MType::Timestamp => String::from("timestamp"),
            MType::Address => String::from("address"),
            MType::Operation => String::from("operation"),
            MType::Ticket => String::from("ticket"),
            MType::Contract(_) => String::from("contract"),
            MType::Option(_) => String::from("option"),
            MType::Or(_) => String::from("or"),
            MType::Pair(_) => String::from("pair"),
            MType::List(_) => String::from("list"),
            MType::Set(_) => String::from("set"),
            MType::Map(_) => String::from("map"),
            MType::Big_map(_) => String::from("big_map"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OptionValue {
    pub m_type: MType,
    pub value: Box<Option<MValue>>,
}

impl OptionValue {
    pub fn new(val: Option<MValue>, m_type: MType) -> OptionValue {
        OptionValue {
            m_type,
            value: Box::new(val),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrValue {
    pub m_type: (MType, MType),
    pub value: Box<Or>,
}

impl OrValue {
    pub fn new(val: Or, types: (MType, MType)) -> OrValue {
        let (left_type, right_type) = types.clone();

        OrValue {
            m_type: types,
            value: {
                // checks the type matches the value
                match &val {
                    Or::Left(left_val) => {
                        if left_val.get_type() == left_type {
                            Box::new(val)
                        } else {
                            panic!("Left value doesn't match its provided type for new OrValue")
                        }
                    }
                    Or::Right(right_val) => {
                        if right_val.get_type() == right_type {
                            Box::new(val)
                        } else {
                            panic!("Right value doesn't match its provided type for new OrValue")
                        }
                    }
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PairValue {
    pub m_type: (MType, MType),
    pub value: Box<(MValue, MValue)>,
}

impl PairValue {
    /// creates a new pair from the provided values
    pub fn new(val1: MValue, val2: MValue) -> PairValue {
        PairValue {
            m_type: (val1.get_type(), val2.get_type()),
            value: Box::new((val1, val2)),
        }
    }

    /// unpairs a pair
    pub fn unpair(&self) -> (MValue, MValue) {
        *self.value.clone()
    }

    /// extracts left field of the pair
    pub fn car(&self) -> MValue {
        self.value.clone().0
    }

    /// extracts right field of the pair
    pub fn cdr(&self) -> MValue {
        self.value.clone().1
    }

    /// checks if a pair is right-combed
    /// if it is, returns Some(pair depth), if it isn't, returns None
    pub fn check_right_comb_depth(&self) -> Option<usize> {
        fn right_combed(pair: &PairValue, acc: usize) -> usize {
            match &*pair.value {
                (_, MValue::Pair(pair)) => right_combed(&pair, acc + 1),
                _ => acc,
            }
        }
        let acc = right_combed(self, 1);

        if acc == 0 {
            return None;
        } else {
            return Some(acc);
        }
    }

    /// unfold right-combed nested pairs
    pub fn unfold(&self, depth: usize) -> Result<PairValue, String> {
        match &self.check_right_comb_depth() {
            None => Err(format!("The pair is not right-combed: {:?}", &self.value)),
            Some(pair_depth) => {
                if *pair_depth < depth {
                    Err(format!(
                        "The pair is not deep enough, expected a depth of {}, but got {}",
                        depth, pair_depth
                    ))
                } else {
                    if depth == 1 {
                        return Ok(self.clone());
                    } else {
                        match &self.value.1 {
                            MValue::Pair(pair) => pair.unfold(depth - 1),
                            _ => Err(String::from("The value in the right field of the pair was expected to be a pair")),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CollectionValue {
    pub m_type: MType,
    pub value: Box<Vec<MValue>>,
}

impl CollectionValue {
    /// checks if all the elements in the list are of the expected type
    pub fn check_elements_type(&self, current_instruction: Instruction) -> Result<(), String> {
        let mut collection_res = self.value.clone().into_iter().map(|val| {
            let el_type = val.get_type();
            if el_type == self.m_type {
                Ok(())
            } else {
                Err(format!(
                    "Expected values of type {} in list for `{:?}`, but got a value of type {}",
                    self.m_type.to_string(),
                    current_instruction,
                    el_type.to_string()
                ))
            }
        });
        // finds the first error and returns it if there is one
        match collection_res.find(|x| x.is_err()) {
            None => Ok(()),
            Some(err) => err,
        }
    }

    /// Prepends an element to a collection value vector
    /// `.cons` should be used with MValue of type list to match Michelson CONS
    pub fn cons(&self, element: MValue) -> CollectionValue {
        let mut val = self.value.clone();
        val.insert(0, element);
        CollectionValue {
            m_type: self.clone().m_type,
            value: val,
        }
    }

    /// Alias for `cons` to use with MValue of type set
    /// `.update` should be used with MValue of type list to match Michelson UPDATE
    pub fn update(&mut self, element: MValue) -> CollectionValue {
        self.cons(element)
    }

    /// Returns the size of the collection
    pub fn size(&self) -> nat {
        self.value.len() as nat
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapValue {
    pub key_type: MType,
    pub value_type: MType,
    pub value: map<MValue, MValue>,
}

impl Hash for MapValue {
    fn hash<H>(&self, _: &mut H)
    where
        H: Hasher,
    {
    }
}

impl MapValue {
    pub fn size(&self) -> usize {
        self.value.keys().len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContractValue {
    address: address,
    parameter: MType,
}

impl ContractValue {
    pub fn new(address: address, parameter: MType) -> ContractValue {
        ContractValue { address, parameter }
    }

    pub fn get_address(&self) -> address {
        self.address.clone()
    }

    pub fn get_param(&self) -> MType {
        self.parameter.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    Contract(ContractValue),
    Ticket(Box<Ticket>),
    Option(OptionValue),
    Or(OrValue),
    Pair(PairValue),
    List(CollectionValue),
    Set(CollectionValue),
    Map(MapValue),
    Big_map(MapValue),
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
            MValue::Ticket(_) => String::from("ticket"),
            MValue::Contract(_) => String::from("contract"),
            MValue::Option(_) => String::from("option"),
            MValue::Or(_) => String::from("or"),
            MValue::Pair(_) => String::from("pair"),
            MValue::List(_) => String::from("list"),
            MValue::Set(_) => String::from("set"),
            MValue::Map(_) => String::from("map"),
            MValue::Big_map(_) => String::from("big_map"),
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
            MValue::Ticket(_) => MType::Ticket,
            MValue::Contract(val) => {
                MType::Contract(Box::new((MType::Address, val.parameter.clone())))
            }
            MValue::Option(val) => MType::Option(Box::new(val.m_type.clone())),
            MValue::Or(val) => MType::Or(Box::new(val.m_type.clone())),
            MValue::Pair(val) => MType::Pair(Box::new(val.m_type.clone())),
            MValue::List(val) => MType::List(Box::new(val.m_type.clone())),
            MValue::Set(val) => MType::Set(Box::new(val.m_type.clone())),
            MValue::Map(val) => {
                MType::Map(Box::new((val.key_type.clone(), val.value_type.clone())))
            }
            MValue::Big_map(val) => {
                MType::Big_map(Box::new((val.key_type.clone(), val.value_type.clone())))
            }
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

    /// safeguard method
    /// creates a new address value and checks that the provided value is a valid address
    pub fn new_address(val: String) -> Option<MValue> {
        let mval = MValue::Address(val);
        if mval.is_account_address() == true || mval.is_contract_address() == true {
            Some(mval)
        } else {
            None
        }
    }

    /// creates a new empty list
    pub fn new_empty_list(t: MType) -> MValue {
        MValue::List(CollectionValue {
            m_type: t,
            value: Box::new(vec![]),
        })
    }

    /// creates a new list with the elements passed as arguments
    pub fn new_list(elements: Vec<MValue>, t: MType) -> MValue {
        // TODO: verify that the elements are all of the same MType
        MValue::List(CollectionValue {
            m_type: t,
            value: Box::new(elements),
        })
    }

    /// creates a new empty set
    pub fn new_empty_set(t: MType) -> MValue {
        MValue::Set(CollectionValue {
            m_type: t,
            value: Box::new(vec![]),
        })
    }

    /// creates a new set with the elements passed as arguments
    pub fn new_set(elements: Vec<MValue>, t: MType) -> MValue {
        // TODO: verify that the element in the set are of the same MType and no duplicate
        MValue::Set(CollectionValue {
            m_type: t,
            value: Box::new(elements),
        })
    }

    /// creates a new empty map
    pub fn new_empty_map(key_type: MType, value_type: MType) -> MValue {
        MValue::Map(MapValue {
            key_type,
            value_type,
            value: HashMap::new(),
        })
    }

    /// creates a new map with the elements passed as arguments
    pub fn new_map(key_type: MType, value_type: MType, elements: Vec<(MValue, MValue)>) -> MValue {
        // TODO: verify that the element in the map are of the same MType
        // TODO: verify that the elements can be map keys
        MValue::Map(MapValue {
            key_type,
            value_type,
            value: elements.into_iter().collect(),
        })
    }

    /// creates a new empty big_map
    pub fn new_empty_big_map(key_type: MType, value_type: MType) -> MValue {
        MValue::Big_map(MapValue {
            key_type,
            value_type,
            value: HashMap::new(),
        })
    }

    /// creates a new map with the elements passed as arguments
    pub fn new_big_map(
        key_type: MType,
        value_type: MType,
        elements: Vec<(MValue, MValue)>,
    ) -> MValue {
        // TODO: verify that the element in the map are of the same MType
        // TODO: verify that the elements can be map keys
        MValue::Big_map(MapValue {
            key_type,
            value_type,
            value: elements.into_iter().collect(),
        })
    }

    /// creates a new string value
    pub fn new_string(val: &str) -> MValue {
        MValue::String(val.to_string())
    }

    /// creates a new bytes value
    pub fn new_bytes(val: &str) -> MValue {
        MValue::Bytes(val.to_string())
    }
}

/**
 * TESTS
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mtype_from_string() {
        // simple type
        match MType::from_string("nat") {
            Ok(res) => assert_eq!(res, MType::Nat),
            Err(_) => assert!(false),
        }

        // pair type
        // match MType::from_string("pair nat int") {
        //     Ok(res) => assert_eq!(res, MType::Pair(Box::new((MType::Nat, MType::Int)))),
        //     Err(_) => assert!(false),
        // }
    }
}
