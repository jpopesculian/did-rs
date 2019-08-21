#[macro_use]
extern crate lazy_static;

extern crate base58;
extern crate hex;
extern crate regex;
extern crate serde;
extern crate serde_json;

pub mod did;
pub mod did_document;
pub mod did_url;
mod parsing;
pub mod public_key_item;
mod utils;

pub use crate::did::DecentralizedIdentifer;
pub use crate::did_document::DecentralizedIdentifierDocument;
pub use crate::did_url::DecentralizedIdentiferUrl;
pub use crate::public_key_item::{PublicKeyItem, PublicKeyItemFormat, PublicKeyItemType};
