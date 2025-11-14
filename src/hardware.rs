use std::{
    io::{self, Stdout, Write},
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor, event::{Event, KeyCode, KeyEvent, KeyEventKind, poll, read}, style::{self, Stylize}, terminal
};

struct Display {
    buffer: [[bool; 64]; 32] // TODO: bit pack u8 array instead
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: [[false; 64]; 32]
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [[false; 64]; 32]
    }

    pub fn set(&mut self, x: u8, y: u8, pixel: bool) -> bool {
        let curr = self.buffer[y as usize][x as usize];
        if curr && pixel {
            self.buffer[y as usize][x as usize] = false;
            return curr;
        } else if !curr && pixel {
            self.buffer[y as usize][x as usize] = true;
        }
        false
    }

    pub fn get(&self, x: u8, y: u8) -> bool {
        self.buffer[y as usize][x as usize]
    }
}

pub struct Hardware {
    stdout: Stdout,
    display: Display
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
        Self { stdout, display: Display::new() }
    }

    pub fn clear(&mut self) -> Result<(), &'static str> {
        self.display.clear();
        self.draw()
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

    pub fn display_row(&mut self, byte: u8, x: u8, y: u8) -> Result<(), &'static str> {
        let pixels = Self::extract_pixels(byte);

        for i in 0..8 {
            if x + i > 63 {
                break
            }
            self.display.set(x + i, y, pixels[i as usize]);
        }

        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), &'static str> {
        match self
            .stdout
            .execute(terminal::Clear(terminal::ClearType::All))
        {
            Ok(_) => {
            }
            Err(_) => {
                return Err("Clear display error");
            }
        }

        for y in 0..32 {
            for x in 0..64 {
                if self.display.get(x, y) {
                    let _ = self.stdout.queue(cursor::MoveTo(x as u16, y as u16)).unwrap()
                    .queue(style::PrintStyledContent("█".white()));
                } else {
                    let _ = self.stdout.queue(cursor::MoveTo(x as u16, y as u16)).unwrap().queue(style::PrintStyledContent(" ".white()));
                }
            }
        }

        match self.stdout.flush() {
            Ok(()) => Ok(()),
            Err(_) => Err("Stdout flush error")
        }
    }

    fn extract_pixels(byte: u8) -> [bool; 8] {
        let mut pixels = [false; 8];

        for i in 0..8 {
            pixels[i] = ((byte >> (7 - i)) & 1) == 1;
        }

        pixels
    }
}
