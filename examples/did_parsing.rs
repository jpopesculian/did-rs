extern crate did;

use did::DecentralizedIdentifer;

fn main() {
    let id = DecentralizedIdentifer::new("method")
        .add_identifier("example123")
        .add_identifier("example456");
    println!("{:#?}", id);
    println!("\n-- encoded --\n");
    let encoded = id.encode();
    println!("{}", encoded);
    println!("\n-- decoded --\n");
    let decoded = DecentralizedIdentifer::decode(&encoded);
    println!("{:#?}", decoded);
}
