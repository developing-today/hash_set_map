pub mod lib;
use crate::lib::sha1_hash_map::HsmWithSha1Hasher;
fn main() {
    println!(
        "{:#?}",
        HsmWithSha1Hasher::new()
            .insert_blind("asd".to_string())
            .insert_blind("asd".to_string())
            .insert_blind("dsp".to_string())
            .insert_blind("42".to_string())
            .insert_blind("asd".to_string())
            .insert_blind("asd".to_string())
            .insert_blind("123".to_string())
            .insert_blind("123".to_string())
            .insert_blind("dsp".to_string())
            .insert_blind("dezren3901234".to_string())
            .build()
    );
}
