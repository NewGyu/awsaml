use anyhow::Result;

use headless_chrome::{Browser, LaunchOptions};

pub fn launch_browser() -> Result<()> {
    let browser = Browser::new(LaunchOptions {
        headless: false,
        ..Default::default()
    })?;
    let tab = browser.new_tab()?;
    tab.navigate_to("https://www.rust-lang.org")?;
    tab.wait_until_navigated()?;
    Ok(())
}
