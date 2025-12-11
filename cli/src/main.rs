use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::{self, Write};
use std::path::PathBuf;
use chrono::prelude::*;
use regex::Regex;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor,
    execute,
};
use core::{utils};

const PROYECTOS_PATH: &str = "./data/Proyectos";

fn get_filename_path() -> PathBuf {
    let now = Local::now();
    let week = now.iso_week().week();
    let year = now.year();
    let folder_path = format!("./data/Semanas anteriores/W{} {}", week, year);
    fs::create_dir_all(&folder_path).expect("Failed to create directory");
    let filename = format!("{}/{}.txt", folder_path, now.format("%d-%m-%Y"));
    PathBuf::from(filename)
}

fn start_record_note() {
    // Check if file exists, if not create it
    let filename_path_buf = get_filename_path();
    let filename_path = filename_path_buf.as_path();

    if !filename_path.exists() {
        println!("File does not exist, creating it...");
        if let Err(e) = fs::write(filename_path, "") {
            println!("Error creating file: {}", e);
        } else {
            println!("File created sucessfully");
        }
    }

    // Get list of proyects
    let mut file_names: Vec<String> = Vec::new();
    match fs::read_dir(PROYECTOS_PATH){
        Ok(entries) => {
            file_names = entries
                .filter_map(|entry| {
                        entry.ok()
                            .and_then(|e| e.file_name().into_string().ok())
                    })
                .collect();
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
        }
    }

    // Empezar programa para hacer una línea
    println!("Escribe '/q' para salir del programa");
    println!("Modo raw activado - lee carácter por carácter\r");

    let mut selected_project: Option<File> = None;
    let mut selected_project_name: Option<String> = None;
    let mut selected_task: Option<String> = None;
    let mut project_tasks: Vec<String> = Vec::new();
    let mut selector: Vec<String>;
    let re = Regex::new(r"\\([0-9])$").unwrap();

    // Activar modo raw
    enable_raw_mode().unwrap();

    let mut input_buffer = String::new();

    print!("> ");
    io::stdout().flush().unwrap();

    loop {
        // Leer evento del teclado
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match code {
                KeyCode::Char(c) => {
                    //TODO: Meter un prompt de qué proyecto está el usuario
                    // Agregar carácter al buffer
                    input_buffer.push(c);

                    // Redibujar todo
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(0, cursor::position().unwrap().1),
                        Clear(ClearType::FromCursorDown)
                    ).unwrap();

                    // Mostrar la línea de entrada
                    print!("> {}\r\n", input_buffer);

                    // Mostrar el buffer debajo
                    if selected_project.is_none() {
                        selector = utils::order_vector(&input_buffer, &file_names);
                    } else {
                        selector = utils::order_vector(&input_buffer, &project_tasks);
                    }
                    print!("{:?}", selector);

                    // Volver al final de la línea de entrada
                    execute!(
                        io::stdout(),
                        cursor::MoveTo((2 + input_buffer.len()) as u16, cursor::position().unwrap().1 - 1)
                    ).unwrap();
                    io::stdout().flush().unwrap();

                    // Verificar si el buffer termina con "/q"
                    if input_buffer.ends_with("\\q") {
                        print!("\r\n\r\n");
                        println!("Saliendo del programa...\r");
                        break;
                    }

                    // Verificar si el buffer termina con "/q"
                    if let Some(caps) = re.captures(&input_buffer) {
                        let number = &caps[1].parse::<usize>().unwrap();  // This is "5"
                        //TODO: Quitar los .txt del search
                        //let project_path = format!("{}/{}.txt", PROYECTOS_PATH, &file_names[*number]);
                        let project_path = format!("{}/{}", PROYECTOS_PATH, &selector[*number]);

                        // Read project file and populate project_tasks
                        if selected_project.is_none() {
                            match fs::read_to_string(&project_path) {
                                Ok(content) => {
                                    project_tasks = content.lines()
                                        .map(|line| line.to_string())
                                        .collect();
                                },
                                Err(e) => eprintln!("Failed to read project file: {}", e)
                            }

                            // Open the file for writing (append mode)
                            //TODO: No crear una referencia
                            match OpenOptions::new()
                                .write(true)
                                .append(true)
                                .open(&project_path) {
                                Ok(file) => {
                                    selected_project = Some(file);
                                    selected_project_name = Some(selector[*number].to_string());
                                },
                                Err(e) => eprintln!("Failed to open file: {}", e)
                            }

                            input_buffer.clear();
                        } else if selected_task.is_none() {
                            selected_task = Some(selector[*number].to_string());
                        }
                    }
                }
                KeyCode::Enter => {

                    // Limpiar todo desde el cursor hacia abajo
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(0, cursor::position().unwrap().1),
                        Clear(ClearType::FromCursorDown)
                    ).unwrap();

                    // Procesar la línea completa
                    print!("\r\n");

                    if selected_project.is_none() {
                        // Ensure PROYECTOS_PATH exists
                        if let Err(e) = fs::create_dir_all(PROYECTOS_PATH) {
                            eprintln!("Failed to create projects directory: {}", e);
                        }

                        let project_name = input_buffer.trim().to_string() + ".txt";
                        let project_path = PROYECTOS_PATH.to_string() + "/" + &project_name;
                        match OpenOptions::new()
                            .write(true)
                            .append(true)
                            .create(true)
                            .open(&project_path) {
                            Ok(file) => {
                                selected_project = Some(file);
                                selected_project_name = Some(project_name);
                            },
                            Err(e) => eprintln!("Failed to create file: {}", e)
                        }
                        //TODO: testear este caso
                    } else if selected_task.is_none() {
                        let task_to_save = input_buffer.trim();
                        project_tasks.push(task_to_save.to_string());

                        if let Some(ref mut file) = selected_project {
                            if let Err(e) = writeln!(file, "{}", task_to_save) {
                                eprintln!("Error writing to file: {}", e);
                            }
                        }

                        selected_task = Some(task_to_save.to_string());
                    }

                    input_buffer.clear();
                    print!("> ");
                    io::stdout().flush().unwrap();
                }
                KeyCode::Backspace => {
                    // Borrar último carácter
                    if !input_buffer.is_empty() {
                        input_buffer.pop();

                        // Redibujar todo
                        execute!(
                            io::stdout(),
                            cursor::MoveTo(0, cursor::position().unwrap().1),
                            Clear(ClearType::FromCursorDown)
                        ).unwrap();

                        // Mostrar la línea de entrada
                        print!("> {}\r\n", input_buffer);

                        // Mostrar el buffer debajo
                        print!("{:?}", utils::order_vector(&input_buffer, &file_names));

                        // Volver al final de la línea de entrada
                        execute!(
                            io::stdout(),
                            cursor::MoveTo((2 + input_buffer.len()) as u16, cursor::position().unwrap().1 - 1)
                        ).unwrap();
                        io::stdout().flush().unwrap();
                    }
                }
                KeyCode::Esc => {
                    print!("\r\n\r\n");
                    println!("Saliendo del programa...\r");
                    break;
                }
                _ => {}
            }

            if !selected_task.is_none() {
                match OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(&filename_path) {
                        Ok(mut file) => {
                            if let Err(e) = write!(file, "{} {}_{} (",
                                Local::now().format("%H:%M"),
                                selected_project_name.as_ref().unwrap().replace(" ", "-").replace(".txt", ""),
                                selected_task.as_ref().unwrap().replace(" ", "-")
                            ) {
                                eprintln!("Error writing to file: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error writing to file: {}", e);
                        }
                        
                    }
                break;
            }
        }
    }

    // Desactivar modo raw al salir
    disable_raw_mode().unwrap();

}

fn end_record_note() {
    let filename_path_buf = get_filename_path();
    let filename_path = filename_path_buf.as_path();
    // Activar modo raw
    enable_raw_mode().unwrap();

    let mut input_buffer = String::new();

    print!("> ");
    io::stdout().flush().unwrap();
    loop {
        // Leer evento del teclado
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match code {
                KeyCode::Char(c) => {
                    // Agregar carácter al buffer
                    input_buffer.push(c);

                    // Redibujar todo
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(0, cursor::position().unwrap().1),
                        Clear(ClearType::FromCursorDown)
                    ).unwrap();

                    // Mostrar la línea de entrada
                    print!("> {}\r\n", input_buffer);

                    // Volver al final de la línea de entrada
                    execute!(
                        io::stdout(),
                        cursor::MoveTo((2 + input_buffer.len()) as u16, cursor::position().unwrap().1 - 1)
                    ).unwrap();
                    io::stdout().flush().unwrap();

                    // Verificar si el buffer termina con "/q"
                    if input_buffer.ends_with("\\q") {
                        print!("\r\n\r\n");
                        println!("Saliendo del programa...\r");
                        break;
                    }
                }
                KeyCode::Enter => {
                    // Limpiar todo desde el cursor hacia abajo
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(0, cursor::position().unwrap().1),
                        Clear(ClearType::FromCursorDown)
                    ).unwrap();

                    // Procesar la línea completa
                    print!("\r\n");

                        match OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(&filename_path) {
                            Ok(mut file) => {
                                match writeln!(file,
                                    "{}) {}",
                                    input_buffer,
                                    Local::now().format("%H:%M")
                                ){
                                    Ok(()) => {}
                                    Err(e) => eprintln!("Failed to write in file: {}", e)
                                }
                            },
                            Err(e) => eprintln!("Failed to create file: {}", e)
                        }

                    input_buffer.clear();
                    print!("> ");
                    io::stdout().flush().unwrap();
                    break;
                }
                KeyCode::Backspace => {
                    // Borrar último carácter
                    if !input_buffer.is_empty() {
                        input_buffer.pop();

                        // Redibujar todo
                        execute!(
                            io::stdout(),
                            cursor::MoveTo(0, cursor::position().unwrap().1),
                            Clear(ClearType::FromCursorDown)
                        ).unwrap();

                        // Mostrar la línea de entrada
                        print!("> {}\r\n", input_buffer);

                        // Volver al final de la línea de entrada
                        execute!(
                            io::stdout(),
                            cursor::MoveTo((2 + input_buffer.len()) as u16, cursor::position().unwrap().1 - 1)
                        ).unwrap();
                        io::stdout().flush().unwrap();
                    }

                }
                KeyCode::Esc => {
                    print!("\r\n\r\n");
                    println!("Saliendo del programa...\r");
                    break;
                }
                _ => {}
            }
        }
    }

    // Desactivar modo raw al salir
    disable_raw_mode().unwrap();

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No argument passed to program, it will execute add line to todays file");
        args.push("-a".to_string());
    }

    match args[1].as_str() {
        _=>{
            let filename_path_buf = get_filename_path();
            let filename_path = filename_path_buf.as_path();
            if !filename_path.exists() || fs::metadata(filename_path)?.len() == 0 {
                start_record_note();
            } else {
                match File::open(filename_path) {
                    Ok(mut file) => {
                        use std::io::Seek;
                        use std::io::SeekFrom;

                        // Seek to 2 bytes before the end
                        file.seek(SeekFrom::End(-1))?;
    
                        // Read the last 2 bytes
                        let mut buffer = vec![0u8; 1];
                        file.read_exact(&mut buffer)?;
    
                        println!("Byte 0: {} -> char: '{}'", buffer[0], buffer[0] as char);
                        if buffer[0] == 10 {
                            start_record_note();
                        } else {
                            end_record_note();
                        }
                    },
                    Err(e) => eprintln!("Failed to open file: {}", e)
                }
            }
        }
    }
    Ok(())
}
