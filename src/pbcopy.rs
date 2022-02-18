use std::error::Error;
use std::io::Read;

use nix::unistd::{fork, ForkResult};
use x11_clipboard::Clipboard;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input)?;

    // We have to fork a child process otherwise the clipboard is cleared when our process exits
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let clipboard = Clipboard::new()?;

            clipboard.store(
                clipboard.setter.atoms.clipboard,
                clipboard.setter.atoms.utf8_string,
                input,
            )?;

            // Wait for the clipboard to change, so we can end the process
            clipboard.load_wait(
                clipboard.getter.atoms.clipboard,
                clipboard.getter.atoms.utf8_string,
                clipboard.getter.atoms.property,
            )?;

            Ok(())
        }
        _ => Ok(()),
    }
}
