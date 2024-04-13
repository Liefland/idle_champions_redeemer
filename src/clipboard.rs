use crate::{err, verbose};

pub struct ClipboardIsolation {
    verbose: bool,
    clipboard: arboard::Clipboard,
    previous_clipboard: Option<String>,
}

/// ClipboardIsolation
/// Aims to offer a robust way to isolate the clipboard, and restore it when done.
/// Avoids exposing the users clipboard contents to the application or stdout.
impl ClipboardIsolation {
    pub fn isolate(
        new_clipboard: String,
        verbose: bool,
    ) -> Result<ClipboardIsolation, &'static str> {
        let cb = arboard::Clipboard::new().map_err(|err| {
            err!("Failed to initialize clipboard: {}", err.to_string());
            "Failed to initialize clipboard"
        })?;

        let mut isolation = ClipboardIsolation {
            clipboard: cb,
            previous_clipboard: None,
            verbose,
        };

        isolation.start()?;
        isolation.write_clipboard(&new_clipboard, true)?;

        Ok(isolation)
    }

    fn start(&mut self) -> Result<(), &'static str> {
        verbose!(self, "==> Isolating clipboard");

        self.previous_clipboard = Some(self.read_clipboard()?);

        Ok(())
    }

    fn end(&mut self) -> Result<(), &'static str> {
        verbose!(self, "==> Resetting clipboard");

        let prev = self.previous_clipboard.clone();
        self.previous_clipboard = None;

        match prev {
            Some(text) => self.write_clipboard(&text, false),
            None => {
                Ok(()) // No previous clipboard to restore
            }
        }
    }

    fn write_clipboard(&mut self, contents: &str, show: bool) -> Result<(), &'static str> {
        if show {
            verbose!(self, "==> Writing '{}' to clipboard", contents);
        } else {
            verbose!(self, "==> Writing contents to clipboard");
        }

        self.clipboard.set_text(contents).map_err(|err| {
            err!("Failed to write code to clipboard: {}", err);
            "Failed to write to clipboard"
        })
    }

    fn read_clipboard(&mut self) -> Result<String, &'static str> {
        verbose!(self, "==> Reading from clipboard");

        self.clipboard.get_text().map_err(|err| {
            err!("Failed to read clipboard: {}", err);
            "Failed to read from clipboard"
        })
    }
}

impl Drop for ClipboardIsolation {
    fn drop(&mut self) {
        verbose!(self, "==> Dropping clipboard isolation");

        if let Err(err) = self.end() {
            err!("Failed to reset clipboard: {}", err);
        }
    }
}
