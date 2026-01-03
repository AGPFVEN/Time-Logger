use std::{fs, path::PathBuf};
use std::io::{ErrorKind, Write};
use std::fs::{OpenOptions};
use anyhow::{Result, Context};
use chrono::prelude::*;

pub const SEMANA_ACTUAL_PATH: &str = "./data/Esta semana";
pub const PROYECTOS_PATH: &str = "./data/Proyectos";

//TODO: devolver vacío y comunicar y si algo no funcionó (para todas las funciones?)

pub fn init() -> PathBuf {
    // Check if todays file exists, if not create it
    let filename_path_buf = get_filename();
    let filename_path = filename_path_buf.as_path();
    if !filename_path.exists() {
        println!("File does not exist, creating it...");
        if let Err(e) = fs::write(filename_path, "") {
            println!("Error creating file: {}", e);
        } else {
            println!("File created sucessfully");
        }
    }
    return filename_path_buf;
}

fn construct_project_path(project_name: &str) -> PathBuf {
    if project_name.ends_with(".txt") {
        return PathBuf::from(format!("{}/{}", PROYECTOS_PATH, project_name))
    } else {
        return PathBuf::from(format!("{}/{}.txt", PROYECTOS_PATH, project_name))
    }
}

pub fn get_filename() -> PathBuf {
    let now = Local::now();
    let week = now.iso_week().week();
    let year = now.year();
    let folder_path = format!("./data/Semanas anteriores/W{} {}", week, year);
    fs::create_dir_all(&folder_path).expect("Failed to create directory");
    let filename = format!("{}/{}.txt", folder_path, now.format("%d-%m-%Y"));
    PathBuf::from(filename)
}

// TODO: Usar un search para no traer todos proyectos
pub fn get_projects() -> Vec<String> {
    fs::read_dir(PROYECTOS_PATH)
        .map(|entries| {
            entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| e.file_name().into_string().ok())
                })
                .collect()
        })
        .unwrap_or_else(|e| {
            eprintln!("Error reading directory: {}", e);
            Vec::new()
        })
}

// TODO: Usar un search para no traer todos proyectos
pub fn get_tasks_from_project(project_name: &str) -> Result<Vec<String>> {
let project_path = format!("{}/{}", PROYECTOS_PATH, project_name);

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
pub fn create_project(project_name: &str) -> Result<String> {
    // Create project storage folder if doesnt exist
    if let Err(e) = fs::create_dir_all(PROYECTOS_PATH) {
        eprintln!("Failed to create projects directory: {}", e);
    }
    // Create project file if doesnt exist
    // Técnicamente no es posible guardar dos proyectos con el mismo nombre si se usa el nombre del proyecto como nombre de archivo
    let resultado = OpenOptions::new()
        .write(true)
        .create_new(true) // Esta es la clave
        .open(construct_project_path(project_name));
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

pub fn create_task(project_name: &str, task_name: &str) {
    let resultado = OpenOptions::new()
        .write(true)
        .append(true)
        .open(construct_project_path(project_name));
    match resultado {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", task_name) {
                eprintln!("Error writing to file: {}", e);
            }
        }
        Err(e) => panic!("Ocurrió un error inesperado: {}", e),
    }
}

pub fn start_timer_on_task(project_name: &str, task_name: &str) -> Result<()> {
    match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(get_filename()) {
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