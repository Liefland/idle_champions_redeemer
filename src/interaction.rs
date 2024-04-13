use crate::cache::Cache;
use crate::clipboard::ClipboardIsolation;
use crate::config::Instructions;
use crate::{cache, err, progress, verbose};
use enigo::{Keyboard, Mouse};
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
    slow: bool,
    verbose: bool,
}

const CHEST_CODE_LENGTH_SHORT: usize = 12;
const CHEST_CODE_LENGTH_LONG: usize = 16;

macro_rules! sleep_millis {
    ($milliseconds:expr, $slow:expr) => {
        if $slow {
            std::thread::sleep(std::time::Duration::from_millis(($milliseconds + 500)));
        } else {
            std::thread::sleep(std::time::Duration::from_millis($milliseconds));
        }
    };
}

macro_rules! action {
    ($self:ident, $action:expr, $fncall:stmt, $sleep:expr) => {
        verbose!($self, $action);

        $fncall

        sleep_millis!($sleep, $self.slow);
    };
}

macro_rules! key {
    ($self:ident, $action:expr, $key:expr, $sleep:expr) => {
        action!(
            $self,
            $action,
            $self.send_keyclick($key).map_err(|e| {
                err!("Failed to send keypress: {}", e);
                "Failed to send keypress"
            })?,
            $sleep
        );
    };
}

macro_rules! click {
    ($self:ident, $action:expr, $coords:expr, $sleep:expr) => {
        action!(
            $self,
            $action,
            $self.send_click($coords).map_err(|e| {
                err!("Failed to send click: {}", e);
                "Failed to send click"
            })?,
            $sleep
        );
    };
}

impl Interactor {
    pub fn new(
        instructions: Instructions,
        slow: bool,
        verbose: bool,
    ) -> Result<Interactor, &'static str> {
        Ok(Interactor {
            enigo: enigo::Enigo::new(&enigo::Settings::default()).map_err(|e| {
                err!("Failed to initialize enigo: {}", e);
                "Failed to initialize enigo"
            })?,
            instructions,
            verbose,
            slow,
        })
    }

    pub fn redeem_many(&mut self, mut codes: Vec<String>) -> Result<(), Vec<String>> {
        if codes.is_empty() {
            return Ok(());
        }

        let cache_path = cache::path();
        let mut cache = Cache::from_file(&cache_path).unwrap_or_else(|e| {
            err!("Failed to read cache from file: {}", e);
            Cache::new()
        });

        codes.retain(|code| {
            if cache.contains(code) {
                verbose!(self, "Skipping code '{}', already redeemed", code);
                return false;
            }

            true
        });

        // Store mouse position
        let (mouse_x, mouse_y) = self.enigo.location().map_err(|e| {
            err!("Failed to get mouse position: {}", e);
            vec![format!("Failed to get mouse position: {}", e)]
        })?;

        let mut failed_codes: Vec<String> = vec![];

        let len = codes.len();

        if len == 0 {
            println!("No (new) codes to redeem, all of them have already been cached.");
            println!("If you want to redeem them again, clear the cache file (--bust-cache) and try again.");
            return Ok(());
        } else {
            println!("Redeeming {} codes: {}", len, codes.join(", "));
        }

        let (progress_sender, _thread_handle) = progress::bar_create(len);

        for code in codes {
            progress_sender.send(format!("CODE {}", code)).ok();

            if let Err(err) = self.redeem(&code) {
                err!("Failed to redeem code '{}': {}", &code, err);
                failed_codes.push(code.clone());
                progress_sender.send("INC".to_string()).ok();
                sleep_millis!(100, self.slow);
                continue;
            };

            progress_sender.send("INC".to_string()).ok();
            cache.push(code);
            // we need to wait for the chest animation to finish on success
            sleep_millis!(2600, self.slow);
        }
        progress_sender.send("FINISH".to_string()).ok();

        if !failed_codes.is_empty() {
            return Err(failed_codes);
        }

        // Reset mouse position
        self.enigo
            .move_mouse(mouse_x, mouse_y, enigo::Coordinate::Abs)
            .map_err(|e| {
                err!("Failed to move mouse: {}", e);
                vec![format!("Failed to move mouse: {}", e)]
            })?;
        #[cfg(feature = "cache")]
        match cache.write(&cache_path) {
            Ok(_) => {
                verbose!(self, "Cache written to file");
            }
            Err(e) => {
                err!("Failed to write cache to file: {}", e);
            }
        };

        Ok(())
    }

    pub fn redeem(&mut self, code: &str) -> Result<(), String> {
        let normalized_code = self.normalize(code)?;
        let instructions = self.instructions;

        #[cfg(not(feature = "progress"))]
        println!("Redeeming code '{}'", &normalized_code);

        // Isolate the clipboard to prevent interference, it implements Drop and will restore the clipboard when it goes out of scope
        let _cb_isolation = ClipboardIsolation::isolate(normalized_code, self.verbose)?;

        click!(
            self,
            "Clicking 'Unlock a Locked Chest'",
            &instructions.unlock_chest,
            2500
        );
        action!(self, "Pasting the code", self.paste_clipboard()?, 1500);

        // this animation takes forever if successful
        // which is the whole reason i wrote this software in the first place
        key!(self, "Redeeming the code", enigo::Key::Return, 5000);

        // Success case: We got a card to flip
        verbose!(self, "Checking for 'card', two branches possible [A], [B]");
        // Delays here are a bit finnicky.
        // A1's delay is propogated in B1, as we can handle both branches in the same time frame
        // It's possible there's more than one card to flip, in which case we need to hit space multiple times
        key!(self, "[A] Flip card (1/6)", enigo::Key::Space, 10); // A1
        key!(self, "[A] Flip card (2/6?)", enigo::Key::Space, 10); // A1
        key!(self, "[A] Flip card (3/6?)", enigo::Key::Space, 10); // A1
        key!(self, "[A] Flip card (4/6?)", enigo::Key::Space, 10); // A1
        key!(self, "[A] Flip card (5/6?)", enigo::Key::Space, 10); // A1
        key!(self, "[A] Flip card (6/6?)", enigo::Key::Space, 10); // A1
        key!(self, "[B] Dismiss error", enigo::Key::Escape, 1000); // B1
        key!(self, "[B] Closing the chest UI", enigo::Key::Escape, 3000); // A2
        key!(self, "[A] Acknowledging card", enigo::Key::Space, 500); // B2

        Ok(())
    }

    fn normalize(&self, code: &str) -> Result<String, String> {
        let normalized = code.replace('-', "");

        self.validate(&normalized)?;

        Ok(normalized)
    }

    fn validate(&self, code: &str) -> Result<(), String> {
        if code.len() != CHEST_CODE_LENGTH_SHORT && code.len() != CHEST_CODE_LENGTH_LONG {
            return Err(format!(
                "Code must be {} or {} characters long",
                CHEST_CODE_LENGTH_SHORT, CHEST_CODE_LENGTH_LONG
            ));
        }

        Ok(())
    }

    pub fn send_click(&mut self, coords: &Coordinates) -> Result<(), &'static str> {
        verbose!(self, "==> Sending CLICK at X:{}, Y:{}", coords.x, coords.y);

        self.enigo
            .move_mouse(coords.x, coords.y, enigo::Coordinate::Abs)
            .map_err(|e| {
                err!("Failed to move mouse: {}", e);
                "Failed to move mouse"
            })?;

        sleep_millis!(10, false); // Probably not needed

        self.enigo
            .button(enigo::Button::Left, enigo::Direction::Click)
            .map_err(|e| {
                err!("Failed to click mouse: {}", e);
                "Failed to click mouse"
            })?;

        Ok(())
    }

    fn send_keyclick(&mut self, key_press: enigo::Key) -> Result<(), &'static str> {
        verbose!(self, "==> Sending KEY '{:?}'", key_press);

        self.enigo
            .key(key_press, enigo::Direction::Click)
            .map_err(|e| {
                err!("Failed to press key: {}", e);
                "Failed to press key"
            })?;

        Ok(())
    }

    fn paste_clipboard(&mut self) -> Result<(), &'static str> {
        verbose!(self, "==> Pasting clipboard");

        verbose!(self, "==> Sending KEY '{:?}'", enigo::Key::Control);
        self.enigo
            .key(enigo::Key::Control, enigo::Direction::Press)
            .map_err(|e| {
                err!("Failed to press key: {}", e);
                "Failed to press key"
            })?;

        sleep_millis!(25, self.slow);
        self.send_keyclick(enigo::Key::Unicode('v')).map_err(|e| {
            err!("Failed to press key: {}", e);
            "Failed to press key"
        })?;

        sleep_millis!(25, self.slow);
        self.enigo
            .key(enigo::Key::Control, enigo::Direction::Release)
            .map_err(|e| {
                err!("Failed to release key: {}", e);
                "Failed to release key"
            })?;

        Ok(())
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
