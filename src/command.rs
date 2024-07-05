use anyhow::{Context, Result};

use std::process::Command as Cmd;

pub fn execute_command(command: &str) -> Result<()> {
    Cmd::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .with_context(|| format!("Failed to execute command: {}", command))?;
    Ok(())
}
