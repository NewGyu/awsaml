use std::io::{Read, Write};

use anyhow::Result;
mod assertion;
mod entra_id;
mod req;

use assertion::SamlAssertion;
use base64::prelude::{Engine, BASE64_STANDARD};
use flate2::read::DeflateDecoder;
use req::SamlAuthRequest;

// acquire the SAML assertion from the IdP
pub trait SamlIdProvider {
    fn authenticate(&self) -> Result<SamlAssertion>;
}

type Base64EncodedString = String;
type RawString = String;

#[derive(Debug)]
struct EncodedSAML(Base64EncodedString);

impl EncodedSAML {
    fn from_raw_string(raw_string: RawString) -> Self {
        let mut deflater =
            flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
        deflater.write_all(raw_string.as_bytes()).unwrap();
        let deflated = deflater.finish().unwrap();
        let deflated = BASE64_STANDARD.encode(deflated.as_slice());
        let deflated = urlencoding::encode(deflated.as_str());
        EncodedSAML(deflated.to_string())
    }

    fn to_raw_string(&self) -> Result<RawString> {
        let decoded = urlencoding::decode(&self.0)?;
        let decoded = BASE64_STANDARD.decode(decoded.into_owned())?;
        let mut deflater = DeflateDecoder::new(decoded.as_slice());
        let mut xml_string = String::new();
        deflater.read_to_string(&mut xml_string)?;
        Ok(xml_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENCODED_SAML_REQUEST:&str = "jZFRS8MwFIX%2fSsn72qZLXQxtIVs3GEyQTX3w7VKvrtAkNTed4q%2b369iToL5e7jmc75yCwHS90kM42j2%2bD0gh%2bjSdpZIN3ioH1JKyYJBUaNRB3%2b1UFqfKYIAXCMCibV2yjRS1lFLr%2bVKIFV9KIeaLXIu1zHha365Z9ISeWmdLNopHDdGAW0sBbBhPaZbPUj7jNw9cqJSrnMeLTD6f%2f%2b6BqD1hyV6hI2SRJkIfRqeVszQY9Af0p7bBx%2f2uZMcQelJJQu2bbW0MHxSDgS9n48aZ5AzKLmxqgv6dsPcuuMaNko3zDU4FXWNUxUTg%2f9MUXBOz6u98RXIxrork5y7VNw%3d%3d";

    #[test]
    fn test_decode_to_raw_string() {
        //Arrange
        let target = EncodedSAML(ENCODED_SAML_REQUEST.to_string());
        //Act
        let result = target.to_raw_string();
        //Assert
        assert!(result.is_ok());
        assert!(result.unwrap().contains("<samlp:AuthnRequest"));
    }

    #[test]
    fn test_encode_decode() {
        //Arrange
        let raw_string = "just an example".to_string();
        let target = EncodedSAML::from_raw_string(raw_string.to_string());
        print!("{:?}", target);
        //Act
        let result = target.to_raw_string();
        //Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), raw_string);
    }
}
