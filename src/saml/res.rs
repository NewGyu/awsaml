use super::EncodedSAML;

#[derive(Debug)]
pub struct SamlResponse {}

impl SamlResponse {
    pub fn from_encoded(encoded: EncodedSAML) -> Self {
        let raw_string = encoded.to_raw_string();
        println!("{:?}", raw_string);
        SamlResponse {}
    }
}
