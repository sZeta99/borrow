use crate::config::Command;
use anyhow::{Context, Result};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::process::Command as Cmd;

pub fn execute_command(command: &str) -> Result<()> {
    Cmd::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .with_context(|| format!("Failed to execute command: {}", command))?;
    Ok(())
}

pub fn show_recent(commands: &mut [Command]) {
    commands.sort_by_key(|cmd| cmd.last_used.clone());
    commands.reverse();
}

pub fn show_most_used(commands: &mut [Command]) {
    commands.sort_by_key(|cmd| cmd.usage_count);
    commands.reverse();
}

pub fn fuzzy_search<'a>(commands: &'a [Command], query: &str) -> Vec<&'a Command> {
    let matcher = SkimMatcherV2::default();
    let mut results: Vec<_> = commands
        .iter()
        .filter_map(|cmd| {
            matcher
                .fuzzy_match(&cmd.command, query)
                .map(|score| (cmd, score))
        })
        .collect();
    results.sort_by_key(|&(_, score)| -score);
    results.into_iter().map(|(cmd, _)| cmd).collect()
}
