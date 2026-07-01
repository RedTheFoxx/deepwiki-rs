use std::path::{Path, PathBuf};

use crate::ai_fixer::AiFixer;
use crate::markdown_scanner::MarkdownScanner;
use crate::mermaid_validator::MermaidValidator;
use crate::utils::extract_mermaid_blocks;

#[derive(Debug, Clone)]
pub struct ProcessorParams {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub timeout_seconds: u64,
    pub dry_run: bool,
    pub verbose: bool,
}

pub struct MermaidProcessor {
    scanner: MarkdownScanner,
    validator: MermaidValidator,
    ai_fixer: Option<AiFixer>,
    verbose: bool,
}

pub struct ProcessResult {
    pub total_files: usize,
    pub total_mermaid_blocks: usize,
    pub invalid_blocks: usize,
    pub fixed_blocks: usize,
}

impl MermaidProcessor {
    pub async fn new(params: ProcessorParams) -> Result<Self, Box<dyn std::error::Error>> {
        let scanner = MarkdownScanner::new();
        let validator = MermaidValidator::with_config(Some(params.timeout_seconds))?;

        let ai_fixer = if params.dry_run {
            None
        } else {
            Some(AiFixer::new(&params).await?)
        };

        Ok(Self {
            scanner,
            validator,
            ai_fixer,
            verbose: params.verbose,
        })
    }

    pub async fn process_directory(
        &self,
        directory: &Path,
        dry_run: bool,
    ) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        if self.verbose {
            println!("🚀 Scanning directory: {}", directory.display());
        }

        let markdown_files = self.scanner.scan_directory(directory)?;

        if self.verbose {
            println!("📄 Found {} markdown files", markdown_files.len());
        }

        let mut total_mermaid_blocks = 0;
        let mut invalid_blocks = 0;
        let mut fixed_blocks = 0;

        for file_path in &markdown_files {
            if self.verbose {
                println!("\n📝 Processing file: {}", file_path.display());
            }

            let result = self.process_file(file_path, dry_run).await?;

            total_mermaid_blocks += result.total_blocks;
            invalid_blocks += result.invalid_blocks;
            fixed_blocks += result.fixed_blocks;
        }

        Ok(ProcessResult {
            total_files: markdown_files.len(),
            total_mermaid_blocks,
            invalid_blocks,
            fixed_blocks,
        })
    }

    async fn process_file(
        &self,
        file_path: &PathBuf,
        dry_run: bool,
    ) -> Result<FileProcessResult, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        let mermaid_blocks = extract_mermaid_blocks(&content);

        if mermaid_blocks.is_empty() {
            if self.verbose {
                println!("   ℹ️  No mermaid code blocks found");
            }
            return Ok(FileProcessResult::default());
        }

        if self.verbose {
            println!("   🔍 Found {} mermaid code blocks", mermaid_blocks.len());
        }

        let mut invalid_blocks = 0;
        let mut fixed_blocks = 0;
        let mut file_modified = false;
        let mut new_content = content.clone();

        for (index, (_start_pos, _end_pos, mermaid_code)) in mermaid_blocks.iter().enumerate() {
            if self.verbose {
                println!(
                    "      📊 Validating block {}/{}",
                    index + 1,
                    mermaid_blocks.len()
                );
            }

            match self.validator.validate(mermaid_code) {
                Ok(_) => {
                    if self.verbose {
                        println!("         ✅ Block is valid");
                    }
                }
                Err(e) => {
                    if self.verbose {
                        println!("         ❌ Block is invalid: {}", e);
                    }
                    invalid_blocks += 1;

                    if !dry_run {
                        if let Some(ai_fixer) = &self.ai_fixer {
                            match ai_fixer.fix_mermaid(mermaid_code).await {
                                Ok(fixed_code) => match self.validator.validate(&fixed_code) {
                                    Ok(_) => {
                                        if self.verbose {
                                            println!("         🔧 Fix succeeded");
                                        }
                                        new_content = new_content.replace(mermaid_code, &fixed_code);
                                        file_modified = true;
                                        fixed_blocks += 1;
                                    }
                                    Err(validation_error) => {
                                        if self.verbose {
                                            println!(
                                                "         ⚠️  Fixed code is still invalid: {}",
                                                validation_error
                                            );
                                        }
                                    }
                                },
                                Err(fix_error) => {
                                    if self.verbose {
                                        println!("         ⚠️  AI fix failed: {}", fix_error);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if file_modified {
            std::fs::write(file_path, new_content)?;
            if self.verbose {
                println!("   💾 File updated");
            }
        }

        Ok(FileProcessResult {
            total_blocks: mermaid_blocks.len(),
            invalid_blocks,
            fixed_blocks,
        })
    }
}

#[derive(Default)]
struct FileProcessResult {
    total_blocks: usize,
    invalid_blocks: usize,
    fixed_blocks: usize,
}
