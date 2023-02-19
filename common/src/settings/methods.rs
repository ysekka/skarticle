use super::r#struct::Settings;

impl Settings {
    pub fn from_path(path: &std::path::Path) -> Self {
        let settings_path = std::fs::read_to_string(path)
        .expect("Error occured during reading settings path.");

        Self::from_str(&settings_path)
    }

    pub fn from_str(string: &str) -> Self {
        ron::from_str::<Settings>(string)
        .expect("Error occured during parsing input text.")
    }
}