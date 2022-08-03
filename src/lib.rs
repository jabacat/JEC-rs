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
    fn test_config_file_create() {
        let conf = ConfigFile { path: "./test.yml" };
        assert!(!conf.exists());
        conf.create();
        assert!(conf.exists());
    }

    #[test]
    fn test_config_file_remove() {
        let conf = ConfigFile { path: "./test.yml" };
        assert!(conf.exists());
        conf.remove();
        assert!(!conf.exists());
    }

    #[test]
    fn test_config_file_from_home() {
        let conf = ConfigFile::from_home("./test.yml");
        assert!(conf.path.contains("home"));
    }

    #[test]
    fn test_config_dir_init() {
        let conf = ConfigFile { path: "./config/" };
        assert_eq!(conf.path, "config/")
    }

    #[test]
    fn test_config_dir_exists() {
        let conf = ConfigDir { path: "./config/" };
        assert!(!conf.exists());
    }

    #[test]
    fn test_config_dir_create() {
        let conf = ConfigDir { path: "./config/" };
        assert!(!conf.exists());
        conf.create();
        assert!(conf.exists());
    }

    #[test]
    fn test_config_dir_remove() {
        let conf = ConfigDir { path: "./config/" };
        assert!(conf.exists());
        conf.remove();
        assert!(!conf.exists());
    }

    #[test]
    fn test_config_dir_from_home() {
        let conf = ConfigDir::from_home("./config/");
        assert!(conf.path.contains("home"));
    }
}
