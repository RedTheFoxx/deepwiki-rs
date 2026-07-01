pub mod ai_fixer;
pub mod markdown_scanner;
pub mod mermaid_validator;
pub mod processor;
pub mod utils;

pub use processor::{MermaidProcessor, ProcessResult, ProcessorParams};

/// Returns true when a Chromium/Chrome executable can be located on the host.
pub fn chromium_available() -> bool {
    headless_chrome::browser::default_executable().is_ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn chromium_available_does_not_panic() {
        let _ = super::chromium_available();
    }
}
