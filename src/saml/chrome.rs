//! Chrome SAML support.
use super::{EncodedSAML, Result, SamlAuthRequest, SamlIdProvider, SamlResponse};
use anyhow::anyhow;
use headless_chrome::protocol::cdp::types::Event;
use headless_chrome::protocol::cdp::Network::{self, Request};
use headless_chrome::{Browser, LaunchOptions, Tab};
use std::path::PathBuf;
use std::sync::{
    mpsc::{channel, Receiver},
    Arc,
};
use url::{form_urlencoded, Url};

#[derive(Debug)]
/// An agent that performs SAML authentication by manipulating Headless Chrome
pub struct ChromeSamlAgent {
    idp: Box<dyn SamlIdProvider>,
    sp_callback_url: String,
    user_data_dir: PathBuf,
}

impl ChromeSamlAgent {
    pub fn new(idp: Box<dyn SamlIdProvider>, callback_url: Url, user_data_dir: PathBuf) -> Self {
        ChromeSamlAgent {
            idp,
            sp_callback_url: callback_url.to_string(),
            user_data_dir,
        }
    }

    /// To acquire SAML assertion from IdP,
    /// the agent will send a SAML request to IdP
    /// with launching a browser tab.
    pub fn saml_request_to_idp(&mut self, saml_req: SamlAuthRequest) -> Result<SamlResponse> {
        let (_browser, tab, receiver) = self.launch_browser_tab()?;
        let url = self.idp.request_url(saml_req).to_string();
        log::debug!("Navigating to: {}", &url);
        tab.navigate_to(&url)?;
        log::debug!("navigated");
        receiver.recv()?
    }

    /// Launch a browser tab,
    /// and set event listener to capture the callback request
    fn launch_browser_tab(
        &mut self,
    ) -> Result<(Browser, Arc<Tab>, Receiver<Result<SamlResponse>>)> {
        let browser = Browser::new(LaunchOptions {
            headless: false,
            user_data_dir: Some(self.user_data_dir.clone()),
            ..Default::default()
        })?;
        let tab = browser.new_tab()?;

        let _ = tab.call_method(Network::Enable {
            max_total_buffer_size: Some(100_000_000),
            max_resource_buffer_size: Some(100_000_000),
            max_post_data_size: Some(100_000_000),
        })?;

        let (sender, receiver) = channel::<Result<SamlResponse>>();
        let sender = Arc::new(sender);

        // add event listener to capture the callback request
        {
            let callback_url = self.sp_callback_url.to_string();
            let sender = sender.clone();
            tab.add_event_listener(Arc::new(move |event: &Event| match event {
                Event::NetworkRequestWillBeSent(e) => {
                    if e.params.request.url == callback_url && e.params.request.method == "POST" {
                        let r = Self::capture_callback_request(&e.params.request);
                        sender.send(r).expect("Failed to send to receiver");
                    }
                }
                _ => {}
            }))?;
            log::debug!("event listener added");
        }
        // add response handler to handle error response
        {
            let sender = sender.clone();
            tab.register_response_handling(
                "document",
                Box::new(move |params, fetch_body| match params.response.status {
                    401 => {
                        log::error!("401: {:?}", params.response.url);
                    }
                    400.. => {
                        log::error!("Received response is error: {:?}", params.response);
                        sender
                            .send(Err(anyhow!(
                                "Response error {}, {}",
                                params.response.status,
                                params.response.url
                            )))
                            .expect("Failed to send to receiver");
                    }
                    _ => {
                        //                        log::info!("{}: {}", params.response.status, params.response.url)
                    }
                }),
            )?;
            log::debug!("response handler added");
        }
        Ok((browser, tab, receiver))
    }

    /// capture the callback request from IdP to SP
    /// and then extract the SAMLResponse.
    /// Note: The name is "response" but it's actually a redirect request from IdP to SP.
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
