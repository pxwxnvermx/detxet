use std::{
    env,
    io::{stdout, Stdout, Write},
};

use crossterm::{
    cursor::{MoveLeft, MoveRight, MoveTo},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

struct Editor {
    stdout: Stdout,
    content: String,
    cursor_pos: (u16, u16),
}

trait EditorCommands {
    fn flush_content_to_terminal(&mut self);
    fn clear_screen(&mut self);
    fn move_cursor(&mut self);
}

impl EditorCommands for Editor {
    fn flush_content_to_terminal(&mut self) {
        self.clear_screen();
        write!(self.stdout, "{}", self.content).expect("failed writing");
        self.move_cursor();
        self.stdout.flush().expect("failed in startup");
    }

    fn clear_screen(&mut self) {
        execute!(self.stdout, Clear(ClearType::All)).expect("failed clearing");
    }

    fn move_cursor(&mut self) {
        execute!(self.stdout, MoveTo(self.cursor_pos.0, self.cursor_pos.1)).expect("error moving");
    }
}

/*
* TODO: Opening files
* TODO: Saving a file
* */

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    enable_raw_mode().expect("failed enable raw mode");
    let mut editor = Editor {
        stdout: stdout(),
        content: String::new(),
        cursor_pos: (0, 0),
    };

    editor.clear_screen();
    // event loop
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
                    editor.content.push(c);
                    editor.cursor_pos.0 += 1;
                }
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    editor.content.pop();
                    editor.cursor_pos.0 -= 1;
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    editor.content.push_str("\r\n");
                    editor.cursor_pos.1 += 1;
                }
                KeyEvent {
                    code: KeyCode::Tab, ..
                } => {
                    editor.content.push_str("\t");
                }
                KeyEvent {
                    code: KeyCode::Left,
                    ..
                } => {
                    editor.cursor_pos.0 -= 1;
                    execute!(editor.stdout, MoveLeft(1)).expect("failed moving left");
                }
                KeyEvent {
                    code: KeyCode::Right,
                    ..
                } => {
                    editor.cursor_pos.0 += 1;
                    execute!(editor.stdout, MoveRight(1)).expect("failed moving right");
                }
                _ => {
                    //todo
                }
            }
            editor.flush_content_to_terminal()
        };
    }

    disable_raw_mode().expect("failed disable raw mode");
    Ok(())
}
