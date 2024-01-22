use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

fn flush_content_to_terminal(stdout: &mut Stdout, content: &String) {
    execute!(stdout, Clear(ClearType::All)).expect("failed clearing");
    execute!(stdout, MoveTo(0, 0)).expect("error moving");
    write!(stdout, "{}", content).expect("failed writing");
    stdout.flush().expect("failed in startup");
}

fn some_fn(some: &mut String) {
    print!("{some}");
}

fn main() -> std::io::Result<()> {
    enable_raw_mode().expect("failed enable raw mode");

    let stdout = stdout();
    let mut content = String::new();
    execute!(&stdout, Clear(ClearType::All)).expect("failed clearing");
    execute!(&stdout, MoveTo(0, 0)).expect("error moving");

    let some_s = String::new();
    some_fn(&mut some_s);

    loop {
        if let Event::Key(event) = event::read()? {
            match event {
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: event::KeyModifiers::CONTROL,
                    ..
                } => break,
                KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                } => {
                    content.push(c);
                }
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    content.pop();
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    content += "\r\n";
                }
                KeyEvent {
                    code: KeyCode::Tab, ..
                } => {
                    content += "\t";
                }
                _ => {
                    //todo
                }
            }

            flush_content_to_terminal(&stdout, &content)
        };
    }

    disable_raw_mode().expect("failed disable raw mode");
    Ok(())
}
