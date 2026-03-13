use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

const PROGRESS_DIR: &str = ".dsa-forge";
const PROGRESS_FILE: &str = "progress.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Progress {
    /// Lessons the user has marked as read: lesson_id -> true
    #[serde(default)]
    pub lessons_read: HashMap<String, bool>,

    /// Problems solved: problem_id -> best result
    #[serde(default)]
    pub problems: HashMap<String, ProblemProgress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemProgress {
    pub solved: bool,
    pub best_comparisons: Option<usize>,
    pub best_swaps: Option<usize>,
    pub best_total_ops: Option<usize>,
}

impl Progress {
    fn path() -> PathBuf {
        PathBuf::from(PROGRESS_DIR).join(PROGRESS_FILE)
    }

    /// Load progress from disk, returning default if file doesn't exist.
    pub fn load() -> Self {
        let path = Self::path();
        match std::fs::read_to_string(&path) {
            Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save progress to disk.
    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(path, contents)
    }
}
