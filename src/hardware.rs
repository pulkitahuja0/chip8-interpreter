use std::{io::{self, Stdout}, time::Duration};

use crossterm::{ExecutableCommand, event::{poll, read}, terminal};

pub struct Hardware {
    stdout: Stdout
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
        _ => Err("Invalid value used for character")
    }
}

impl Hardware {
    pub fn new() -> Self {
        let stdout = io::stdout();
        Self {
            stdout
        }
    }

    pub fn clear(&mut self) -> Result<(), &'static str> {
        match self.stdout.execute(terminal::Clear(terminal::ClearType::All)) {
            Ok(_) => {
                return Ok(());
            },
            Err(_) => {
                return Err("Clear display error");
            }
        }
    }

    pub fn check_key(key: u8) -> Result<bool, &'static str> {
        let key = match value_to_char(key) {
            Ok(v) => v,
            Err(err) => return Err(err)
        };

        match poll(Duration::from_secs(0)) {
            Ok(available) => {
                if !available {
                    return Ok(false);
                }

                let event = match read() {
                    Ok(e) => e,
                    Err(_) => return Err("Event reading error")
                };

                    match event.as_key_press_event() {
                        None => {
                            return Ok(false)
                        },
                        Some(event) => {
                            return Ok(event.code.is_char(key));
                        }
                    }
            },
            Err(_) => {
                return Err("Polling error")
            }
        }
    }
}