use crate::config::{self, AppConfig};
use chrono::Local;
use std::{
    fmt,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    thread,
    time::Duration,
};

#[derive(Debug)]
pub enum StoreError {
    InvalidInput(String),
    Io(std::io::Error),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(f, "{message}"),
            Self::Io(err) => write!(f, "{err}"),
        }
    }
}

impl From<std::io::Error> for StoreError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub fn append(config: &AppConfig, body: &str) -> Result<PathBuf, StoreError> {
    config::validate(config).map_err(StoreError::InvalidInput)?;

    let trimmed = body.trim();
    if trimmed.is_empty() {
        return Err(StoreError::InvalidInput("Memo body is empty.".to_string()));
    }

    let mut last_error = None;
    for attempt in 0..4 {
        match append_once(config, trimmed) {
            Ok(path) => return Ok(path),
            Err(StoreError::Io(err)) => {
                last_error = Some(err);
                if attempt < 3 {
                    thread::sleep(Duration::from_millis(100));
                }
            }
            Err(err) => return Err(err),
        }
    }

    Err(StoreError::Io(last_error.unwrap_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to append memo.")
    })))
}

fn append_once(config: &AppConfig, body: &str) -> Result<PathBuf, StoreError> {
    let now = Local::now();
    let stem = now.format(&config.filename_format).to_string();
    validate_filename_stem(&stem)?;

    let base_dir = PathBuf::from(&config.save_directory);
    fs::create_dir_all(&base_dir)?;

    let path = base_dir.join(format!("{stem}.md"));
    let is_new = !path.exists() || path.metadata().map(|m| m.len() == 0).unwrap_or(false);

    let mut entry = String::new();
    if is_new {
        let date = now.format("%Y-%m-%d");
        entry.push_str(&format!("---\ndate: {date}\ncreated_by: memoake\n---\n\n"));
        entry.push_str(&format!("# {date}\n\n"));
    }

    let heading = "#".repeat(config.timestamp_heading_level as usize);
    let timestamp = now.format(&config.timestamp_format);
    entry.push_str(&format!("{heading} {timestamp}\n\n{body}\n\n"));

    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
    file.write_all(entry.as_bytes())?;
    file.flush()?;

    Ok(path)
}

fn validate_filename_stem(stem: &str) -> Result<(), StoreError> {
    if stem.trim().is_empty() {
        return Err(StoreError::InvalidInput(
            "Filename format produced an empty filename.".to_string(),
        ));
    }

    if let Some(ch) = stem
        .chars()
        .find(|ch| matches!(ch, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'))
    {
        return Err(StoreError::InvalidInput(format!(
            "Filename format produced a Windows-forbidden character: {ch}"
        )));
    }

    Ok(())
}
