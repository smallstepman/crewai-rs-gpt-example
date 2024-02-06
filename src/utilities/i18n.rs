use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct I18N {
    #[serde(skip)]
    translations: HashMap<String, HashMap<String, String>>,
    language: Option<String>,
}

impl I18N {
    pub fn new(language: Option<String>) -> Self {
        I18N {
            translations: HashMap::new(),
            language,
        }
    }

    pub fn load_translation(&mut self) -> Result<(), String> {
        let language = self.language.clone().unwrap_or_else(|| "en".to_string());
        let mut dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir_path.push(format!("../translations/{}.json", language));

        let file_content = fs::read_to_string(dir_path)
            .map_err(|_| format!("Translation file for language '{}' not found.", language))?;

        self.translations = serde_json::from_str(&file_content)
            .map_err(|_| "Error decoding JSON from the prompts file.".to_string())?;

        Ok(())
    }

    pub fn slice(&self, slice: &str) -> Result<String, String> {
        self.retrieve("slices", slice)
    }

    pub fn errors(&self, error: &str) -> Result<String, String> {
        self.retrieve("errors", error)
    }

    pub fn tools(&self, tool: &str) -> Result<String, String> {
        self.retrieve("tools", tool)
    }

    fn retrieve(&self, kind: &str, key: &str) -> Result<String, String> {
        self.translations
            .get(kind)
            .and_then(|k| k.get(key))
            .cloned()
            .ok_or_else(|| format!("Translation for '{}':'{}' not found.", kind, key))
    }
}

fn main() {
    let mut i18n = I18N::new(Some("en".to_string()));
    match i18n.load_translation() {
        Ok(_) => {
            println!("Translations loaded successfully.");
        }
        Err(e) => {
            println!("Error loading translations: {}", e);
        }
    }
}
