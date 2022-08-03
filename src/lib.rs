use dirs::home_dir;
use std::fs;
use std::fs::File;
use std::path::Path;

struct ConfigFile {
    path: String,
}

trait FileActions {
    fn exists(&self) -> bool;
    fn create(&self);
    fn remove(&self);
    fn from_home(path: String) -> ConfigFile;
}

struct ConfigDir {
    path: String,
}

trait DirActions {
    fn exists(&self) -> bool;
    fn create(&self);
    fn remove(&self);
    fn from_home(path: String) -> ConfigFile;
}

impl FileActions for ConfigFile {
    fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    fn create(&self) {
        File::create(&self.path).expect("Error encountered while creating file!");
    }

    fn remove(&self) {
        fs::remove_file(&self.path).unwrap();
    }

    fn from_home(path: String) -> ConfigFile {
        let home = home_dir().unwrap();
        let first = Path::new(&home)
            .join(path)
            .into_os_string()
            .into_string()
            .unwrap();

        ConfigFile { path: first }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_file_init() {
        let conf = ConfigFile {
            path: "./test.yml".to_string(),
        };
        assert_eq!(conf.path, "test.yml")
    }

    #[test]
    fn test_config_file_exists() {
        let conf = ConfigFile {
            path: "./test.yml".to_string(),
        };
        assert!(!conf.exists());
    }

    #[test]
    fn test_config_file_create() {
        let conf = ConfigFile {
            path: "./test.yml".to_string(),
        };
        assert!(!conf.exists());
        conf.create();
        assert!(conf.exists());
    }

    #[test]
    fn test_config_file_remove() {
        let conf = ConfigFile {
            path: "./test.yml".to_string(),
        };
        assert!(conf.exists());
        conf.remove();
        assert!(!conf.exists());
    }

    #[test]
    fn test_config_file_from_home() {
        let conf = ConfigFile::from_home("./test.yml".to_string());
        assert!(conf.path.contains("home"));
    }

    #[test]
    fn test_config_dir_init() {
        let conf = ConfigFile {
            path: "./config/".to_string(),
        };
        assert_eq!(conf.path, "config/")
    }

    #[test]
    fn test_config_dir_exists() {
        let conf = ConfigDir {
            path: "./config/".to_string(),
        };
        assert!(!conf.exists());
    }

    #[test]
    fn test_config_dir_create() {
        let conf = ConfigDir {
            path: "./config/".to_string(),
        };
        assert!(!conf.exists());
        conf.create();
        assert!(conf.exists());
    }

    #[test]
    fn test_config_dir_remove() {
        let conf = ConfigDir {
            path: "./config/".to_string(),
        };
        assert!(conf.exists());
        conf.remove();
        assert!(!conf.exists());
    }

    #[test]
    fn test_config_dir_from_home() {
        let conf = ConfigDir::from_home("./config/".to_string());
        assert!(conf.path.contains("home"));
    }
}
