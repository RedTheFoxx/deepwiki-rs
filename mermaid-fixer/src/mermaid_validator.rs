use mermaid_rs::Mermaid;
use std::thread;
use std::time::Duration;

pub struct MermaidValidator {
    #[allow(dead_code)]
    timeout: Duration,
}

impl MermaidValidator {
    pub fn with_config(timeout_seconds: Option<u64>) -> Result<Self, Box<dyn std::error::Error>> {
        if !crate::chromium_available() {
            return Err("Chromium/Chrome executable not found".into());
        }

        Ok(Self {
            timeout: Duration::from_secs(timeout_seconds.unwrap_or(30)),
        })
    }

    /// Kept for API compatibility; each validation already uses an isolated Chrome session.
    pub fn reset_connection(&self) {}

    /// Validate whether mermaid code is valid
    pub fn validate(&self, mermaid_code: &str) -> Result<(), MermaidValidationError> {
        if mermaid_code.trim().is_empty() {
            return Err(MermaidValidationError::EmptyCode);
        }

        let cleaned_code = self.preprocess_code(mermaid_code);
        self.render_with_retry(&cleaned_code, mermaid_code, 3)
    }

    fn render_with_retry(
        &self,
        cleaned_code: &str,
        original_code: &str,
        max_attempts: usize,
    ) -> Result<(), MermaidValidationError> {
        let mut last_error = String::new();

        for attempt in 0..max_attempts {
            let mermaid = match Mermaid::new() {
                Ok(mermaid) => mermaid,
                Err(e) => {
                    last_error = e.to_string();
                    if attempt + 1 < max_attempts {
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    }
                    return Err(MermaidValidationError::RenderError {
                        message: last_error,
                        error_type: MermaidErrorType::Unknown,
                        original_code: original_code.to_string(),
                    });
                }
            };

            match mermaid.render(cleaned_code) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    last_error = e.to_string();
                    if Self::is_connection_closed_error(&last_error) && attempt + 1 < max_attempts {
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    }
                    return Err(MermaidValidationError::RenderError {
                        message: last_error.clone(),
                        error_type: self.classify_error(&last_error),
                        original_code: original_code.to_string(),
                    });
                }
            }
        }

        Err(MermaidValidationError::RenderError {
            message: last_error,
            error_type: MermaidErrorType::Unknown,
            original_code: original_code.to_string(),
        })
    }

    fn is_connection_closed_error(message: &str) -> bool {
        message.contains("underlying connection is closed")
    }

    /// Preprocess mermaid code
    fn preprocess_code(&self, code: &str) -> String {
        code.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("%%"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Classify error type
    fn classify_error(&self, error_message: &str) -> MermaidErrorType {
        let error_lower = error_message.to_lowercase();

        if error_lower.contains("syntax") || error_lower.contains("parse") {
            MermaidErrorType::SyntaxError
        } else if error_lower.contains("node") || error_lower.contains("vertex") {
            MermaidErrorType::NodeError
        } else if error_lower.contains("edge") || error_lower.contains("arrow") || error_lower.contains("link") {
            MermaidErrorType::EdgeError
        } else if error_lower.contains("graph") || error_lower.contains("diagram") {
            MermaidErrorType::GraphStructureError
        } else if error_lower.contains("style") || error_lower.contains("class") {
            MermaidErrorType::StyleError
        } else {
            MermaidErrorType::Unknown
        }
    }
}

#[derive(Debug, Clone)]
pub enum MermaidValidationError {
    EmptyCode,
    RenderError {
        message: String,
        error_type: MermaidErrorType,
        #[allow(dead_code)]
        original_code: String,
    },
}

#[derive(Debug, Clone)]
pub enum MermaidErrorType {
    SyntaxError,
    NodeError,
    EdgeError,
    GraphStructureError,
    StyleError,
    Unknown,
}

impl std::fmt::Display for MermaidValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MermaidValidationError::EmptyCode => {
                write!(f, "Mermaid code is empty")
            }
            MermaidValidationError::RenderError { message, error_type, .. } => {
                write!(f, "Mermaid render error ({:?}): {}", error_type, message)
            }
        }
    }
}

impl std::error::Error for MermaidValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::extract_mermaid_blocks;
    use std::fs;

    #[test]
    fn invalid_then_valid_in_sequence() {
        let validator = MermaidValidator::with_config(Some(30)).expect("validator init");
        let invalid = "grph TB\na-->b";
        let valid = "graph TB\na-->b";

        let err = validator.validate(invalid).expect_err("invalid diagram");
        assert!(
            !err.to_string().contains("underlying connection is closed"),
            "unexpected connection error after invalid diagram: {err}"
        );

        validator.validate(valid).expect("valid diagram after invalid");
    }

    #[test]
    fn architecture_blocks_do_not_cascade_connection_errors() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../docs/en/2.Architecture.md"
        );
        let content = fs::read_to_string(path).expect("architecture doc");
        let blocks = extract_mermaid_blocks(&content);
        assert!(!blocks.is_empty(), "expected mermaid blocks in architecture doc");

        let validator = MermaidValidator::with_config(Some(30)).expect("validator init");

        for (index, (_, _, code)) in blocks.iter().enumerate() {
            let result = validator.validate(code);
            if let Err(err) = &result {
                let msg = err.to_string();
                assert!(
                    !msg.contains("underlying connection is closed"),
                    "block {} cascaded connection error: {msg}",
                    index + 1
                );
            }
        }
    }
}
