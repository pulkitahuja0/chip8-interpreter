use std::{
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    ExecutableCommand,
    event::{Event, KeyCode, KeyEvent, KeyEventKind, poll, read},
    terminal,
};

pub struct Hardware {
    stdout: Stdout,
}

fn value_to_char(value: u8) -> Result<char, &'static str> {
    match value {
        0 => Ok('0'),
        1 => Ok('1'),
        2 => Ok('2'),
        3 => Ok('3'),
        4 => Ok('4'),
        5 => Ok('5'),
        6 => Ok('6'),
        7 => Ok('7'),
        8 => Ok('8'),
        9 => Ok('9'),
        0xA => Ok('a'),
        0xB => Ok('b'),
        0xC => Ok('c'),
        0xD => Ok('d'),
        0xE => Ok('e'),
        0xF => Ok('f'),
        _ => Err("Invalid value used for character"),
    }
}

fn char_to_value(c: char) -> Result<u8, &'static str> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'a' | 'A' => Ok(0xA),
        'b' | 'B' => Ok(0xB),
        'c' | 'C' => Ok(0xC),
        'd' | 'D' => Ok(0xD),
        'e' | 'E' => Ok(0xE),
        'f' | 'F' => Ok(0xF),
        _ => Err("Invalid character used for value"),
    }
}

impl Hardware {
    pub fn new() -> Self {
        let stdout = io::stdout();
        Self { stdout }
    }

    pub fn clear(&mut self) -> Result<(), &'static str> {
        match self
            .stdout
            .execute(terminal::Clear(terminal::ClearType::All))
        {
            Ok(_) => {
                return Ok(());
            }
            Err(_) => {
                return Err("Clear display error");
            }
        }
    }

    pub fn check_key(key: u8) -> Result<bool, &'static str> {
        let key = match value_to_char(key) {
            Ok(v) => v,
            Err(err) => return Err(err),
        };

        match poll(Duration::from_secs(0)) {
            Ok(available) => {
                if !available {
                    return Ok(false);
                }

                let event = match read() {
                    Ok(e) => e,
                    Err(_) => return Err("Event reading error"),
                };

                match event.as_key_press_event() {
                    None => return Ok(false),
                    Some(event) => {
                        return Ok(event.code.is_char(key));
                    }
                }
            }
            Err(_) => return Err("Polling error"),
        }
    }

    pub fn get_key() -> Result<u8, &'static str> {
        match Self::read_until() {
            Ok(c) => match char_to_value(c) {
                Ok(v) => return Ok(v),
                Err(err) => return Err(err),
            },
            Err(err) => return Err(err),
        }
    }

    fn read_until() -> Result<char, &'static str> {
        loop {
            let event = match read() {
                Ok(e) => e,
                Err(_) => return Err("Event reading error"),
            };

            if let Event::Key(key_event) = event {
                if key_event.kind == KeyEventKind::Press {
                    if key_event.code == KeyCode::Char('0')
                        || key_event.code == KeyCode::Char('1')
                        || key_event.code == KeyCode::Char('2')
                        || key_event.code == KeyCode::Char('3')
                        || key_event.code == KeyCode::Char('4')
                        || key_event.code == KeyCode::Char('5')
                        || key_event.code == KeyCode::Char('6')
                        || key_event.code == KeyCode::Char('7')
                        || key_event.code == KeyCode::Char('8')
                        || key_event.code == KeyCode::Char('9')
                        || key_event.code == KeyCode::Char('a')
                        || key_event.code == KeyCode::Char('b')
                        || key_event.code == KeyCode::Char('c')
                        || key_event.code == KeyCode::Char('d')
                        || key_event.code == KeyCode::Char('e')
                        || key_event.code == KeyCode::Char('f')
                    {
                        match key_event.code.as_char() {
                            None => return Err("Event reading error"),
                            Some(ch) => return Ok(ch),
                        }
                    }
                } else {
                    continue;
                }
            }
        }
    }
}
