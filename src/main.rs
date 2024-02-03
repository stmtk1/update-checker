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

const WHICH_COMMAND: &'static str = "which";
const BREW_COMMAND: &'static str = "brew";
const CONFIG_ARG: &'static str = "config";
const LINE_REGEX: &'static str = r"(?m)^Core tap JSON:\s*(.+)\s*$";
const DATE_REGEX: &'static str = r"\d{2}\s+\w+\s\d{2}:\d{2}\sUTC";


fn main() -> anyhow::Result<()> {
    command_validation()?;
    println!("{:?}", get_last_updated()?);
    Ok(())
}


fn command_validation() -> Result<()> {
    let output = Command::new(WHICH_COMMAND).args([BREW_COMMAND]).output().map_err(|_| ApplicationError::CommandRunFailed { command_name: String::from(format!("{} {}", WHICH_COMMAND, BREW_COMMAND)) })?;
    if !output.status.success() {
        return Err(ApplicationError::CommandNotFound { command_name: String::from(BREW_COMMAND) }.into())
    }
    Ok(())
}

fn get_last_updated() -> Result<String> {
    let output = Command::new(BREW_COMMAND).args([CONFIG_ARG]).output().map_err(|_| ApplicationError::CommandRunFailed { command_name: String::from(format!("{} {}", BREW_COMMAND, CONFIG_ARG)) })?;
    if !output.status.success() {
        return Err(ApplicationError::CommandRunFailed { command_name: String::from(format!("{} {}", BREW_COMMAND, CONFIG_ARG)) }.into())
    }
    let config = String::from_utf8(output.stdout).map_err(|_| ApplicationError::StringFormatError{})?;
    // let line_matcher = Regex::new("^Core tap JSON:\\s*(/.+)\\s*$").map_err(|_| ApplicationError::InvalidPattern{})?;
    let line_matcher = Regex::new(LINE_REGEX).map_err(|_| ApplicationError::InvalidPattern{})?;
    let line = String::from(line_matcher.find(&config).ok_or(ApplicationError::PatternNotFound { pattern: String::from(LINE_REGEX) })?.as_str());
    let date_matcher = Regex::new(DATE_REGEX).map_err(|_| ApplicationError::InvalidPattern{})?;
    let date_str = date_matcher.find(&line).ok_or(ApplicationError::PatternNotFound { pattern: String::from(DATE_REGEX) })?.as_str();
    Ok(String::from(date_str))
}