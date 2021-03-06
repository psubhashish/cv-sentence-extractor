#[derive(Clone)]
pub struct Config {
    pub language: String,
    pub no_check: bool,
    pub directory: String,
    pub max_sentences_per_text: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            language: String::from("en"),
            no_check: false,
            directory: String::from(""),
            max_sentences_per_text: 3,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_config() {
        let config : Config = Config {
            ..Default::default()
        };

        assert_eq!(config.language, "en");
        assert_eq!(config.no_check, false);
        assert_eq!(config.directory, "");
        assert_eq!(config.max_sentences_per_text, 3);
    }
}
