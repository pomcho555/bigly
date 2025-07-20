use clap::{App, AppSettings, Arg};

use core::commands;
use utils::app_config::AppConfig;
use utils::error::Result;

/// Match commands
pub fn cli_match() -> Result<()> {
    // Get matches
    let cli_matches = cli_config()?;

    // Merge clap config file if the value is set
    AppConfig::merge_config(cli_matches.value_of("config"))?;

    // Check for search term (main functionality)
    if let Some(search_term) = cli_matches.value_of("search") {
        let target_file = cli_matches.value_of("file");
        commands::search(search_term, target_file)?;
        return Ok(());
    }

    // Matches Commands or display help
    match cli_matches.subcommand_name() {
        Some("hazard") => {
            commands::hazard()?;
        }
        Some("error") => {
            commands::simulate_error()?;
        }
        Some("config") => {
            commands::config()?;
        }
        _ => {
            // Arguments are required by default (in Clap)
            // This section should never execute and thus
            // should probably be logged in case it executed.
        }
    }
    Ok(())
}

/// Configure Clap
/// This function will configure clap and match arguments
pub fn cli_config() -> Result<clap::ArgMatches> {
    let logo = r#"    ____  ______ _     __ __
   / __ )/  _/ /|   / // /
  / __  |/ // /_|  / // /_
 / /_/ // // __/ / /__  __/
/_____/___/_/   /_/  /_/   

    "BIGLY!" 

A CLI tool that greps all files under the current directory"#;

    let cli_app = App::new("bigly")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.0.1")
        .about(logo)
        .author("pomcho555 <pomcho555@users.noreply.github.com>")
        .arg(
            Arg::new("search")
                .help("Search term to grep for in files")
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Set a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .value_name("FILE")
                .help("Target a specific file")
                .takes_value(true),
        )
        .subcommand(App::new("hazard").about("Generate a hazardous occurance"))
        .subcommand(App::new("error").about("Simulate an error"))
        .subcommand(App::new("config").about("Show Configuration"));

    // Get matches
    let cli_matches = cli_app.get_matches();

    Ok(cli_matches)
}
