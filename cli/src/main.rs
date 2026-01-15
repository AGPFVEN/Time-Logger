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
use core::{utils, data_managing::text_storage};
use clap::Parser;

// automatiza --help y --version
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "./data")]
    config_path: std::path::PathBuf,
}

fn start_record_note(args: Args) {
    // Confirms that needed files exists
    text_storage::init(&args.config_path);

    // Get list of proyects
    let projects: Vec<String> = text_storage::get_projects(&args.config_path); 

    // Needed variables
    let mut selected_project: String = "".to_string();
    let mut project_tasks: Vec<String> = Vec::new();
    let mut selector: Vec<String>;
    let re = Regex::new(r"\\([0-9])$").unwrap();
    let mut input_buffer = String::new();

    // Activar modo raw
    enable_raw_mode().unwrap();
    print!(">");
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
                    let _ = execute!(
                        io::stdout(),
                        cursor::MoveTo(0, cursor::position().unwrap().1),
                        Clear(ClearType::FromCursorDown)
                    );

                    // Mostrar la línea de entrada
                    print!(">{}\r\n", input_buffer);

                    // Mostrar el buffer debajo
                    if selected_project.is_empty() {
                        selector = utils::order_vector(&input_buffer, &projects);
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

                    // Verificar si el buffer termina con "/num"
                    if let Some(caps) = re.captures(&input_buffer) {
                        let number = &caps[1].parse::<usize>().unwrap();  // This is "5"
                        //TODO: Quitar los .txt del search

                        // Read project file and populate project_tasks
                        if selected_project.is_empty() {
                            //let project_path = format!("{}/{}.txt", text_storage::PROYECTOS_PATH, &file_names[*number]);
                            selected_project = selector[*number].to_string();
                            match text_storage::get_tasks_from_project(
                                &args.config_path,
                                &selector[*number]
                            ) {
                                Ok(returned_tasks) => project_tasks = returned_tasks,
                                Err(e) => eprintln!("Failed to create project: {}", e)
                            }
                            input_buffer.clear();
                        } else {
                            match text_storage::start_timer_on_task(&args.config_path, &selected_project, &selector[*number]) {
                                Ok(()) => break,
                                Err(e) => eprintln!("Failed to create project: {}", e)
                            }
                            break;
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
                    let user_input = input_buffer.trim().to_string();

                    if selected_project.is_empty() {
                        // Ensure text_storage::PROYECTOS_PATH exists
                        match text_storage::create_project(&args.config_path,&user_input) {
                            Ok(returned_project) => selected_project = returned_project,
                            Err(e) => eprintln!("Failed to create project: {}", e)
                        }
                        //TODO: testear este caso
                    } else {
                        text_storage::create_task(&selected_project, &user_input);
                        match text_storage::start_timer_on_task(&args.config_path, &selected_project, &input_buffer.trim().to_string()) {
                            Ok(()) => break,
                            Err(e) => eprintln!("Failed to create project: {}", e)
                        }
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
                        print!("{:?}", utils::order_vector(&input_buffer, &projects));

                        // Volver al final de la línea de entrada
                        execute!(
                            io::stdout(),
                            cursor::MoveTo((2 + input_buffer.len()) as u16, cursor::position().unwrap().1 - 1)
                        ).unwrap();
                        io::stdout().flush().unwrap();
                    }
                }
                //TODO: Añadir signals para que hagan cosas (crtl+c, etc)
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

fn end_record_note(args: Args) {
    let filename_path_buf = text_storage::get_todays_filename(&args.config_path);
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
    // Get all arguments
    let args = Args::parse();

    // Construct todays filename
    let todays_file_path: PathBuf = PathBuf::from(&args.config_path)
        .join(text_storage::get_todays_filename(&args.config_path));

    // If todays file exists, is empty or complete start a new entry, else end the current note
    if !todays_file_path.exists() || fs::metadata(&todays_file_path)?.len() == 0 {
        start_record_note(args);
    } else {
        match File::open(&todays_file_path) {
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
                    start_record_note(args);
                } else {
                    end_record_note(args);
                }
            },
            Err(e) => eprintln!("Failed to open file: {}", e)
        }
    }
    std::process::exit(0);
}
