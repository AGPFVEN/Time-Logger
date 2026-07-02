use anyhow::{Context, Result};
use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::process;
use std::{fs, path::PathBuf};
use toml::Value;

const WEEKS_PATH: &str = "Weeks";
const PROYECTS_PATH: &str = "Projects";

struct TextStorage {
    data_path: PathBuf,
    todays_file: PathBuf,
}

impl TextStorage {
    pub fn new(config_path: PathBuf) -> Self {
        // Set up text storage config values

        // Set up todays file path
        let todays_file = Self::get_todays_filename(&data_path);

        // Check if todays file exists, if not create it
        if !todays_file.exists() {
            println!("Todays file does not exist, creating it...");
            if let Err(e) = fs::write(&todays_file, "") {
                eprintln!("Error creating file: {}", e);
                process::exit(1);
            } else {
                println!("Todays file created sucessfully");
            }
        }

        // Construir y retornar la instancia al final
        Self {
            data_path: config_path,
            todays_file,
        }
    }

    fn read_config(config_file_path: &PathBuf) -> () {
        let config_file_content = fs::read(config_file_path)
            .expect("Error reading config file");
        let config_values: Value = config_file_content.parse()
            .expect("Error parsing TOML config file");
        if let Some(port) = config_values["server"]["port"].as_integer() {
            println!("El puerto configurado es: {}", port);
        } else {
            println!("No se encontró el puerto o no es un número entero.");
        }
    }

    }

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

        // It also creates the needed folders if they doesnt exist
        fs::create_dir_all(&folder_path).expect("Failed to create directory");
        let filename = format!("{}/{}.txt", folder_path, now.format("%d-%m-%Y"));
        PathBuf::from(filename)
    }
}

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
        return PathBuf::from(format!(
            "{}/{}.txt",
            data_path.join(PROYECTS_PATH).display(),
            project_name
        ));
    }
}

// TODO: Usar un search para no traer todos proyectos
pub fn get_projects(data_path: &PathBuf) -> Vec<String> {
    match fs::read_dir(data_path.join(PROYECTS_PATH)) {
        // Get all entries of dir
        Ok(entries) => entries
            .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect(),
        // If an errors pop, show it
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    //TODO: Ask to create before creating it
                    println!(
                        "Proyect folder not found at '{}', creating it ...",
                        data_path.join(PROYECTS_PATH).display()
                    );
                    if let Err(create_error) = fs::create_dir_all(data_path.join(PROYECTS_PATH)) {
                        eprintln!(
                            "Failed to create the proyect folder, Reason: {}",
                            create_error
                        );
                    } else {
                        println!("Proyect folder created successfully.");
                    }
                    Vec::new()
                }
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
    let project_path = format!(
        "{}/{}",
        data_path.join(PROYECTS_PATH).display(),
        project_name
    );

    // El operador '?' reemplaza todo el match.
    // Si falla, devuelve el error con el contexto que añadimos.
    let content = fs::read_to_string(&project_path)
        .with_context(|| format!("No se pudo leer el archivo del proyecto: {}", project_path))?;

    let project_tasks: Vec<String> = content.lines().map(|line| line.to_string()).collect();

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
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            return Err(anyhow::anyhow!(
                "El archivo ya existe, así que no hice nada."
            ));
        }
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
        .open(get_todays_filename(data_path))
    {
        Ok(mut file) => {
            if let Err(e) = write!(
                file,
                "{} {}_{} (",
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

pub fn end_timer_on_task(
    data_path: &PathBuf,
    project_name: &str,
    task_name: &str,
    input_buffer: &str,
) -> Result<()> {
    let filename_path = get_todays_filename(data_path);

    // Si falla al abrir, salta del bloque y devuelve el Err(anyhow) automáticamente
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filename_path)
        .with_context(|| format!("Failed to create file: {}", filename_path.display()))?;

    // Si falla al escribir, hace exactamente lo mismo
    writeln!(file, "TU_TEXTO_AQUI").with_context(|| "Failed to write in file")?;

    Ok(())
}
