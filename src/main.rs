use clipboard::{ClipboardProvider, ClipboardContext};
use std::io::Write;

macro_rules! error {
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
        std::process::exit(1);
    })
}

fn main() {
    let mut ctx: ClipboardContext = match ClipboardProvider::new() {
        Ok(cp) => cp,
        Err(e) => error!("Error getting clipboard provider: {}", e),
    };
    match ctx.get_contents() {
        Ok(contents) => print!("{}", contents),
        Err(e) => error!("Error getting clipboard contents: {}", e),
    }
    if let Err(e) = std::io::stdout().flush() {
        error!("Error flushing stdout: {}", e)
    }
}
