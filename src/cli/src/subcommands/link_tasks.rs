use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

use app_core::{data_managing::Storage, utils};

pub fn link_tasks(storage: Box<dyn Storage>) -> Result<(), io::Error> {
    let projects: Vec<String> = storage.get_projects();

    // Needed variables
    let mut selected_project: String = "".to_string();
    let mut selected_parent_task: String = "".to_string();
    let mut project_tasks: Vec<String> = Vec::new();
    let mut selector: Vec<String> = Vec::new();
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

                    if selected_project.is_empty() {
                        if tab_selector.is_none() {
                            print!("To link tasks a project must be selected")
                        } else {
                            selected_project = selector[tab_selector.unwrap()].to_string();
                            project_tasks = storage.get_tasks_from_project(&selected_project);
                            print!("{:?}", project_tasks);
                        }
                    } else if selected_parent_task.is_empty() {
                        if tab_selector.is_none() {
                            print!("To link tasks a task must be selected");
                        } else {
                            selected_parent_task = selector[tab_selector.unwrap()].to_string();
                            print!("parent task selected: {:?}", selected_parent_task);
                        }
                    } else {
                        if tab_selector.is_none() {
                            print!("To link tasks a task must be selected");
                        } else {
                            storage.link_task_2_task(
                                &selected_project,
                                &selected_parent_task,
                                &selector[tab_selector.unwrap()].to_string(),
                            );
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
