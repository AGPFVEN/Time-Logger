// main.rs - Real-time key capture (HFT-style)
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    ExecutableCommand,
};
use std::io::stdout;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut timers: Vec<(Instant, Option<u128>)> = Vec::new();
    let mut current_timer: Option<Instant> = None;

    println!("Commands:");
    println!("  SPACE - Start/Stop timer");
    println!("  'l' - List all timers");
    println!("  'q' - Quit\r");

    loop {
        // Non-blocking - this is key for HFT!
        if event::poll(std::time::Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char(' ') => {
                        match current_timer {
                            None => {
                                // START: Capture timestamp immediately
                                current_timer = Some(Instant::now());
                                println!("\r\n⏱️  Timer started!");
                            }
                            Some(start) => {
                                // STOP: Calculate duration
                                let elapsed = start.elapsed().as_micros();
                                timers.push((start, Some(elapsed)));
                                current_timer = None;
                                println!("\r\n⏸️  Timer stopped: {} μs", elapsed);
                            }
                        }
                    }
                    KeyCode::Char('l') => {
                        println!("\r\n📊 Completed timers:");
                        for (i, (_, duration)) in timers.iter().enumerate() {
                            if let Some(d) = duration {
                                println!("  #{}: {} μs", i + 1, d);
                            }
                        }
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
        
        // Could do other work here without blocking!
        // This is the HFT principle - never block
    }
    
    // Cleanup
    terminal::disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    
    Ok(())
}
