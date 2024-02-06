use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Default)]
struct I18N {
    translations: HashMap<String, HashMap<String, String>>,
    language: Option<String>,
}

impl I18N {
    fn load_translation(&mut self) -> Result<(), String> {
        let language = self.language.clone().unwrap_or_else(|| "en".to_string());
        let dir_path = std::env::current_dir().map_err(|e| e.to_string())?;
        let prompts_path = dir_path
            .join(format!("../translations/{}.json", language))
            .canonicalize()
            .map_err(|e| e.to_string())?;

        let mut file = File::open(prompts_path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| e.to_string())?;

        let translations: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&contents).map_err(|e| e.to_string())?;

        self.translations = translations;

        Ok(())
    }

    fn slice(&self, slice: &str) -> Result<String, String> {
        self.retrieve("slices", slice)
    }

    fn errors(&self, error: &str) -> Result<String, String> {
        self.retrieve("errors", error)
    }

    fn tools(&self, error: &str) -> Result<String, String> {
        self.retrieve("tools", error)
    }

    fn retrieve(&self, kind: &str, key: &str) -> Result<String, String> {
        match self.translations.get(kind) {
            Some(map) => match map.get(key) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Translation for '{}':'{}' not found.", kind, key)),
            },
            None => Err(format!("Translation for '{}' not found.", kind)),
        }
    }
}
