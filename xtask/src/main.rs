use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: CommandName,
}

#[derive(Subcommand)]
enum CommandName {
    Build,
    Check,
    Test,
    Fmt,
    Lint,
    Clean,
    Release,
    Package,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut command = match cli.command {
        CommandName::Build => cargo("build"),
        CommandName::Check => cargo("check"),
        CommandName::Test => cargo("test"),
        CommandName::Fmt => cargo("fmt"),
        CommandName::Lint => { let mut command = cargo("clippy"); command.args(["--", "-D", "warnings"]); command },
        CommandName::Clean => cargo("clean"),
        CommandName::Release => { let mut command = cargo("build"); command.args(["--release"]); command },
        CommandName::Package => { let mut command = cargo("build"); command.args(["--release"]); command },
    };
    let status = command.status()?;
    if status.success() { Ok(()) } else { anyhow::bail!("command failed with {status}") }
}

fn cargo(subcommand: &str) -> Command {
    let mut command = Command::new("cargo");
    command.arg(subcommand);
    command
}
