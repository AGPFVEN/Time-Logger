use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{fs, io::ErrorKind, panic};

use app_core::data_managing::{Storage, TimerState, StorageConfig};
mod subcommands;
use subcommands::{
    link_tasks::link_tasks,
    record_time_entry::{end_record_note, start_record_note},
};

#[derive(Deserialize, Debug)]
struct ConfigPrincipal {
    storage: StorageConfig,
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
    Link,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all arguments
    let args = Args::parse();

    // Lectura, parseo e inicialización en 3 líneas
    let config_content = match fs::read_to_string(&args.config_path) {
        Ok(content) => content,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                return Err(format!(
                    "Config file could not be found in '{}'",
                    args.config_path.display()
                )
                .into());
            }
            ErrorKind::PermissionDenied => {
                return Err(format!(
                    "There's no permission to read '{}'",
                    args.config_path.display()
                )
                .into());
            }
            _ => return Err(Box::new(err)),
        },
    };

    let config: ConfigPrincipal = match toml::from_str(&config_content) {
        Ok(parsed) => parsed,
        Err(err) => {
            // `err` de toml::de::Error incluye la línea y columna exacta del fallo
            return Err(format!("Error al parsear el TOML:\n{err}").into());
        }
    };

    let storage_obj: Box<dyn Storage> = config.storage.build();

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
            _ => panic!("The operation related to recording a time entry couldn't be identified"),
        },
        Commands::Link => match link_tasks(storage_obj) {
            Ok(_) => println!("Task link succesful"),
            Err(_) => panic!("Something has gone very wrong"),
        },
    }
    std::process::exit(0);
}
