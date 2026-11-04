use clap::{CommandFactory, Parser};

use super::{Cli, Commands, ConfigAction};

#[test]
fn cli_definition_is_valid() {
    Cli::command().debug_assert();
}

#[test]
fn no_subcommand_parses_prompt_as_trailing_args() {
    let cli = Cli::try_parse_from(["bbrs", "write", "a", "function"]).unwrap();
    assert!(cli.command.is_none());
    assert_eq!(cli.prompt, vec!["write", "a", "function"]);
}

#[test]
fn config_init_parses_to_config_action() {
    let cli = Cli::try_parse_from(["bbrs", "config", "init"]).unwrap();
    assert!(matches!(
        cli.command,
        Some(Commands::Config {
            action: ConfigAction::Init
        })
    ));
}

#[test]
fn thinking_flags_parse() {
    let cli = Cli::try_parse_from(["bbrs", "--thinking", "enabled", "--thinking-budget", "16000", "hello"]).unwrap();

    assert_eq!(cli.thinking.as_deref(), Some("enabled"));
    assert_eq!(cli.thinking_budget, Some(16_000));
    assert_eq!(cli.prompt, vec!["hello"]);
}

#[test]
fn deleted_flags_are_rejected() {
    assert!(Cli::try_parse_from(["bbrs", "--config-path"]).is_err());
    assert!(Cli::try_parse_from(["bbrs", "--login"]).is_err());
    assert!(Cli::try_parse_from(["bbrs", "--list-sessions"]).is_err());
    assert!(Cli::try_parse_from(["bbrs", "--skills-path"]).is_err());
    assert!(Cli::try_parse_from(["bbrs", "--init-config"]).is_err());
    assert!(Cli::try_parse_from(["bbrs", "--logout"]).is_err());
}
