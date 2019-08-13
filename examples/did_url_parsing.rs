extern crate did;

use did::{DecentralizedIdentifer, DecentralizedIdentiferUrl};

fn main() {
    let id = DecentralizedIdentifer::new("method")
        .add_identifier("example123")
        .add_identifier("example456");
    let did = id.encode();
    let url = format!("{};cool;param:name=1;done=now?cool=1&what#a-fragment", did);
    println!("{}", url);
    println!("{:#?}", DecentralizedIdentiferUrl::decode(&url));
}
