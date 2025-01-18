use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::EncodedSAML;

pub struct SamlAuthRequest {
    id: Uuid,
    instant: DateTime<Utc>,
    app_id_uri: String,
}

impl SamlAuthRequest {
    pub fn new(app_id_uri: String) -> Self {
        SamlAuthRequest {
            id: Uuid::new_v4(),
            instant: Utc::now(),
            app_id_uri,
        }
    }

    fn to_xml(&self) -> String {
        format!(
            r#"
            <samlp:AuthnRequest
                AssertionConsumerServiceURL="{redirect_to}"
                ID="id_{id}"
                IssueInstant="{timestamp}"
                ProtocolBinding="urn:oasis:names:tc:SAML:2.0:bindings:HTTP-POST"
                Version="2.0"
                xmlns:samlp="urn:oasis:names:tc:SAML:2.0:protocol"
            >
                <saml:Issuer xmlns:saml="urn:oasis:names:tc:SAML:2.0:assertion">{req_issuer}</saml:Issuer>
                <samlp:NameIDPolicy Format="urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress" />
            </samlp:AuthnRequest>"#,
            redirect_to = self.app_id_uri,
            id = self.id,
            timestamp = self.instant.to_rfc3339(),
            req_issuer = self.app_id_uri
        )
    }

    pub fn to_encoded_saml(&self) -> EncodedSAML {
        EncodedSAML::from_raw_string(self.to_xml())
    }
}

impl ToString for SamlAuthRequest {
    fn to_string(&self) -> String {
        self.to_xml()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saml_auth_request_to_string() {
        //Arrange
        let target = SamlAuthRequest::new("https://example.com".to_string());
        //Act
        let result = target.to_string();
        println!("{}", result);
        //Assert
        assert!(result.contains("<samlp:AuthnRequest"));
    }

    #[test]
    fn test_encoded_saml() {
        //Arrange
        let target = SamlAuthRequest::new("https://example.com".to_string());
        //Act
        let result = target.to_encoded_saml();
        println!("{:?}", result);
        //Assert
        assert!(result.0.contains("jZFRS8MwFIX%2fSsn72qZLXQxtIVs3GEyQTX3w7VKvrtAkNTed4q%2b369iToL5e7jmc75yCwHS90kM42j2%2bD0gh%2bjSdpZIN3ioH1JKyYJBUaNRB3%2b1UFqfKYIAXCMCibV2yjRS1lFLr%2bVKIFV9KIeaLXIu1zHha365Z9ISeWmdLNopHDdGAW0sBbBhPaZbPUj7jNw9cqJSrnMeLTD6f%2f%2b6BqD1hyV6hI2SRJkIfRqeVszQY9Af0p7bBx%2f2uZMcQelJJQu2bbW0MHxSDgS9n48aZ5AzKLmxqgv6dsPcuuMaNko3zDU4FXWNUxUTg%2f9MUXBOz6u98RXIxrork5y7VNw%3d%3d"));
    }
}
