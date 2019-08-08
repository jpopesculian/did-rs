extern crate did;

use did::{DecentralizedIdentifierDocument, PublicKeyItem, PublicKeyItemFormat, PublicKeyItemType};

fn main() {
    let controller = "did:example:123";
    let pub_key_item = PublicKeyItem::new(
        controller.to_owned(),
        format!("{}#key-1", controller),
        PublicKeyItemType::RsaVerificationKey2018,
        vec![0, 255, 0],
        PublicKeyItemFormat::Base58,
    );
    let mut dido = DecentralizedIdentifierDocument::new(controller.to_owned());
    dido.add_public_key_item(pub_key_item);

    let serialized = serde_json::to_string_pretty(&dido).unwrap();
    let deserialized: DecentralizedIdentifierDocument = serde_json::from_str(&serialized).unwrap();
    println!("{:#?}", dido);
    println!("\n-- serialized --\n");
    println!("{}", serialized);
    println!("\n-- deserialized --\n");
    println!("{:#?}", deserialized);
}
