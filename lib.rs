use std::path::PathBuf;
use std::error::Error;
use std::fmt::{self,Display};

#[derive(Debug)] 
pub struct ConfigPathError{
    cause: String,
}

impl Display for ConfigPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't get config path: {}", self.cause)
    }
}

impl Error for ConfigPathError {
    // The `source` method is optional but recommended for chaining errors
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // If your error wraps another `Error` type, return a reference to it here.
            // For example, if InvalidInput wrapped a `std::io::Error`, you'd return `Some(inner_error)`.
            _ => None, // No source error in this simple example
        }
    }
}

pub fn get_config_root() -> Result<PathBuf, ConfigPathError> {
    let cfg_path;
    if cfg!(target_os = "macos") {
        match std::env::home_dir() {
            Some(path) => cfg_path = format!("{}/Library/Application Support", path.display().to_string()),
            None => return Err(ConfigPathError{ cause: "no home dir set".to_string()}),
        }
    } else if cfg!(unix) {
        match std::env::var("HOME") {
            Ok(path) => cfg_path = format!("{path}/.config"),
            Err(_) => return Err(ConfigPathError{ cause: "no HOME set".to_string()}),
        }
    } else if cfg!(windows) {
        match std::env::var("LOCALAPPDATA") {
            Ok(path) => cfg_path = path,
            Err(_) => {
                return Err(ConfigPathError{ cause: "no LOCALAPPDATA set".to_string()})
            }
        }
    } else {
        return Err(ConfigPathError{ cause: format!("unsupported platform: {}", std::env::consts::OS)})
    }
    Ok(PathBuf::from(&cfg_path))
}
