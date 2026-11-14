use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{fs, io::ErrorKind, panic};

use app_core::data_managing::{Storage, TimerState};
mod subcommands;
use subcommands::{
    link_tasks::link_tasks,
    record_time_entry::{end_record_note, start_record_note},
};

// Structure of config file
#[derive(Deserialize, Debug)]

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum StorageConfig {
    Sqlite { database_url: String },
    Odoo { url: String, db: String, username: String, token: String },
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
    let storage_obj = config.storage.build();

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
            wer => panic!("The operation related to recording a time entry couldn't be identified"),
        },
        Commands::Link => match link_tasks(storage_obj) {
            Ok(_) => println!("Task link succesful"),
            Err(_) => panic!("Something has gone very wrong"),
        },
    }
    std::process::exit(0);
}
