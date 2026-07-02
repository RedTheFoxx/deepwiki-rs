use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::generator::agent_executor::{AgentExecuteParams, extract};
use crate::generator::context::GeneratorContext;
use crate::types::DirectoryInfo;

/// LLM directory scoring result — path-keyed to avoid index mismatch
#[derive(Debug, Serialize, Deserialize, Clone, Default, JsonSchema)]
#[serde(default)]
pub struct DirectoryScoreResult {
    /// Absolute or relative path matching DirectoryInfo.path
    #[serde(default, alias = "rel_path", alias = "directory", alias = "dir")]
    pub path: String,
    #[serde(default, deserialize_with = "deserialize_f64_lenient")]
    pub score: f64,
    #[serde(default)]
    pub reasoning: String,
}

fn deserialize_f64_lenient<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(n) => Ok(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => Ok(s.parse::<f64>().unwrap_or(0.0)),
        serde_json::Value::Bool(v) => Ok(if v { 1.0 } else { 0.0 }),
        _ => Ok(0.0),
    }
}

/// Directory scoring response containing scores for all scored directories
#[derive(Debug, Serialize, Deserialize, Clone, Default, JsonSchema)]
#[serde(default)]
pub struct DirectoryScoringResponse {
    #[serde(default)]
    pub scores: Vec<DirectoryScoreResult>,
}

/// Directory scorer — uses LLM to score directories by business value
pub struct DirectoryScorer;

impl DirectoryScorer {
    pub fn new() -> Self {
        Self
    }

    /// Score multiple directories with LLM
    pub async fn score_directories(
        &self,
        context: &GeneratorContext,
        directories: &[DirectoryInfo],
    ) -> Result<HashMap<PathBuf, f64>> {
        if directories.is_empty() {
            return Ok(HashMap::new());
        }

        let prompt_sys = "You are a professional code architecture analyst specializing in evaluating the business importance of code directories.".to_string();
        let project_path = &context.config.project_path;
        let prompt_user = self.build_scoring_prompt(directories, project_path);

        let cache_scope = format!(
            "directory_scoring_{}",
            context.config.project_path.to_string_lossy().replace(['/', '\\', ':', '.'], "_")
        );
        let project_name = context
            .config
            .project_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let dir_list: String = directories
            .iter()
            .take(5)
            .map(|d| {
                relative_path_key(d, project_path).unwrap_or_else(|| normalize_path_key(&d.name))
            })
            .collect::<Vec<_>>()
            .join(", ");
        let more = if directories.len() > 5 {
            format!(", +{} more", directories.len() - 5)
        } else {
            String::new()
        };
        let log_tag = format!("dir_score({}): {} dirs ({}{})", project_name, directories.len(), dir_list, more);

        let response: DirectoryScoringResponse = extract(
            context,
            AgentExecuteParams {
                prompt_sys,
                prompt_user,
                cache_scope: cache_scope.to_string(),
                log_tag,
                progress: None,
            },
        )
        .await?;

        // Build path → score map from keyed response (normalized keys)
        let mut score_map: HashMap<String, f64> = HashMap::new();
        for result in &response.scores {
            let normalized_path = normalize_path_key(&result.path);
            if !normalized_path.is_empty() {
                score_map.insert(normalized_path, result.score.clamp(0.0, 1.0));
            }
        }

        // Match directories by normalized relative path, then fallbacks
        let mut scores = HashMap::new();
        let mut missing = 0usize;
        for dir in directories {
            let matched_score = directory_lookup_keys(dir, project_path)
                .iter()
                .find_map(|key| score_map.get(key).copied());

            if let Some(score) = matched_score {
                scores.insert(dir.path.clone(), score);
            } else {
                missing += 1;
                scores.insert(dir.path.clone(), 0.0);
            }
        }
        if missing > 0 {
            eprintln!(
                "⚠️  Warning: {} directories had no matching LLM score (will use 0.0)",
                missing
            );
        }

        Ok(scores)
    }

    fn build_scoring_prompt(&self, directories: &[DirectoryInfo], project_path: &PathBuf) -> String {
        let mut dir_list = String::new();
        for dir in directories {
            let relative_path = relative_path_key(dir, project_path)
                .unwrap_or_else(|| normalize_path_key(&dir.name));
            let file_names: Vec<String> = std::fs::read_dir(&dir.path)
                .ok()
                .map(|entries| {
                    entries
                        .filter_map(|e| e.ok())
                        .filter_map(|e| e.file_name().to_str().map(String::from))
                        .filter(|n| !n.starts_with('.'))
                        .take(20)
                        .collect()
                })
                .unwrap_or_default();

            dir_list.push_str(&format!(
                "- {} (rel_path: {}, {} files, {} subdirs): {:?}\n",
                dir.name,
                relative_path,
                dir.file_count,
                dir.subdirectory_count,
                file_names
            ));
        }

        format!(
            r#"Rate the business importance of each directory for a software project.

Rate based on:
1. Business value - does it contain core business logic, APIs, or data layer?
2. Code concentration - is it a hub with many imports/exports?
3. Infrastructure role - is it a core package, main entry, or config layer?
IMPORTANT: Backend directories (*.py, *.go, *.rs, *.java, *.kt, etc.) should be rated higher than frontend directories (*.ts, *.js, *.tsx, *.vue, *.jsx, etc.) when business value is comparable.

Directories to rate:
{}

Output JSON with a "scores" array, each entry with "path" (use the exact rel_path shown), "score" (0.0-1.0) and "reasoning":
{{"scores": [{{"path": "src", "score": 0.8, "reasoning": "..."}}, {{"path": "cmd", "score": 0.7, "reasoning": "..."}}, ...]}}

IMPORTANT: Output valid JSON only, no markdown fences."#,
            dir_list
        )
    }
}

/// Normalize a path string for comparison: trim, unify separators, strip `./` and leading `/`.
fn normalize_path_key(path: &str) -> String {
    path.trim()
        .replace('\\', "/")
        .trim_start_matches("./")
        .trim_start_matches('/')
        .trim_end_matches('/')
        .to_string()
}

/// Resolve a directory path relative to the project root.
fn relative_path_key(dir: &DirectoryInfo, project_path: &Path) -> Option<String> {
    if let Ok(relative) = dir.path.strip_prefix(project_path) {
        let key = normalize_path_key(&relative.to_string_lossy());
        if !key.is_empty() {
            return Some(key);
        }
    }

    let base = std::fs::canonicalize(project_path).ok()?;
    let dir_abs = std::fs::canonicalize(&dir.path)
        .or_else(|_| std::fs::canonicalize(project_path.join(&dir.path)))
        .ok()?;

    pathdiff::diff_paths(&dir_abs, &base).map(|relative| normalize_path_key(&relative.to_string_lossy()))
}

/// Candidate lookup keys for a directory, ordered from most to least specific.
fn directory_lookup_keys(dir: &DirectoryInfo, project_path: &Path) -> Vec<String> {
    let mut keys = Vec::new();

    if let Some(relative) = relative_path_key(dir, project_path) {
        keys.push(relative);
    }

    keys.push(normalize_path_key(&dir.path.to_string_lossy()));
    keys.push(normalize_path_key(&dir.name));

    keys
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn dir_at(path: &str, name: &str) -> DirectoryInfo {
        DirectoryInfo {
            path: PathBuf::from(path),
            name: name.to_string(),
            file_count: 0,
            subdirectory_count: 0,
            total_size: 0,
            importance_score: 0.0,
        }
    }

    #[test]
    fn matches_windows_dot_prefixed_paths_from_cache() {
        let project = Path::new(".");
        let dirs = vec![
            dir_at(r".\assets", "assets"),
            dir_at(r".\assets\skill-litho", "skill-litho"),
            dir_at(r".\src\generator", "generator"),
        ];
        let mut score_map = HashMap::new();
        for (path, score) in [
            ("assets", 0.1),
            ("assets/skill-litho", 0.1),
            ("src/generator", 0.9),
        ] {
            score_map.insert(normalize_path_key(path), score);
        }

        for dir in &dirs {
            let keys = directory_lookup_keys(dir, project);
            let matched = keys.iter().find_map(|key| score_map.get(key).copied());
            assert!(
                matched.is_some(),
                "no match for {:?}, keys={:?}, map={:?}",
                dir.path,
                keys,
                score_map
            );
        }
    }

    #[test]
    fn normalize_path_key_strips_leading_separator_from_strip_prefix() {
        assert_eq!(normalize_path_key(r"\assets\skill-litho"), "assets/skill-litho");
        assert_eq!(normalize_path_key("/assets/skill-litho"), "assets/skill-litho");
    }

    #[test]
    fn normalize_path_key_unifies_separators_and_prefix() {
        assert_eq!(
            normalize_path_key(r".\assets\skill-litho\scripts"),
            "assets/skill-litho/scripts"
        );
        assert_eq!(normalize_path_key("docs/en/"), "docs/en");
    }

    #[test]
    fn directory_lookup_keys_prefers_relative_path() {
        let project = PathBuf::from(r"C:\proj");
        let dir = dir_at(r"C:\proj\assets\skill-litho", "skill-litho");
        let keys = directory_lookup_keys(&dir, &project);
        assert_eq!(keys[0], "assets/skill-litho");
    }

    #[test]
    fn relative_path_key_handles_absolute_dir_with_dot_project_root() {
        let project = std::env::current_dir().expect("cwd");
        let assets = project.join("assets");
        if !assets.exists() {
            return;
        }

        let dir = dir_at(&assets.to_string_lossy(), "assets");
        let key = relative_path_key(&dir, Path::new(".")).expect("relative key");
        assert_eq!(key, "assets");
    }

    #[test]
    fn score_map_matches_llm_relative_path_against_absolute_dir() {
        let mut score_map = HashMap::new();
        score_map.insert(normalize_path_key("assets/skill-litho"), 0.9);

        let project = PathBuf::from(r"C:\proj");
        let dir = dir_at(r"C:\proj\assets\skill-litho", "skill-litho");
        let matched = directory_lookup_keys(&dir, &project)
            .iter()
            .find_map(|key| score_map.get(key).copied());

        assert_eq!(matched, Some(0.9));
    }
}
