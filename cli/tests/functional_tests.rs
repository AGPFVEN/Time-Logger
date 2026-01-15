use std::{path::PathBuf, thread, time};
use anyhow::{Ok, Result};
use rexpect::spawn;
use tempfile::tempdir;
//use indoc::indoc;


#[test]
fn open_close() -> Result<()> {
    // Create the isolated temporary directory
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    // Binary path
    let bin_path = "../target/debug/cli";

    // Construct the command
    let cmd = format!("{} --config-path \"{}\"", bin_path, temp_path.display());

    // Spawn the process with the new command string
    let mut p = spawn(&cmd, Some(10_000))?;
    thread::sleep(time::Duration::from_millis(200));
    p.send("\x1b")?;
    p.flush()?;

    // Check if the required folders exist
    if temp_path.join(PathBuf::from("Projects")).exists()
    && temp_path.join(PathBuf::from("Weeks")).exists() {
        return Ok(());
    } else {
        Err(anyhow::anyhow!("Required folders not found"))
    }
}
/* Un test jodido con p.exp_string() y p.send()
#[test]
fn test_flujo_interactivo() -> Result<()> {
    // 1. OBTENER LA RUTA DEL BINARIO
    // Cargo expone automáticamente la ruta al ejecutable compilado.
    // El nombre de la variable es CARGO_BIN_EXE_ + el nombre de tu package definido en cli/Cargo.toml
    // Si tu package se llama "time-logger-cli", la variable es "CARGO_BIN_EXE_time-logger-cli"
    //let bin_path = "./target/debug/cliiii";
    let bin_path = "../target/debug/cli";
    let mut p = spawn(bin_path, Some(10000))?;

    // 3. TU LÓGICA DE TEST (Ejemplo hipotético)
    p.exp_string(&indoc!{"No argument passed to program, it will execute add line to todays file
        Todays file does not exist, creating it...
        Todays file created sucessfully
        Proyect folder not found at './data/Proyectos', creating it ...
        Proyect folder created successfully.
        >"}.replace("\n", "\r\n")
    )?;
    // ESPERA: Dale 100ms o 200ms para asegurar que el input loop está listo
    p.send("\x1b")?;
    // El flush es para que el programa lea el esc
    p.flush()?;
    p.exp_string(&indoc!{"
        Saliendo del programa..."}.replace("\n", "\r\n")
    )?;
    p.exp_eof()?;
    // ... inside your test ...

    // Fail the test intentionally so you can read the logs
    return Ok(());
}
*/