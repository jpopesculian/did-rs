use crate::public_key_item::PublicKeyItem;
use serde::{Deserialize, Serialize};

const DID_CONTEXT: &str = "https://www.w3.org/2019/did/v1";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DecentralizedIdentifierDocument {
    #[serde(rename = "@context")]
    context: String, // ItemOrVec<String>
    #[serde(alias = "@id")]
    id: String,
    public_key: Vec<PublicKeyItem>,
}

impl DecentralizedIdentifierDocument {
    pub fn new(id: String) -> Self {
        DecentralizedIdentifierDocument {
            context: DID_CONTEXT.to_owned(),
            id,
            public_key: vec![],
        }
    }

    pub fn add_public_key_item(&mut self, public_key_item: PublicKeyItem) {
        self.public_key.push(public_key_item);
    }
}
