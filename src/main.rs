use std::fs;
use std::io::{self, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // Open a file or create a new one
    let file_path = "example.txt";
    let mut content = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(_) => String::new(),
    };

    // Initialize terminal and enter raw mode
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = io::stdin();

    // Print initial file content
    write!(stdout, "{}", content).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Char('\n') => {
                content.push('\n');
            }
            termion::event::Key::Char(c) => {
                content.push(c);
            }
            termion::event::Key::Ctrl('q') => {
                break;
            }
            _ => {}
        }

        // Clear the screen and print the updated content
        write!(stdout, "{}{}", termion::clear::All, content).unwrap();
        stdout.flush().unwrap();
    }

    // Save the file before exiting
    fs::write(file_path, content).expect("Failed to save the file");
}
