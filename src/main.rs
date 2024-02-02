use std::process::Command;
use anyhow::Result;
use thiserror::Error;
use regex::Regex;

#[derive(Debug, Error)]
enum ApplicationError {
    #[error("command not found: {command_name}")]
    CommandNotFound {
        command_name: String,
    },
    #[error("command execution failed: {command_name}")]
    CommandRunFailed {
        command_name: String,
    },
    #[error("string format error")]
    StringFormatError{},
    #[error("regex invalid pattern")]
    InvalidPattern{},
    #[error("pattern not found {pattern}")]
    PatternNotFound {
        pattern: String,
    },
}


fn main() -> anyhow::Result<()> {
    command_validation()?;
    println!("{:?}", get_last_updated()?);
    Ok(())
}


fn command_validation() -> Result<()> {
    let output = Command::new("which").args(["brew"]).output().map_err(|_| ApplicationError::CommandRunFailed { command_name: String::from("which brew") })?;
    if !output.status.success() {
        return Err(ApplicationError::CommandNotFound { command_name: String::from("brew") }.into())
    }
    Ok(())
}

fn get_last_updated() -> Result<String> {
    let output = Command::new("brew").args(["config"]).output().map_err(|_| ApplicationError::CommandRunFailed { command_name: String::from("brew config") })?;
    if !output.status.success() {
        return Err(ApplicationError::CommandRunFailed { command_name: String::from("brew config") }.into())
    }
    let config = String::from_utf8(output.stdout).map_err(|_| ApplicationError::StringFormatError{})?;
    // let line_matcher = Regex::new("^Core tap JSON:\\s*(/.+)\\s*$").map_err(|_| ApplicationError::InvalidPattern{})?;
    let line_matcher = Regex::new(r"(?m)^Core tap JSON:\s*(.+)\s*$").map_err(|_| ApplicationError::InvalidPattern{})?;
    let line = String::from(line_matcher.find(&config).ok_or(ApplicationError::PatternNotFound { pattern: String::from("^Core tap JSON:\\s*(/.+)\\s*$") })?.as_str());
    let date_matcher = Regex::new(r"\d{2}\s+\w+\s\d{2}:\d{2}\s\w+").map_err(|_| ApplicationError::InvalidPattern{})?;
    let date_str = String::from(date_matcher.find(&line).ok_or(ApplicationError::PatternNotFound { pattern: String::from(r"\d{2}\s+\w+\s\d{2}:\d{2}\s\w+") })?.as_str());
    Ok(date_str)
}