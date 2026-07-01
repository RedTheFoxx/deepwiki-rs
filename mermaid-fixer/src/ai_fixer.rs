use serde::{Deserialize, Serialize};

use crate::processor::ProcessorParams;

pub struct AiFixer {
    api_key: String,
    model: String,
    prompt_template: String,
    base_url: String,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct FixResponse {
    fixed_code: String,
    explanation: String,
    changes: Option<Vec<ChangeDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChangeDetail {
    #[serde(rename = "type")]
    change_type: String,
    original: String,
    fixed: String,
    reason: String,
}

impl AiFixer {
    pub async fn new(params: &ProcessorParams) -> Result<Self, Box<dyn std::error::Error>> {
        if params.api_key.is_empty() {
            return Err("LLM API key is required for mermaid fixing".into());
        }

        let prompt_template = include_str!("prompt.tpl").to_owned();

        Ok(Self {
            api_key: params.api_key.clone(),
            model: params.model.clone(),
            prompt_template,
            base_url: params.base_url.clone(),
            max_tokens: params.max_tokens,
            temperature: params.temperature,
        })
    }

    pub async fn fix_mermaid(
        &self,
        mermaid_code: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = self.build_prompt(mermaid_code);

        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            max_tokens: Some(self.max_tokens),
            temperature: Some(self.temperature),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("LLM API request failed: {}", error_text).into());
        }

        let openai_response: OpenAIResponse = response.json().await?;

        if openai_response.choices.is_empty() {
            return Err("LLM API returned an empty response".into());
        }

        let content = &openai_response.choices[0].message.content;
        self.extract_fixed_code(content)
    }

    fn build_prompt(&self, mermaid_code: &str) -> String {
        self.prompt_template
            .replace("{{MERMAID_CODE}}", mermaid_code)
    }

    fn extract_fixed_code(&self, response: &str) -> Result<String, Box<dyn std::error::Error>> {
        let cleaned_response = self.clean_response(response);

        if let Ok(fix_response) = serde_json::from_str::<FixResponse>(&cleaned_response) {
            println!("         📋 Fix explanation: {}", fix_response.explanation);
            if let Some(changes) = &fix_response.changes {
                for (i, change) in changes.iter().enumerate() {
                    println!(
                        "         🔧 Change {}: {} -> {}",
                        i + 1,
                        change.change_type,
                        change.reason
                    );
                }
            }
            return Ok(fix_response.fixed_code);
        }

        if let Some(code) = self.extract_code_block(response) {
            return Ok(code);
        }

        Ok(response.trim().to_string())
    }

    fn clean_response(&self, response: &str) -> String {
        let response = response.trim();

        if response.starts_with("```json") && response.ends_with("```") {
            let lines: Vec<&str> = response.lines().collect();
            if lines.len() > 2 {
                return lines[1..lines.len() - 1].join("\n");
            }
        }

        if response.starts_with("```") && response.ends_with("```") {
            let lines: Vec<&str> = response.lines().collect();
            if lines.len() > 2 {
                return lines[1..lines.len() - 1].join("\n");
            }
        }

        response.to_string()
    }

    fn extract_code_block(&self, response: &str) -> Option<String> {
        let lines: Vec<&str> = response.lines().collect();
        let mut in_code_block = false;
        let mut code_lines = Vec::new();

        for line in lines {
            if line.trim().starts_with("```mermaid") {
                in_code_block = true;
                continue;
            }

            if line.trim() == "```" && in_code_block {
                break;
            }

            if in_code_block {
                code_lines.push(line);
            }
        }

        if !code_lines.is_empty() {
            Some(code_lines.join("\n"))
        } else {
            None
        }
    }
}
