use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct TomlHelper {
    file_path: PathBuf,
}

impl TomlHelper {
    /// Create a new instance of `TomlHelper` with the specified file path.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            file_path: path.as_ref().to_path_buf(),
        }
    }

    /// Reads the TOML file and deserializes its content into a structure of type `T`.
    pub fn read<T>(&self) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("File error: {:?}: {}", self.file_path, e))?;
        let config: T = toml::from_str(&content)
            .map_err(|e| format!("Error of TOML parsing: {:?}: {}", self.file_path, e))?;

        Ok(config)
    }

    /// Saves the provided data structure of type `T` into the TOML file, overwriting any existing content.
    pub fn write<T>(&self, data: &T) -> Result<(), String>
    where
        T: Serialize,
    {
        let toml_string = toml::to_string_pretty(data)
            .map_err(|e| format!("Erreur de sérialisation TOML : {}", e))?;
        let mut file = File::create(&self.file_path)
            .map_err(|e| format!("Impossible de créer le fichier {:?}: {}", self.file_path, e))?;

        file.write_all(toml_string.as_bytes())
            .map_err(|e| format!("Erreur d'écriture dans {:?}: {}", self.file_path, e))?;
        Ok(())
    }

    /// Reads the TOML file and deserializes its content into a structure of type `T`.
    pub fn read_or_create_default<T>(&self, default_data: T) -> Result<T, String>
    where
        T: Serialize + DeserializeOwned + Clone,
    {
        if !self.file_path.exists() {
            self.write(&default_data)?;
            Ok(default_data)
        } else {
            self.read()
        }
    }
}
