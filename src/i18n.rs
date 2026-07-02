use serde::{Deserialize, Serialize};

/// Target language type
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum TargetLanguage {
    #[serde(rename = "zh")]
    Chinese,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "vi")]
    Vietnamese,
}

impl Default for TargetLanguage {
    fn default() -> Self {
        Self::English
    }
}

impl std::fmt::Display for TargetLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetLanguage::Chinese => write!(f, "zh"),
            TargetLanguage::English => write!(f, "en"),
            TargetLanguage::Japanese => write!(f, "ja"),
            TargetLanguage::Korean => write!(f, "ko"),
            TargetLanguage::German => write!(f, "de"),
            TargetLanguage::French => write!(f, "fr"),
            TargetLanguage::Russian => write!(f, "ru"),
            TargetLanguage::Vietnamese => write!(f, "vi"),
        }
    }
}

impl std::str::FromStr for TargetLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "zh" | "chinese" | "中文" => Ok(TargetLanguage::Chinese),
            "en" | "english" | "英文" => Ok(TargetLanguage::English),
            "ja" | "japanese" | "日本語" | "日文" => Ok(TargetLanguage::Japanese),
            "ko" | "korean" | "한국어" | "韩文" => Ok(TargetLanguage::Korean),
            "de" | "german" | "deutsch" | "德文" => Ok(TargetLanguage::German),
            "fr" | "french" | "français" | "法文" => Ok(TargetLanguage::French),
            "ru" | "russian" | "русский" | "俄文" => Ok(TargetLanguage::Russian),
            "vi" | "vietnamese" | "vn" | "vietnam" => Ok(TargetLanguage::Vietnamese),
            _ => Err(format!("Unknown target language: {}", s)),
        }
    }
}

impl TargetLanguage {
    /// Get the descriptive name of the language
    pub fn display_name(&self) -> &'static str {
        match self {
            TargetLanguage::Chinese => "中文",
            TargetLanguage::English => "English",
            TargetLanguage::Japanese => "日本語",
            TargetLanguage::Korean => "한국어",
            TargetLanguage::German => "Deutsch",
            TargetLanguage::French => "Français",
            TargetLanguage::Russian => "Русский",
            TargetLanguage::Vietnamese => "Tiếng Việt",
        }
    }

    /// Get the prompt instruction for the language
    pub fn prompt_instruction(&self) -> &'static str {
        match self {
            TargetLanguage::Chinese => "请使用中文编写文档，确保语言表达准确、专业、易于理解。",
            TargetLanguage::English => "Please write the documentation in English, ensuring accurate, professional, and easy-to-understand language.",
            TargetLanguage::Japanese => "日本語でドキュメントを作成してください。正確で専門的で理解しやすい言語表現を心がけてください。",
            TargetLanguage::Korean => "한국어로 문서를 작성해 주세요. 정확하고 전문적이며 이해하기 쉬운 언어 표현을 사용해 주세요.",
            TargetLanguage::German => "Bitte schreiben Sie die Dokumentation auf Deutsch und stellen Sie sicher, dass die Sprache präzise, professionell und leicht verständlich ist.",
            TargetLanguage::French => "Veuillez rédiger la documentation en français, en vous assurant que le langage soit précis, professionnel et facile à comprendre.",
            TargetLanguage::Russian => "Пожалуйста, напишите документацию на русском языке, обеспечив точность, профессионализм и понятность изложения.",
            TargetLanguage::Vietnamese => "Hãy viết toàn bộ tài liệu bằng tiếng Việt tự nhiên, chính xác và dễ hiểu, sử dụng đúng thuật ngữ kỹ thuật.",
        }
    }

    /// Get directory name
    pub fn get_directory_name(&self, dir_type: &str) -> String {
        match self {
            TargetLanguage::Chinese => {
                match dir_type {
                    "deep_exploration" => "4、深入探索".to_string(),
                    _ => dir_type.to_string(),
                }
            }
            TargetLanguage::English => {
                match dir_type {
                    "deep_exploration" => "4.Deep-Exploration".to_string(),
                    _ => dir_type.to_string(),
                }
            }
            TargetLanguage::Japanese => {
                match dir_type {
                    "deep_exploration" => "4-詳細探索".to_string(),
                    _ => dir_type.to_string(),
                }
            }
            TargetLanguage::Korean => {
                match dir_type {
                    "deep_exploration" => "4-심층-탐색".to_string(),
                    _ => dir_type.to_string(),
                }
            }
            TargetLanguage::German => {
                match dir_type {
                    "deep_exploration" => "4-Tiefere-Erkundung".to_string(),
                    _ => dir_type.to_string(),
                }
            }
            TargetLanguage::French => {
                match dir_type {
                    "deep_exploration" => "4-Exploration-Approfondie".to_string(),
                    _ => dir_type.to_string(),
                }
            }
            TargetLanguage::Russian => {
                match dir_type {
                    "deep_exploration" => "4-Глубокое-Исследование".to_string(),
                    _ => dir_type.to_string(),
                }
            }
            TargetLanguage::Vietnamese => {
                match dir_type {
                    "deep_exploration" => "4-Khám-phá-chi-tiết".to_string(),
                    _ => dir_type.to_string(),
                }
            }
        }
    }

    /// Get document filename
    pub fn get_doc_filename(&self, doc_type: &str) -> String {
        match self {
            TargetLanguage::Chinese => {
                match doc_type {
                    "overview" => "1、项目概述.md".to_string(),
                    "architecture" => "2、架构概览.md".to_string(),
                    "workflow" => "3、工作流程.md".to_string(),
                    "boundary" => "5、边界调用.md".to_string(),
                    "database" => "6、数据库概览.md".to_string(),
                    _ => format!("{}.md", doc_type),
                }
            }
            TargetLanguage::English => {
                match doc_type {
                    "overview" => "1.Overview.md".to_string(),
                    "architecture" => "2.Architecture.md".to_string(),
                    "workflow" => "3.Workflow.md".to_string(),
                    "boundary" => "5.Boundary-Interfaces.md".to_string(),
                    "database" => "6.Database-Overview.md".to_string(),
                    _ => format!("{}.md", doc_type),
                }
            }
            TargetLanguage::Japanese => {
                match doc_type {
                    "overview" => "1-プロジェクト概要.md".to_string(),
                    "architecture" => "2-アーキテクチャ概要.md".to_string(),
                    "workflow" => "3-ワークフロー.md".to_string(),
                    "boundary" => "5-境界インターフェース.md".to_string(),
                    "database" => "6-データベース概要.md".to_string(),
                    _ => format!("{}.md", doc_type),
                }
            }
            TargetLanguage::Korean => {
                match doc_type {
                    "overview" => "1-프로젝트-개요.md".to_string(),
                    "architecture" => "2-아키텍처-개요.md".to_string(),
                    "workflow" => "3-워크플로우.md".to_string(),
                    "boundary" => "5-경계-인터페이스.md".to_string(),
                    "database" => "6-데이터베이스-개요.md".to_string(),
                    _ => format!("{}.md", doc_type),
                }
            }
            TargetLanguage::German => {
                match doc_type {
                    "overview" => "1-Projektübersicht.md".to_string(),
                    "architecture" => "2-Architekturübersicht.md".to_string(),
                    "workflow" => "3-Arbeitsablauf.md".to_string(),
                    "boundary" => "5-Grenzschnittstellen.md".to_string(),
                    "database" => "6-Datenbankübersicht.md".to_string(),
                    _ => format!("{}.md", doc_type),
                }
            }
            TargetLanguage::French => {
                match doc_type {
                    "overview" => "1-Aperçu-du-Projet.md".to_string(),
                    "architecture" => "2-Aperçu-de-l'Architecture.md".to_string(),
                    "workflow" => "3-Flux-de-Travail.md".to_string(),
                    "boundary" => "5-Interfaces-de-Frontière.md".to_string(),
                    "database" => "6-Aperçu-Base-de-Données.md".to_string(),
                    _ => format!("{}.md", doc_type),
                }
            }
            TargetLanguage::Russian => {
                match doc_type {
                    "overview" => "1-Обзор-Проекта.md".to_string(),
                    "architecture" => "2-Обзор-Архитектуры.md".to_string(),
                    "workflow" => "3-Рабочий-Процесс.md".to_string(),
                    "boundary" => "5-Граничные-Интерфейсы.md".to_string(),
                    "database" => "6-Обзор-Базы-Данных.md".to_string(),
                    _ => format!("{}.md", doc_type),
                }
            }
            TargetLanguage::Vietnamese => {
                match doc_type {
                    "overview" => "1-Tổng-quan-Dự-án.md".to_string(),
                    "architecture" => "2-Kiến-trúc.md".to_string(),
                    "workflow" => "3-Luồng-xử-lý.md".to_string(),
                    "boundary" => "5-Lớp-giao-tiếp-biên.md".to_string(),
                    "database" => "6-Tổng-quan-Cơ-sở-Dữ-liệu.md".to_string(),
                    _ => format!("{}.md", doc_type),
                }
            }
        }
    }

    /// English label for the target language, used in console logs only
    pub fn english_label(&self) -> &'static str {
        match self {
            TargetLanguage::Chinese => "Chinese",
            TargetLanguage::English => "English",
            TargetLanguage::Japanese => "Japanese",
            TargetLanguage::Korean => "Korean",
            TargetLanguage::German => "German",
            TargetLanguage::French => "French",
            TargetLanguage::Russian => "Russian",
            TargetLanguage::Vietnamese => "Vietnamese",
        }
    }

    // ===== Console Messages (always English) =====

    /// Warning: Cannot read config file, using default config
    pub fn msg_config_read_error(&self) -> &'static str {
        "⚠️ Warning: Cannot read config file {:?}, using default config"
    }

    /// Warning: Unknown provider, using default provider
    pub fn msg_unknown_provider(&self) -> &'static str {
        "⚠️ Warning: Unknown provider: {}, using default provider"
    }

    /// Warning: Unknown target language, using default language (English)
    pub fn msg_unknown_language(&self) -> &'static str {
        "⚠️ Warning: Unknown target language: {}, using default language (English)"
    }

    /// Using cached AI analysis result
    pub fn msg_cache_hit(&self) -> &'static str {
        "   ✅ Using cached AI analysis result: {}"
    }

    /// Performing AI analysis — {} = current, {} = total, {} = path
    pub fn msg_ai_analyzing(&self) -> &'static str {
        "   🤖 Performing AI analysis [{}/{}]: {}"
    }

    /// Cache miss - need AI inference
    pub fn msg_cache_miss(&self) -> &'static str {
        "   ⌛ Cache miss [{}] - AI inference required"
    }

    /// Cache write - result cached
    pub fn msg_cache_write(&self) -> &'static str {
        "   💾 Cache write [{}] - Result cached"
    }

    /// Cache error
    pub fn msg_cache_error(&self) -> &'static str {
        "   ❌ Cache error [{}]: {}"
    }

    /// Using cached compression result
    pub fn msg_cache_compression_hit(&self) -> &'static str {
        "   💾 Using cached compression result [{}]"
    }

    /// Cannot read file
    #[allow(dead_code)]
    pub fn msg_cannot_read_file(&self) -> &'static str {
        "Cannot read file: {}"
    }

    /// Agent type display names (always English, used in console logs)
    pub fn msg_agent_type(&self, agent_type: &str) -> String {
        match agent_type {
            "system_context" => "System Context Research Report".to_string(),
            "domain_modules" => "Domain Modules Research Report".to_string(),
            "architecture" => "System Architecture Research Report".to_string(),
            "workflow" => "Workflow Research Report".to_string(),
            "key_modules" => "Key Modules and Components Research Report".to_string(),
            "boundary" => "Boundary Interface Research Report".to_string(),
            "database" => "Database Overview Research Report".to_string(),
            _ => agent_type.to_string(),
        }
    }

    /// Warning: Document content not found
    pub fn msg_doc_not_found(&self) -> &'static str {
        "⚠️ Warning: Document content not found, key: {}"
    }

    /// Mermaid fixer error
    pub fn msg_mermaid_error(&self) -> &'static str {
        "⚠️ Error occurred during mermaid diagram repair: {}"
    }

    /// Chromium unavailable for mermaid fixing
    pub fn msg_mermaid_chromium_unavailable(&self) -> &'static str {
        "⚠️ Chromium/Chrome not detected, skipping mermaid diagram fixing"
    }

    /// Summary reasoning failed
    pub fn msg_summary_reasoning_failed(&self) -> &'static str {
        "⚠️  Summary reasoning failed, returning original partial result...{}"
    }

    /// Domain analysis failed
    pub fn msg_domain_analysis_failed(&self) -> &'static str {
        "⚠️ Domain module analysis: {} analysis failed: {}"
    }

    /// No code path for domain
    pub fn msg_no_code_path_for_domain(&self) -> &'static str {
        "⚠️ Domain '{}' has no associated code paths"
    }
}