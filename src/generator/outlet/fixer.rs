use crate::generator::context::GeneratorContext;
use anyhow::Result;
use mermaid_fixer::{MermaidProcessor, ProcessorParams};
use std::path::Path;

/// Mermaid diagram fixer integrated in-process via the mermaid-fixer library.
pub struct MermaidFixer;

impl MermaidFixer {
    pub fn chromium_available() -> bool {
        mermaid_fixer::chromium_available()
    }

    pub async fn fix_mermaid_charts(
        context: &GeneratorContext,
        target_dir: &Path,
    ) -> Result<()> {
        println!("🔧 Starting mermaid chart fixing...");

        let llm_config = &context.config.llm;
        let fixer_config = &context.config.mermaid_fixer;

        let model = fixer_config
            .model
            .clone()
            .unwrap_or_else(|| llm_config.model_powerful.clone());

        let temperature = llm_config.temperature.unwrap_or(0.1) as f32;

        let params = ProcessorParams {
            api_key: llm_config.api_key.clone(),
            model,
            base_url: llm_config.openai_compatible_api_base_url(),
            max_tokens: llm_config.max_tokens,
            temperature,
            timeout_seconds: fixer_config.timeout_seconds,
            dry_run: fixer_config.dry_run,
            verbose: fixer_config.verbose,
        };

        if fixer_config.verbose {
            println!(
                "🚀 Fixing mermaid charts in {} (model: {})",
                target_dir.display(),
                params.model
            );
        }

        match MermaidProcessor::new(params).await {
            Ok(processor) => {
                match processor
                    .process_directory(target_dir, fixer_config.dry_run)
                    .await
                {
                    Ok(result) => {
                        println!(
                            "✅ Mermaid chart fixing completed (files: {}, blocks: {}, invalid: {}, fixed: {})",
                            result.total_files,
                            result.total_mermaid_blocks,
                            result.invalid_blocks,
                            result.fixed_blocks
                        );
                    }
                    Err(e) => {
                        let msg = context.config.target_language.msg_mermaid_error();
                        println!("{}", msg.replace("{}", &e.to_string()));
                    }
                }
            }
            Err(e) => {
                let msg = context.config.target_language.msg_mermaid_error();
                println!("{}", msg.replace("{}", &e.to_string()));
            }
        }

        Ok(())
    }

    pub async fn auto_fix_after_output(context: &GeneratorContext) -> Result<()> {
        let output_dir = &context.config.output_path;

        if !context.config.mermaid_fixer.enabled {
            return Ok(());
        }

        if !output_dir.exists() {
            println!("⚠️ Output directory does not exist, skipping mermaid chart fixing");
            return Ok(());
        }

        if !Self::chromium_available() {
            let msg = context
                .config
                .target_language
                .msg_mermaid_chromium_unavailable();
            println!("{}", msg);
            return Ok(());
        }

        Self::fix_mermaid_charts(context, output_dir).await
    }
}
