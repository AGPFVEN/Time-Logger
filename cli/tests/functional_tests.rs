use rexpect::spawn;

#[test]
fn test_flujo_interactivo() -> Result<(), Box<dyn std::error::Error>> {
    // 1. OBTENER LA RUTA DEL BINARIO
    // Cargo expone automáticamente la ruta al ejecutable compilado.
    // El nombre de la variable es CARGO_BIN_EXE_ + el nombre de tu package definido en cli/Cargo.toml
    // Si tu package se llama "time-logger-cli", la variable es "CARGO_BIN_EXE_time-logger-cli"
    //let bin_path = "./target/debug/cliiii";
    let bin_path = "../target/debug/cli";

    // 2. INICIAR REXPECT
    // Usamos bin_path en lugar de escribir "target/debug/..." a mano
    let mut p = spawn(bin_path, Some(2000))?;

    // 3. TU LÓGICA DE TEST (Ejemplo hipotético)
    p.exp_regex("No argument passed to program, it will execute add line to todays file")?;
    p.send("\x1b")?;
    p.exp_eof()?;

    Ok(())
}