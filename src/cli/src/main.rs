use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{fs, io::ErrorKind, panic};

use app_core::data_managing::{Storage, TimerState};
mod subcommands;
use subcommands::record_time_entry::{end_record_note, start_record_note};

// Structure of config file
#[derive(Deserialize, Debug)]
struct ConfigPrincipal {
    storage: StorageConfig,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum StorageConfig {
    Sqlite { database_url: String },
}

// Flags
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./config.toml")]
    config_path: std::path::PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Record,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all arguments
    let args = Args::parse();

    // Read, parse and validate config file
    let config_file_content = match fs::read_to_string(&args.config_path) {
        Ok(content) => content,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                panic!("Config file could not be found");
            }
            _ => {
                panic!("Error trying to read config file");
            }
        },
    };
    let config: ConfigPrincipal =
        toml::from_str(&config_file_content).expect("Error while parsing config file");

    // Set up storage configuration
    let storage_obj: Box<dyn Storage> = match config.storage {
        StorageConfig::Sqlite { database_url } => Box::new(
            app_core::data_managing::data_sqlite::SqliteStorage::init(&database_url),
        ),
    };

    // Route subcommand
    match args.command {
        Commands::Record => match storage_obj.get_timer_state() {
            (TimerState::NotStarted, None) => match start_record_note(storage_obj) {
                Ok(_) => println!("Timer started succesfuly"),
                Err(_) => panic!("Error starting timer"),
            },
            (TimerState::Started, Some(time_entry_id)) => {
                match end_record_note(storage_obj, time_entry_id) {
                    Ok(_) => println!("Timer ended succesfuly"),
                    Err(_) => panic!("Error ending timer"),
                }
            }
            _ => panic!("Something has gone very wrong"),
        },
    }
    std::process::exit(0);
}
