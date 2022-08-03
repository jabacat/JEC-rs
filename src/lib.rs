#[cfg(test)]
mod tests {
    #[test]
    fn test_config_file_init() {
        let conf = ConfigFile { path: "./test.yml" };
        assert_eq!(conf.path, "test.yml")
    }

    #[test]
    fn test_config_file_exists() {
        let conf = ConfigFile { path: "./test.yml" };
        assert!(!conf.exists());
    }

    #[test]
    fn test_config_dir_init() {
        let conf = ConfigDir { path: "./config/" };
        assert!(conf.path.contains("home"));
    }

    #[test]
    fn test_config_dir_exists() {
        let conf = ConfigDir { path: "./config/" };
        assert!(!conf.exists());
    }
}
