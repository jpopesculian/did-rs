#[macro_use]
extern crate lazy_static;

extern crate base58;
extern crate hex;
extern crate regex;
extern crate serde;
extern crate serde_json;

pub mod did;
pub mod document;
pub mod public_key_item;

pub use did::DecentralizedIdentifer;
pub use document::DecentralizedIdentifierDocument;
pub use public_key_item::{PublicKeyItem, PublicKeyItemFormat, PublicKeyItemType};
