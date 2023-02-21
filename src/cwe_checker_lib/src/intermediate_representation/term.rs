use crate::prelude::*;

mod builder_high_lvl;
mod builder_low_lvl;

/// A term identifier consisting of an ID string (which is required to be unique)
/// and an address to indicate where the term is located.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Tid {
    //https://github.com/fkie-cad/cwe_checker/issues/305
    id: String,//唯一标识
    pub address: String,//地址，不能作为唯一标识，因为转成IR后地址可能相同
}

impl Tid {
    /// Generate a new term identifier with the given ID string
    /// and with unknown address.
    pub fn new<T: ToString>(val: T) -> Tid {
        Tid {
            id: val.to_string(),
            address: "UNKNOWN".to_string(),
        }
    }

    /// Add a suffix to the ID string and return the new `Tid`
    pub fn with_id_suffix(self, suffix: &str) -> Self {
        Tid {
            id: self.id + suffix,
            address: self.address,
        }
    }

    /// Generate the ID of a block starting at the given address.
    ///
    /// Note that the block may not actually exist.
    /// For cases where one assembly instruction generates more than one block,
    /// the returned block ID is the one that would be executed first if a jump to the given address happened.
    pub fn blk_id_at_address(address: &str) -> Tid {
        Tid {
            id: format!("blk_{address}"),
            address: address.to_string(),
        }
    }
}

impl std::fmt::Display for Tid {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.id)
    }
}

/// A term is an object inside a binary with an address and an unique ID (both contained in the `tid`).
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Term<T> {
    /// The term identifier, which also contains the address of the term
    pub tid: Tid,
    /// The object
    pub term: T,
}
