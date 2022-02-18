use std::error::Error;
use std::io::Write;
use std::os::unix::prelude::AsRawFd;
use std::str::from_utf8;
use std::time::Duration;

use nix::unistd::isatty;
use x11_clipboard::Clipboard;

fn main() -> Result<(), Box<dyn Error>> {
    let clipboard = Clipboard::new()?;

    let contents = clipboard.load(
        clipboard.getter.atoms.clipboard,
        clipboard.getter.atoms.utf8_string,
        clipboard.getter.atoms.property,
        Duration::from_secs(1),
    )?;

    let stdout = std::io::stdout();

    if isatty(stdout.as_raw_fd()).unwrap_or(false) {
        match from_utf8(&contents) {
            Ok(string) if string.ends_with("\n") => print!("{string}"),
            Ok(string) => println!("{string}"),
            Err(_) => panic!("Can't write non utf8 encoded content to the terminal"),
        }
    } else {
        stdout.lock().write_all(&contents)?;
    };

    Ok(())
}
