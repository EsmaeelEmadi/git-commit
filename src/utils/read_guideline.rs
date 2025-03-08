use crate::errors::ReadFileError;
use crate::types::Args;

use std::fs;
use std::path::Path;

pub fn read_guideline(args: &Args) -> Result<String, ReadFileError> {
    let current_dir = std::env::current_dir().map_err(ReadFileError::CurrentDirError)?;

    let guideline_file = format!("{}.md", &args.config);
    let guideline_file_path = Path::new(&current_dir).join(guideline_file);

    let content =
        fs::read_to_string(&guideline_file_path).map_err(|e| ReadFileError::FileOpenError {
            path: guideline_file_path.clone(),
            source: e,
        })?;

    Ok(content)
}
