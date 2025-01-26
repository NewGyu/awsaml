//! Chrome SAML support.
use super::{EncodedSAML, Result, SamlAuthRequest, SamlIdProvider, SamlResponse};
use anyhow::anyhow;
use headless_chrome::protocol::cdp::types::Event;
use headless_chrome::protocol::cdp::Network::{self, Request};
use headless_chrome::{Browser, LaunchOptions, Tab};
use std::sync::{
    mpsc::{channel, Receiver},
    Arc,
};
use url::{form_urlencoded, Url};

/// An agent that performs SAML authentication by manipulating Headless Chrome
pub struct ChromeSamlAgent {
    idp: Box<dyn SamlIdProvider>,
    sp_callback_url: Url,
}

impl ChromeSamlAgent {
    pub fn new(idp: Box<dyn SamlIdProvider>, callback_url: Url) -> Self {
        ChromeSamlAgent {
            idp,
            sp_callback_url: callback_url,
        }
    }

    /// Request SAML request to IdP in order to acquire SAML assertion
    pub fn saml_request_to_idp(&mut self, saml_req: SamlAuthRequest) -> Result<SamlResponse> {
        let (_, tab, receiver) = self.launch_browser_tab()?;
        let url = self.idp.request_url(saml_req).to_string();
        tab.navigate_to(&url)?;
        receiver.recv()?
    }

    /// Launch a browser tab,
    /// and set event listener to capture the callback request
    fn launch_browser_tab(
        &mut self,
    ) -> Result<(Browser, Arc<Tab>, Receiver<Result<SamlResponse>>)> {
        let browser = Browser::new(LaunchOptions {
            headless: false,
            ..Default::default()
        })?;
        let tab = browser.new_tab()?;

        let _ = tab.call_method(Network::Enable {
            max_total_buffer_size: Some(100_000_000),
            max_resource_buffer_size: Some(100_000_000),
            max_post_data_size: Some(100_000_000),
        })?;

        let (sender, receiver) = channel::<Result<SamlResponse>>();
        let callback_url = self.sp_callback_url.to_string();
        tab.add_event_listener(Arc::new(move |event: &Event| {
            if let Event::NetworkRequestWillBeSent(e) = event {
                if e.params.request.url == callback_url && e.params.request.method == "POST" {
                    let r = Self::capture_callback_request(&e.params.request);
                    let _ = sender.send(r);
                }
            }
        }))?;

        Ok((browser, tab, receiver))
    }

    /// capture the callback request from IdP to SP
    /// and then extract the SAMLResponse
    fn capture_callback_request(callback_request: &Request) -> Result<SamlResponse> {
        let post_entries = callback_request
            .post_data_entries
            .clone()
            .ok_or(anyhow!("No post data entries"))?;

        // concatenate all flagmented post data entries into a single string
        let concatinated = post_entries
            .iter()
            .filter_map(|entry| entry.bytes.to_owned())
            .collect::<Vec<String>>()
            .join("");

        let saml_response = form_urlencoded::parse(concatinated.as_bytes())
            .find(|(key, _)| key == "SAMLResponse")
            .map(|(_, value)| value.to_string())
            .ok_or(anyhow!("No SAMLResponse found"))?;

        Ok(SamlResponse::from_encoded(EncodedSAML(saml_response)))
    }
}
