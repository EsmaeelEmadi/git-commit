use crate::errors::ReadFileError;
use crate::types::{Args, Config};

use std::fs;
use std::path::Path;

pub fn read_config(args: &Args) -> Result<Config, ReadFileError> {
    let current_dir = std::env::current_dir().map_err(ReadFileError::CurrentDirError)?;

    let config_file = format!("{}.yml", &args.config);
    let config_file_path = Path::new(&current_dir).join(config_file);

    let file = fs::File::open(&config_file_path).map_err(|e| ReadFileError::FileOpenError {
        path: config_file_path.clone(),
        source: e,
    })?;

    let content = serde_yml::from_reader(file).map_err(ReadFileError::DeserializeError)?;

    Ok(content)
}
