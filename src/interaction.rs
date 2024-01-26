use crate::config::Instructions;
use crate::{err, verbose};
use enigo::{KeyboardControllable, MouseControllable};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::stdin;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

pub struct Interactor {
    enigo: enigo::Enigo,
    instructions: Instructions,
    verbose: bool,
    sleep_ms: u64,
}

const CHEST_CODE_LENGTH_SHORT: usize = 12;
const CHEST_CODE_LENGTH_LONG: usize = 16;

impl Interactor {
    pub fn new(instructions: Instructions, sleep_ms: u64, verbose: bool) -> Interactor {
        Interactor {
            enigo: enigo::Enigo::new(),
            instructions,
            verbose,
            sleep_ms,
        }
    }

    pub fn redeem_many(&mut self, codes: Vec<String>) -> Result<(), Vec<String>> {
        let mut failed_codes: Vec<String> = vec![];

        // Store mouse position
        let (mouse_x, mouse_y) = self.enigo.mouse_location();

        for code in codes {
            match self.redeem(&code) {
                Ok(_) => {}
                Err(err) => {
                    err!("Failed to redeem code '{}': {}", code, err);
                    failed_codes.push(code);
                }
            };
            std::thread::sleep(std::time::Duration::from_millis(self.sleep_ms));
        }

        if !failed_codes.is_empty() {
            return Err(failed_codes);
        }

        // Reset mouse position
        self.enigo.mouse_move_to(mouse_x, mouse_y);

        Ok(())
    }

    pub fn redeem(&mut self, code: &str) -> Result<(), String> {
        let normalized_code = self.normalize(code)?;

        let (short_duration, long_duration, duration) = (
            std::time::Duration::from_millis(self.sleep_ms) / 3,
            std::time::Duration::from_millis(self.sleep_ms) * 2,
            std::time::Duration::from_millis(self.sleep_ms),
        );
        let instructions = self.instructions;

        println!("Redeeming code '{}'", code);

        // Press the "Unlock a Locked Chest" button
        self.send_click(&instructions.unlock_chest);
        std::thread::sleep(long_duration); // Longer wait: Unlock chest animation

        if code.len() == CHEST_CODE_LENGTH_SHORT {
            // Switch Character Length
            self.send_click(&instructions.character_switch);
            std::thread::sleep(duration);
        }

        // Submit the Code
        self.send_code(&normalized_code);
        std::thread::sleep(short_duration);

        // Redeem the code
        self.send_keypress(enigo::Key::Return);
        std::thread::sleep(long_duration); // Longer wait: Chest unlock animation

        // Success case: We got a card to flip
        // Flip the "card"
        self.send_keypress(enigo::Key::Space);
        std::thread::sleep(duration);
        // Acknowledge you got the "card"
        self.send_keypress(enigo::Key::Space);
        // Failure case: We got an error (already redeemed, invalid code, etc)
        std::thread::sleep(std::time::Duration::from_millis(100));
        // Close the error
        self.send_keypress(enigo::Key::Return);
        std::thread::sleep(duration);

        // Close the chest UI
        self.send_keypress(enigo::Key::Escape);
        std::thread::sleep(duration);

        Ok(())
    }

    fn normalize(&self, code: &str) -> Result<String, String> {
        let normalized = code.replace('-', "");

        self.validate(&normalized)?;

        Ok(normalized)
    }

    fn validate<'a>(&self, code: &str) -> Result<(), String> {
        if code.len() != CHEST_CODE_LENGTH_SHORT && code.len() != CHEST_CODE_LENGTH_LONG {
            return Err(format!(
                "Code must be {} or {} characters long",
                CHEST_CODE_LENGTH_SHORT, CHEST_CODE_LENGTH_LONG
            ));
        }

        Ok(())
    }

    pub fn send_click(&mut self, coords: &Coordinates) {
        verbose!(self, "Sending CLICK at X:{}, Y:{}", coords.x, coords.y);

        self.enigo.mouse_move_to(coords.x, coords.y);

        std::thread::sleep(std::time::Duration::from_millis(10)); // Probably not needed

        self.enigo.mouse_click(enigo::MouseButton::Left);
    }

    fn send_keypress(&mut self, key_press: enigo::Key) {
        verbose!(self, "Sending KEY '{:?}'", key_press);

        self.enigo.key_click(key_press);
    }

    fn send_code(&mut self, code: &str) {
        verbose!(self, "Sending CODE '{}'", code);
        self.enigo.key_sequence(code);
    }
}

pub fn await_enter() {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Failed to read line");
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(X:{}, Y:{})", self.x, self.y)
    }
}
