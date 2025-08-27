use std::io::{stdout, Write, IsTerminal, Read};
use std::env;
use std::thread;
use std::sync::mpsc;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
    cursor,
    style::{self, Color, SetForegroundColor, ResetColor, Attribute},
};
use vte::{Parser, Perform};
use pty_process::blocking::{Command, Pty};

struct MyVteParser;

impl Perform for MyVteParser {
    fn print(&mut self, c: char) {
        print!("{}", c);
        stdout().flush().unwrap();
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => print!("\r\n"),
            b'\r' => print!("\r"),
            _ => {}
        }
    }

    fn csi_dispatch(&mut self, params: &vte::Params, intermediates: &[u8], ignore: bool, c: char) {
        if ignore || intermediates.len() > 0 {
            return;
        }

        match c {
            'm' => {
                for param in params.iter() {
                    match param[0] {
                        0 => execute!(stdout(), ResetColor).unwrap(),
                        30 => execute!(stdout(), SetForegroundColor(Color::Black)).unwrap(),
                        31 => execute!(stdout(), SetForegroundColor(Color::Red)).unwrap(),
                        32 => execute!(stdout(), SetForegroundColor(Color::Green)).unwrap(),
                        33 => execute!(stdout(), SetForegroundColor(Color::Yellow)).unwrap(),
                        34 => execute!(stdout(), SetForegroundColor(Color::Blue)).unwrap(),
                        35 => execute!(stdout(), SetForegroundColor(Color::Magenta)).unwrap(),
                        36 => execute!(stdout(), SetForegroundColor(Color::Cyan)).unwrap(),
                        37 => execute!(stdout(), SetForegroundColor(Color::White)).unwrap(),
                        1 => execute!(stdout(), style::SetAttribute(Attribute::Bold)).unwrap(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() -> std::io::Result<()> {
    if !stdout().is_terminal() {
        eprintln!("This application requires an interactive terminal.");
        return Ok(())
    }

    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    let mut line = String::new();

    loop {
        execute!(stdout, cursor::MoveToColumn(0), Clear(ClearType::CurrentLine))?;
        let current_dir = env::current_dir()?.to_string_lossy().to_string();
        print!("{}> {}", current_dir, line);
        stdout.flush()?;

        if let Event::Key(event) = event::read()? {
            match event {
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    break;
                }
                KeyEvent { code, .. } => {
                    match code {
                        KeyCode::Char(c) => {
                            line.push(c);
                        }
                        KeyCode::Backspace => {
                            line.pop();
                        }
                        KeyCode::Enter => {
                            let mut parts = line.trim().split_whitespace();
                            let command = match parts.next() {
                                Some(c) => c,
                                None => "",
                            };
                            let args = parts;

                            if command.is_empty() {
                                print!("\r\n");
                                continue;
                            }

                            if command == "exit" {
                                break;
                            }

                            if command == "cd" {
                                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                                if let Err(e) = env::set_current_dir(new_dir) {
                                    println!("\r\nError: {}", e);
                                }
                                line.clear();
                                continue;
                            }

                            let pty = Pty::new().unwrap();
                            let pts = pty.pts().unwrap();
                            let mut cmd = Command::new(command);
                            cmd.args(args);
                            
                            match cmd.spawn(&pts) {
                                Ok(mut child) => {
                                    print!("\r\n");
                                    drop(pts);

                                    let (tx, rx) = mpsc::channel();
                                    let mut pty_reader = pty;
                                    let read_thread = thread::spawn(move || {
                                        let mut buffer = [0u8; 1024];
                                        let mut parser = Parser::new();
                                        let mut performer = MyVteParser;
                                        loop {
                                            if rx.try_recv().is_ok() {
                                                break;
                                            }
                                            let n = pty_reader.read(&mut buffer).unwrap();
                                            if n == 0 {
                                                break;
                                            }
                                            for byte in &buffer[..n] {
                                                parser.advance(&mut performer, *byte);
                                            }
                                        }
                                    });

                                    child.wait().unwrap();
                                    tx.send(()).unwrap();
                                    read_thread.join().unwrap();
                                }
                                Err(e) => {
                                    println!("\r\nError: {}", e);
                                }
                            }
                            line.clear();
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    terminal::disable_raw_mode()
}