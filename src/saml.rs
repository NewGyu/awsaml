mod chrome;
pub mod entra_id;
mod req;
mod res;

use anyhow::Result;
use base64::prelude::{Engine, BASE64_STANDARD};
pub use chrome::ChromeSamlAgent;
use flate2::read::DeflateDecoder;
pub use req::SamlAuthRequest;
pub use res::SamlResponse;
use std::{
    fmt::Debug,
    io::{Read, Write},
};
use url::Url;

// acquire the SAML assertion from the IdP
pub trait SamlIdProvider: Debug {
    fn request_url(&self, saml_request: SamlAuthRequest) -> Url {
        let mut url = self.request_base();
        url.set_query(Some(&format!(
            "SAMLRequest={}",
            saml_request.to_encoded_saml().to_string()
        )));
        url
    }

    fn request_base(&self) -> Url;
}

type Base64EncodedXMLString = String;
type RawXMLString = String;

#[derive(Debug)]
pub struct EncodedSAML(Base64EncodedXMLString);

impl EncodedSAML {
    /// Construct an instance from a raw XML string
    fn from_raw_string(raw_string: RawXMLString) -> Self {
        let deflated = Self::deflate(raw_string);
        let encoded = BASE64_STANDARD.encode(&deflated);
        let encoded = urlencoding::encode(encoded.as_str());
        EncodedSAML(encoded.to_string())
    }

    /// Convert the instance to a raw XML string
    fn to_raw_string(&self) -> Result<RawXMLString> {
        let decoded = urlencoding::decode(&self.0)?;
        let bytes = BASE64_STANDARD.decode(decoded.into_owned())?;
        let raw_xml = Self::inflate(bytes);
        Ok(raw_xml)
    }

    fn deflate(str: String) -> Vec<u8> {
        let mut deflater =
            flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
        deflater.write_all(str.as_bytes()).unwrap();
        deflater.finish().unwrap()
    }

    fn inflate(bytes: Vec<u8>) -> String {
        let mut inflater = DeflateDecoder::new(bytes.as_slice());
        let mut str = String::new();
        inflater.read_to_string(&mut str).unwrap();
        str
    }
}

impl ToString for EncodedSAML {
    fn to_string(&self) -> String {
        self.0.clone()
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
        let result = target.to_raw_string().unwrap();
        //Assert
        assert!(result.starts_with("<samlp:AuthnRequest"));
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
