use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};

struct Sounds {
    stream: OutputStream,
}

impl Sounds {
    pub fn new() -> Self {
        let stream =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default stream");
        Self { stream }
    }

    pub fn play_sound(&self) {
        let sink = Sink::connect_new(&self.stream.mixer());
        let source = SineWave::new(440.0)
            .take_duration(Duration::from_millis(50))
            .amplify(0.20);
        sink.append(source);
        sink.detach();
    }
}

pub struct Timers {
    delay_timer: Arc<Mutex<u8>>,
    sound_timer: Arc<Mutex<u8>>,
}

impl Timers {
    pub fn new(mute: bool) -> Self {
        let delay_timer = Arc::new(Mutex::new(0));
        let sound_timer = Arc::new(Mutex::new(0));

        let delay_clone = Arc::clone(&delay_timer);
        let sound_clone = Arc::clone(&sound_timer);

        let sounds = if !mute { Some(Sounds::new()) } else { None };

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(16)); // 60Hz

                if let Ok(mut delay) = delay_clone.lock() {
                    if *delay > 0 {
                        *delay -= 1;
                    }
                }

                if let Ok(mut sound) = sound_clone.lock() {
                    if *sound > 0 {
                        *sound -= 1;
                        match sounds {
                            Some(ref s) => s.play_sound(),
                            None => {}
                        };
                    }
                }
            }
        });

        Self {
            delay_timer,
            sound_timer,
        }
    }

    pub fn get_delay(&self) -> Result<u8, &'static str> {
        match self.delay_timer.lock() {
            Ok(value) => Ok(*value),
            Err(_) => Err("Failed to lock delay timer"),
        }
    }

    pub fn set_delay(&self, value: u8) -> Result<(), &'static str> {
        match self.delay_timer.lock() {
            Ok(mut timer) => {
                *timer = value;
                Ok(())
            }
            Err(_) => Err("Failed to lock delay timer"),
        }
    }

    pub fn set_sound(&self, value: u8) -> Result<(), &'static str> {
        match self.sound_timer.lock() {
            Ok(mut timer) => {
                *timer = value;
                Ok(())
            }
            Err(_) => Err("Failed to lock sound timer"),
        }
    }
}
