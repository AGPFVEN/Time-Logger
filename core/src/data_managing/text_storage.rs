use std::{fs, path::PathBuf};
use std::io::{ErrorKind, Write};
use std::fs::{OpenOptions};
use anyhow::{Result, Context};
use chrono::prelude::*;

pub const WEEKS_PATH: &str = "Weeks";
pub const PROYECTS_PATH: &str = "Projects";

//TODO: devolver vacío y comunicar y si algo no funcionó (para todas las funciones?)

pub fn init(data_path: &PathBuf) -> PathBuf {
    // Check if todays file exists, if not create it
    let filename_path = get_todays_filename(data_path);
    if !filename_path.exists() {
        println!("Todays file does not exist, creating it...");
        if let Err(e) = fs::write(&filename_path, "") {
            println!("Error creating file: {}", e);
        } else {
            println!("Todays file created sucessfully");
        }
    }
    return filename_path;
}

fn construct_project_path(data_path: &PathBuf, project_name: &str) -> PathBuf {
    if project_name.ends_with(".txt") {
        return data_path.join(PROYECTS_PATH).join(project_name);
    } else {
        return PathBuf::from(format!("{}/{}.txt", data_path.join(PROYECTS_PATH).display(), project_name))
    }
}

// It also creates the needed folders if they doesnt exist
pub fn get_todays_filename(data_path: &PathBuf) -> PathBuf {
    let now = Local::now();
    let week = now.iso_week().week();
    let year = now.year();
    let folder_path = format!("{}/{} W{:02}", data_path.join(WEEKS_PATH).display(), year, week);
    fs::create_dir_all(&folder_path).expect("Failed to create directory");
    let filename = format!("{}/{}.txt", folder_path, now.format("%d-%m-%Y"));
    PathBuf::from(filename)
}

// TODO: Usar un search para no traer todos proyectos
pub fn get_projects(data_path: &PathBuf) -> Vec<String> {
    match fs::read_dir(data_path.join(PROYECTS_PATH)) {
        // Get all entries of dir
        Ok(entries) => {
            entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| e.file_name().into_string().ok())
                })
                .collect()
        },
        // If an errors pop, show it
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    //TODO: Ask to create before creating it
                    println!("Proyect folder not found at '{}', creating it ...", data_path.join(PROYECTS_PATH).display());
                    if let Err(create_error) = fs::create_dir_all(data_path.join(PROYECTS_PATH)) { 
                        eprintln!("Failed to create the proyect folder, Reason: {}", create_error);
                    } else {
                        println!("Proyect folder created successfully.");
                    }
                    Vec::new()
                },
                // Handle permission denied or other IO errors
                _ => {
                    eprintln!("Generic error reading directory: {}", e);
                    Vec::new()
                }
            }
        }
    }
}

// TODO: Usar un search para no traer todos proyectos
pub fn get_tasks_from_project(data_path: &PathBuf, project_name: &str) -> Result<Vec<String>> {
    let project_path = format!("{}/{}", data_path.join(PROYECTS_PATH).display(), project_name);

    // El operador '?' reemplaza todo el match. 
    // Si falla, devuelve el error con el contexto que añadimos.
    let content = fs::read_to_string(&project_path)
        .with_context(|| format!("No se pudo leer el archivo del proyecto: {}", project_path))?
    ;

    let project_tasks: Vec<String> = content.lines()
        .map(|line| line.to_string())
        .collect()
    ;

    Ok(project_tasks)
}

// TODO: Dejar al usuario poder tener dos proyectos con el mismo nombre o dar la opción para que se pueda o no
pub fn create_project(data_path: &PathBuf, project_name: &str) -> Result<String> {
    // Create project storage folder if doesnt exist
    if let Err(e) = fs::create_dir_all(data_path.join(PROYECTS_PATH)) {
        eprintln!("Failed to create projects directory: {}", e);
    }
    // Create project file if doesnt exist
    // Técnicamente no es posible guardar dos proyectos con el mismo nombre si se usa el nombre del proyecto como nombre de archivo
    let resultado = OpenOptions::new()
        .write(true)
        .create_new(true) // Esta es la clave
        .open(construct_project_path(data_path, project_name));
    match resultado {
        Ok(_) => {
            println!("Archivo creado exitosamente.");
            return Ok(project_name.to_string());
        },
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            return  Err(anyhow::anyhow!("El archivo ya existe, así que no hice nada."));
        },
        Err(e) => panic!("Ocurrió un error inesperado: {}", e),
    }
}

pub fn create_task(data_path: &PathBuf, project_name: &str, task_name: &str) {
    let resultado = OpenOptions::new()
        .write(true)
        .append(true)
        .open(construct_project_path(data_path, project_name));
    match resultado {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", task_name) {
                eprintln!("Error writing to file: {}", e);
            }
        }
        Err(e) => panic!("Ocurrió un error inesperado: {}", e),
    }
}

pub fn start_timer_on_task(data_path: &PathBuf, project_name: &str, task_name: &str) -> Result<()> {
    match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(get_todays_filename(data_path)) {
            Ok(mut file) => {
                if let Err(e) = write!(file, "{} {}_{} (",
                    Local::now().format("%H:%M"),
                    project_name.replace(" ", "-").replace(".txt", ""),
                    task_name.replace(" ", "-")
                ) {
                    eprintln!("Error writing to file: {}", e);
                }
                Ok(())
            }
            Err(e) => {
                eprintln!("Error writing to file: {}", e);
                return Err(anyhow::anyhow!("Error writing to file"));
            }
        }
}