use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use escape_string::escape;
use headless_chrome::{Browser, LaunchOptions, Tab};
use unescape::unescape;

/// Error thrown when Mermaid is unable to compile the diagram.
#[derive(Debug)]
pub struct CompileError;

impl Error for CompileError {
    fn description(&self) -> &str {
        "Error occurred while compiling the diagram!"
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompileError")
    }
}

/// Headless Chrome session used to render and validate Mermaid diagrams.
pub struct MermaidRenderer {
    _browser: Browser,
    tab: Arc<Tab>,
    #[cfg_attr(not(windows), allow(dead_code))]
    process_id: Option<u32>,
}

impl MermaidRenderer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let launch_options = LaunchOptions::default_builder()
            .path(Some(
                headless_chrome::browser::default_executable().map_err(|e| e.to_string())?,
            ))
            .build()?;
        let browser = Browser::new(launch_options)?;
        let process_id = browser.get_process_id();

        let mermaid_js = include_str!("../payload/mermaid.min.js");
        let html_payload = include_str!("../payload/index.html");

        let tab = browser.new_tab()?;
        tab.navigate_to(&format!("data:text/html;charset=utf-8,{}", html_payload))?;
        tab.evaluate(mermaid_js, false)?;

        Ok(Self {
            _browser: browser,
            tab,
            process_id,
        })
    }

    pub fn render(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let data = self
            .tab
            .evaluate(&format!("render('{}')", escape(input)), true)?;
        let string = data.value.unwrap_or_default().to_string();
        let slice = unescape(string.trim_matches('"')).unwrap_or_default();

        if slice == "null" {
            return Err(Box::new(CompileError));
        }

        Ok(slice.to_string())
    }
}

impl Drop for MermaidRenderer {
    fn drop(&mut self) {
        #[cfg(windows)]
        if let Some(pid) = self.process_id {
            kill_process_tree(pid);
        }
    }
}

#[cfg(windows)]
fn kill_process_tree(pid: u32) {
    let _ = std::process::Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/T", "/F"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
}
