use base58::{FromBase58, ToBase58};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PublicKeyItemFormat {
    Pem,
    Base58,
    Hex,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PublicKeyItemType {
    Ed25519VerificationKey2018,
    RsaVerificationKey2018,
    EcdsaSecp256k1VerificationKey2019,
}

#[derive(Clone, Debug)]
pub struct PublicKeyItem {
    controller: String,
    id: String,
    key_bytes: Vec<u8>,
    key_format: PublicKeyItemFormat,
    key_type: PublicKeyItemType,
}

impl PublicKeyItem {
    pub fn new(
        controller: String,
        id: String,
        key_type: PublicKeyItemType,
        key_bytes: Vec<u8>,
        key_format: PublicKeyItemFormat,
    ) -> Self {
        PublicKeyItem {
            controller,
            id,
            key_bytes,
            key_format,
            key_type,
        }
    }
}

impl Serialize for PublicKeyItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("PublicKey", 4)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("type", &self.key_type)?;
        s.serialize_field("controller", &self.controller)?;
        match &self.key_format {
            PublicKeyItemFormat::Pem => s.serialize_field("publicKeyPem", ""), // @TODO implement PEM serialization
            PublicKeyItemFormat::Base58 => {
                s.serialize_field("publicKeyBase58", &self.key_bytes.to_base58())
            }
            PublicKeyItemFormat::Hex => {
                s.serialize_field("publicKeyHex", &hex::encode(&self.key_bytes))
            }
        }?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for PublicKeyItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "camelCase")]
        enum Field {
            Id,
            Type,
            Controller,
            PublicKeyBase58,
            PublicKeyHex,
            PublicKeyPem,
        }

        struct PublicKeyItemVisitor;

        impl<'de> Visitor<'de> for PublicKeyItemVisitor {
            type Value = PublicKeyItem;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct PublicKeyItem")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut key_type = None;
                let mut controller = None;
                let mut key_bytes = None;
                let mut key_format = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if key_type.is_some() {
                                return Err(de::Error::duplicate_field("key_type"));
                            }
                            key_type = Some(map.next_value()?);
                        }
                        Field::Controller => {
                            if controller.is_some() {
                                return Err(de::Error::duplicate_field("controller"));
                            }
                            controller = Some(map.next_value()?);
                        }
                        Field::PublicKeyBase58 => {
                            if key_bytes.is_some() {
                                return Err(de::Error::duplicate_field("key_bytes"));
                            }
                            let key_bytes_rep: String = map.next_value()?;
                            key_bytes = Some(key_bytes_rep.from_base58().map_err(|e| {
                                de::Error::custom(format!("Could not decode base58: {:?}", e))
                            })?);
                            key_format = Some(PublicKeyItemFormat::Base58);
                        }
                        Field::PublicKeyHex => {
                            if key_bytes.is_some() {
                                return Err(de::Error::duplicate_field("key_bytes"));
                            }
                            let key_bytes_rep: String = map.next_value()?;
                            key_bytes = Some(hex::decode(key_bytes_rep).map_err(|e| {
                                de::Error::custom(format!("Could not decode hex: {:?}", e))
                            })?);
                            key_format = Some(PublicKeyItemFormat::Hex);
                        }
                        Field::PublicKeyPem => {
                            if key_bytes.is_some() {
                                return Err(de::Error::duplicate_field("key_bytes"));
                            }
                            // TODO deserialize value
                            let key_bytes_rep: Vec<u8> = map.next_value()?;
                            key_bytes = Some(key_bytes_rep);
                            key_format = Some(PublicKeyItemFormat::Pem);
                        }
                    }
                }

                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let key_type = key_type.ok_or_else(|| de::Error::missing_field("type"))?;
                let controller =
                    controller.ok_or_else(|| de::Error::missing_field("controller"))?;
                let key_bytes = key_bytes
                    .ok_or_else(|| de::Error::missing_field("publicKey(Base58|Hex|Pem)"))?;
                let key_format = key_format
                    .ok_or_else(|| de::Error::custom("key_format should be autofilled"))?;
                Ok(PublicKeyItem::new(
                    controller, id, key_type, key_bytes, key_format,
                ))
            }
        }

        const FIELDS: &[&str] = &[
            "id",
            "type",
            "controller",
            "publicKeyBase58",
            "publicKeyHex",
            "publicKeyPem",
        ];
        deserializer.deserialize_struct("PublicKeyItem", FIELDS, PublicKeyItemVisitor)
    }
}
