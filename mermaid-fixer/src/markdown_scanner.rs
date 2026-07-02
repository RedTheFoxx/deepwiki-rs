use std::fs;
use std::path::{Path, PathBuf};

pub struct MarkdownScanner {
    // Optional configuration, e.g. directories to ignore
}

impl MarkdownScanner {
    pub fn new() -> Self {
        Self {}
    }

    /// Scan all markdown files under the given directory
    pub fn scan_directory<P: AsRef<Path>>(&self, dir: P) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut markdown_files = Vec::new();
        self.scan_recursive(dir.as_ref(), &mut markdown_files)?;
        Ok(markdown_files)
    }

    /// Recursively scan a directory
    fn scan_recursive(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        if !dir.is_dir() {
            return Err(format!("Path is not a directory: {}", dir.display()).into());
        }

        let entries = fs::read_dir(dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Skip common directories that should not be scanned
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if self.should_skip_directory(dir_name) {
                        continue;
                    }
                }
                
                // Recursively scan subdirectories
                self.scan_recursive(&path, files)?;
            } else if path.is_file() {
                // Check if this is a markdown file
                if self.is_markdown_file(&path) {
                    files.push(path);
                }
            }
        }

        Ok(())
    }

    /// Check whether the path is a markdown file
    fn is_markdown_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            matches!(extension.to_lowercase().as_str(), "md" | "markdown")
        } else {
            false
        }
    }

    /// Check whether a directory should be skipped
    fn should_skip_directory(&self, dir_name: &str) -> bool {
        matches!(
            dir_name,
            ".git" | ".svn" | ".hg" | 
            "node_modules" | "target" | "build" | "dist" |
            ".vscode" | ".idea" | ".vs" |
            "__pycache__" | ".pytest_cache" |
            "vendor" | "deps"
        )
    }
}
