use anyhow::{Context, Result};
use chrono::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::process;
use std::{fs, path::PathBuf};

use crate::data_managing::{Storage, TimerState};

const WEEKS_PATH: &str = "Weeks";
const PROYECTS_PATH: &str = "Projects";

pub struct TextStorage {
    data_path: PathBuf,
    todays_file: PathBuf,
}

impl TextStorage {
    pub fn new(config_path: PathBuf) -> Self {
        let todays_file = get_todays_filename(&config_path);
        Self {
            data_path: config_path,
            todays_file,
        }
    }
}

impl Storage for TextStorage {
    fn init(&self) -> Result<()> {
        if !self.todays_file.exists() {
            println!("Todays file does not exist, creating it...");
            if let Err(e) = fs::write(&self.todays_file, "") {
                eprintln!("Error creating file: {}", e);
                process::exit(1);
            } else {
                println!("Todays file created sucessfully");
            }
        }
        Ok(())
    }

    fn get_timer_state(&self) -> TimerState {
        if !self.todays_file.exists()
            || fs::metadata(&self.todays_file)
                .expect("Error retrieving todays file metada")
                .len()
                == 0
        {
            return TimerState::NotStarted;
        } else {
            match File::open(&self.todays_file) {
                Ok(mut file) => {
                    use std::io::Seek;
                    use std::io::SeekFrom;

                    file.seek(SeekFrom::End(-1))
                        .expect("Error seeking 2nd ending byte");

                    let mut buffer = vec![0u8; 1];
                    file.read_exact(&mut buffer)
                        .expect("Error reading 2 last bytes");

                    if buffer[0] == 10 {
                        return TimerState::NotStarted;
                    } else {
                        return TimerState::Started;
                    }
                }
                Err(e) => eprintln!("Failed to open file: {}", e),
            }
        }
        return TimerState::NotStarted;
    }

    fn get_projects(&self) -> Vec<String> {
        match fs::read_dir(self.data_path.join(PROYECTS_PATH)) {
            Ok(entries) => entries
                .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
                .collect(),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    println!("Proyect folder not found, creating it ...");
                    if let Err(create_error) =
                        fs::create_dir_all(self.data_path.join(PROYECTS_PATH))
                    {
                        eprintln!("Failed to create the proyect folder: {}", create_error);
                    }
                    Vec::new()
                }
                _ => {
                    eprintln!("Generic error reading directory: {}", e);
                    Vec::new()
                }
            },
        }
    }

    fn get_tasks_from_project(&self, project_name: &str) -> Result<Vec<String>> {
        let project_path = format!(
            "{}/{}",
            self.data_path.join(PROYECTS_PATH).display(),
            project_name
        );
        let content = fs::read_to_string(&project_path).with_context(|| {
            format!("No se pudo leer el archivo del proyecto: {}", project_path)
        })?;
        Ok(content.lines().map(|line| line.to_string()).collect())
    }

    fn create_project(&self, project_name: &str) -> Result<String> {
        let _ = fs::create_dir_all(self.data_path.join(PROYECTS_PATH));
        let resultado = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(construct_project_path(&self.data_path, project_name));

        match resultado {
            Ok(_) => Ok(project_name.to_string()),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                Err(anyhow::anyhow!("El archivo ya existe"))
            }
            Err(e) => panic!("Ocurrió un error inesperado: {}", e),
        }
    }

    fn create_task(&self, project_name: &str, task_name: &str) {
        let resultado = OpenOptions::new()
            .write(true)
            .append(true)
            .open(construct_project_path(&self.data_path, project_name));
        if let Ok(mut file) = resultado {
            let _ = writeln!(file, "{}", task_name);
        }
    }

    fn start_timer_on_task(&self, project_name: &str, task_name: &str) -> Result<()> {
        match OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(get_todays_filename(&self.data_path))
        {
            Ok(mut file) => {
                let _ = write!(
                    file,
                    "{} {}_{} (",
                    Local::now().format("%H:%M"),
                    project_name.replace(" ", "-").replace(".txt", ""),
                    task_name.replace(" ", "-")
                );
                Ok(())
            }
            Err(_) => Err(anyhow::anyhow!("Error writing to file")),
        }
    }

    fn end_timer_on_task(&self, input_buffer: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.todays_file)
            .with_context(|| format!("Failed to create file: {}", self.todays_file.display()))?;

        writeln!(file, "{}) {}", input_buffer, Local::now().format("%H:%M"))
            .with_context(|| "Failed to write in file")?;
        Ok(())
    }
}

// Auxiliar Functions

fn get_todays_filename(data_path: &PathBuf) -> PathBuf {
    let now = Local::now();
    let week = now.iso_week().week();
    let year = now.year();
    let folder_path = format!(
        "{}/{} W{:02}",
        data_path.join(WEEKS_PATH).display(),
        year,
        week
    );

    fs::create_dir_all(&folder_path).expect("Failed to create directory");
    PathBuf::from(format!("{}/{}.txt", folder_path, now.format("%d-%m-%Y")))
}

fn construct_project_path(data_path: &PathBuf, project_name: &str) -> PathBuf {
    if project_name.ends_with(".txt") {
        data_path.join(PROYECTS_PATH).join(project_name)
    } else {
        PathBuf::from(format!(
            "{}/{}.txt",
            data_path.join(PROYECTS_PATH).display(),
            project_name
        ))
    }
}
