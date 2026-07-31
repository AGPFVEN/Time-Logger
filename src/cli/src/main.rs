use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use regex::Regex;
use serde::Deserialize;
use std::{
    fs,
    io::{self, ErrorKind, Write},
    panic,
};

use app_core::{
    data_managing::{Storage, TimerState},
    utils,
};
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
    /// Name of the person to greet
    #[arg(short, long, default_value = "./config.toml")]
    config_path: std::path::PathBuf,
}

fn start_record_note(storage: Box<dyn Storage>) -> Result<(), io::Error> {
    let projects: Vec<String> = storage.get_projects();

    // Needed variables
    let mut selected_project: String = "".to_string();
    let mut project_tasks: Vec<String> = Vec::new();
    let mut selector: Vec<String> = Vec::new();
    let re = Regex::new(r"\\([0-9])$").unwrap();
    let mut input_buffer = String::new();
    let mut tab_selector: Option<usize> = None;

    // Activate raw mode
    enable_raw_mode().unwrap();
    print!("> ");
    io::stdout().flush().unwrap();
    execute!(io::stdout(), cursor::SavePosition).unwrap();
    print!("\r\n{:?}", projects);
    execute!(io::stdout(), cursor::RestorePosition).unwrap();
    io::stdout().flush().unwrap();

    loop {
        // Read keyboard event
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match code {
                KeyCode::Char(c) => {
                    tab_selector = None;
                    //TODO: Make user know which project and task he is on
                    // Add character to buffer
                    input_buffer.push(c);

                    // Redraw everything
                    let _ = execute!(
                        io::stdout(),
                        cursor::MoveToColumn(0),
                        Clear(ClearType::FromCursorDown),
                        cursor::SavePosition
                    );

                    // Show input line
                    print!("> {}\r\n", input_buffer);

                    // Show buffer below input
                    if selected_project.is_empty() {
                        selector = utils::order_vector(&input_buffer, &projects);
                    } else {
                        selector = utils::order_vector(&input_buffer, &project_tasks);
                    }
                    print!("{:?}", selector);

                    // Return to end of input line
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(
                            (2 + input_buffer.len()) as u16,
                            cursor::position().unwrap().1 - 1
                        )
                    )
                    .unwrap();
                    io::stdout().flush().unwrap();

                    // Check if user ends with number
                    if let Some(caps) = re.captures(&input_buffer) {
                        let number = &caps[1].parse::<usize>().unwrap();
                        //TODO: remove .txt from project titles

                        // Read project file and populate project_tasks
                        if selected_project.is_empty() {
                            selected_project = selector[*number].to_string();
                            project_tasks = storage.get_tasks_from_project(&selector[*number]);
                            input_buffer.clear();
                        } else {
                            match storage.start_timer_on_task(&selected_project, &selector[*number])
                            {
                                Ok(()) => break,
                                Err(e) => panic!("Failed to create project: {}", e),
                            }
                        }
                    }
                }
                KeyCode::Enter => {
                    // Clean below cursor
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(0, cursor::position().unwrap().1),
                        Clear(ClearType::FromCursorDown)
                    )
                    .unwrap();

                    // Process complete line
                    print!("\r\n");
                    let user_input = input_buffer.trim().to_string();

                    if selected_project.is_empty() {
                        if tab_selector.is_none() {
                            storage.create_project(&user_input);
                            selected_project = user_input;
                        } else {
                            selected_project = selector[tab_selector.unwrap()].to_string();
                            project_tasks = storage.get_tasks_from_project(&selected_project);
                            print!("{:?}", project_tasks);
                        }
                        //TODO: test this case
                    } else {
                        if tab_selector.is_none() {
                            storage.create_task(&selected_project, &user_input);
                            match storage.start_timer_on_task(
                                &selected_project,
                                &input_buffer.trim().to_string(),
                            ) {
                                Ok(()) => break,
                                Err(e) => panic!("Failed to start timer on new task: {}", e),
                            }
                        } else {
                            match storage.start_timer_on_task(
                                &selected_project,
                                &selector[tab_selector.unwrap()].to_string(),
                            ) {
                                Ok(()) => break,
                                Err(e) => {
                                    panic!("Failed to start timer on existing task: {}", e)
                                }
                            }
                        }
                    }

                    print!("\r\n");
                    input_buffer.clear();
                    io::stdout().flush().unwrap();
                }
                KeyCode::Backspace => {
                    tab_selector = None;
                    // Delete last character
                    if !input_buffer.is_empty() {
                        input_buffer.pop();

                        //Redraw everthing
                        execute!(
                            io::stdout(),
                            cursor::MoveTo(0, cursor::position().unwrap().1),
                            Clear(ClearType::FromCursorDown)
                        )
                        .unwrap();

                        // Show input line
                        print!("> {}\r\n", input_buffer);

                        // Show projects below
                        // TODO: this should be refactored because it is used a lot
                        if selected_project.is_empty() {
                            selector = utils::order_vector(&input_buffer, &projects);
                        } else {
                            selector = utils::order_vector(&input_buffer, &project_tasks);
                        }
                        print!("{:?}", selector);

                        // Return to end of line
                        execute!(
                            io::stdout(),
                            cursor::MoveTo(
                                (2 + input_buffer.len()) as u16,
                                cursor::position().unwrap().1 - 1
                            )
                        )
                        .unwrap();
                        io::stdout().flush().unwrap();
                    }
                }
                KeyCode::Tab => {
                    // Redraw everything
                    let _ = execute!(
                        io::stdout(),
                        cursor::MoveTo(0, cursor::position().unwrap().1),
                        Clear(ClearType::FromCursorDown)
                    );

                    // Show input line
                    print!(">{}\r\n", input_buffer);

                    // Show selector below
                    if selected_project.is_empty() {
                        selector = utils::order_vector(&input_buffer, &projects);
                    } else {
                        selector = utils::order_vector(&input_buffer, &project_tasks);
                    }
                    if tab_selector == None {
                        tab_selector = Some(0);
                    } else {
                        if tab_selector.unwrap() >= selector.len() - 1 {
                            tab_selector = Some(0);
                        } else {
                            tab_selector = Some(tab_selector.unwrap() + 1);
                        }
                    }

                    for (i, item) in selector.iter().enumerate() {
                        if Some(i) == tab_selector {
                            // Highlighted item
                            let _ = execute!(
                                io::stdout(),
                                crossterm::style::SetAttribute(
                                    crossterm::style::Attribute::Reverse
                                ),
                                Print(item),
                                crossterm::style::SetAttribute(
                                    crossterm::style::Attribute::NoReverse
                                )
                            );
                        } else {
                            // Normal item
                            print!("{}", item);
                        }
                        if i != selector.len() - 1 {
                            print!(", ");
                        }
                    }

                    // Come back to end of line
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(
                            (2 + input_buffer.len()) as u16,
                            cursor::position().unwrap().1 - 1
                        )
                    )
                    .unwrap();
                    io::stdout().flush().unwrap();
                }
                KeyCode::BackTab => {
                    // Redraw everything
                    let _ = execute!(
                        io::stdout(),
                        cursor::MoveTo(0, cursor::position().unwrap().1),
                        Clear(ClearType::FromCursorDown)
                    );

                    //Show input line
                    print!(">{}\r\n", input_buffer);

                    // Show selector below
                    if selected_project.is_empty() {
                        selector = utils::order_vector(&input_buffer, &projects);
                    } else {
                        selector = utils::order_vector(&input_buffer, &project_tasks);
                    }
                    if tab_selector == None || tab_selector.unwrap() == 0 {
                        tab_selector = Some(selector.len() - 1);
                    } else {
                        tab_selector = Some(tab_selector.unwrap() - 1);
                    }

                    for (i, item) in selector.iter().enumerate() {
                        if Some(i) == tab_selector {
                            // Highlighted item
                            let _ = execute!(
                                io::stdout(),
                                crossterm::style::SetAttribute(
                                    crossterm::style::Attribute::Reverse
                                ),
                                Print(item),
                                crossterm::style::SetAttribute(
                                    crossterm::style::Attribute::NoReverse
                                ),
                            );
                        } else {
                            // Normal item
                            print!("{}", item);
                        }
                        if i != selector.len() - 1 {
                            print!(", ");
                        }
                    }

                    // Come back to end of line
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(
                            (2 + input_buffer.len()) as u16,
                            cursor::position().unwrap().1 - 1
                        )
                    )
                    .unwrap();
                    io::stdout().flush().unwrap();
                }
                //TODO: Add signals support (crtl+c, etc) (or avoid raw terminal handling)
                KeyCode::Esc => {
                    let _ = execute!(
                        io::stdout(),
                        cursor::RestorePosition,
                        Clear(ClearType::FromCursorDown),
                    );
                    let _ = io::stdout().flush();
                    println!("> Saliendo del programa...\r");
                    break;
                }
                _ => {}
            }
            let _ = execute!(io::stdout(), cursor::RestorePosition);
            let _ = io::stdout().flush();
        }
    }

    disable_raw_mode().unwrap();
    Ok(())
}

fn end_record_note(storage: Box<dyn Storage>, time_entry_id: i32) -> Result<(), io::Error> {
    enable_raw_mode().unwrap();

    let mut input_buffer = String::new();

    print!("> ");
    io::stdout().flush().unwrap();
    loop {
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match code {
                KeyCode::Char(c) => {
                    input_buffer.push(c);

                    // Redraw everything
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(0, cursor::position().unwrap().1),
                        Clear(ClearType::FromCursorDown)
                    )
                    .unwrap();

                    //Show input line
                    print!("> {}\r\n", input_buffer);

                    // Come back to end of line
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(
                            (2 + input_buffer.len()) as u16,
                            cursor::position().unwrap().1 - 1
                        )
                    )
                    .unwrap();
                    io::stdout().flush().unwrap();

                    if input_buffer.ends_with("\\q") {
                        print!("\r\n\r\n");
                        println!("Quitting program ...\r");
                        break;
                    }
                }
                KeyCode::Enter => {
                    // Clean code below cursor
                    execute!(
                        io::stdout(),
                        cursor::MoveTo(0, cursor::position().unwrap().1),
                        Clear(ClearType::FromCursorDown)
                    )
                    .unwrap();

                    // Process line
                    print!("\r\n");

                    match storage.end_timer_on_task(&time_entry_id, &input_buffer) {
                        Ok(()) => break,
                        Err(e) => panic!("Failed to stop time entry: {}", e),
                    }
                }
                KeyCode::Backspace => {
                    // Delete last character
                    if !input_buffer.is_empty() {
                        input_buffer.pop();

                        // Redraw everything
                        execute!(
                            io::stdout(),
                            cursor::MoveTo(0, cursor::position().unwrap().1),
                            Clear(ClearType::FromCursorDown)
                        )
                        .unwrap();

                        //Show input line
                        print!("> {}\r\n", input_buffer);

                        // Come back to end of line
                        execute!(
                            io::stdout(),
                            cursor::MoveTo(
                                (2 + input_buffer.len()) as u16,
                                cursor::position().unwrap().1 - 1
                            )
                        )
                        .unwrap();
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

    disable_raw_mode().unwrap();
    Ok(())
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
        StorageConfig::Sqlite {database_url} => {
            Box::new(app_core::data_managing::data_sqlite::SqliteStorage::init(&database_url))
        }
    };

    match storage_obj.get_timer_state() {
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
    }
    std::process::exit(0);
}
