#![allow(dead_code, unused_variables)]

use crate::app::{run, RunInstructions};
use crate::interaction::await_enter;
use crate::setup::{is_setup, setup};
mod app;
mod cli;
mod config;
mod interaction;
mod macros;
mod setup;

fn main() -> Result<(), &'static str> {
    let matches = match cli::parser() {
        Ok(m) => m,
        Err(e) => {
            err!("{}", e);
            std::process::exit(ExitCode::CliFailed.into())
        }
    };

    if matches.clean {
        std::process::exit(do_clean(matches).into());
    }

    check_setup(&matches);

    let config = match config::read() {
        Ok(c) => c,
        Err(e) => {
            err!("{}", e);
            std::process::exit(ExitCode::ConfigFailed.into());
        }
    };

    if matches.codes.is_empty()
        && matches.url.is_none()
        && config.default_strategy == config::Strategy::Local
    {
        err!("No codes provided");
        std::process::exit(ExitCode::CliFailed.into());
    }

    if !matches.no_interaction {
        println!("Ensure you are on the Chest menu (default hotkey 'o'), and press ENTER to start redemption.");
        await_enter();
    }

    match run(RunInstructions::create(matches, config)) {
        Ok(_) => {}
        Err(e) => {
            err!("{}", e);
            std::process::exit(ExitCode::RunFailed.into());
        }
    }

    Ok(())
}

fn check_setup(matches: &cli::Args) {
    if !is_setup() || matches.setup {
        if matches.no_interaction {
            err!("Cannot run setup without interaction");
            std::process::exit(ExitCode::SetupFailed.into());
        }

        verbose!(matches, "Running setup..");

        match setup() {
            Ok(_) => {
                verbose!(matches, "Setup completed successfully!");
            }
            Err(e) => {
                err!("{}", e);
                std::process::exit(ExitCode::SetupFailed.into());
            }
        };
    }
}

// Returns `never`
fn do_clean(matches: cli::Args) -> ExitCode {
    verbose!(matches, "Removing config file..");

    match config::remove() {
        Ok(_) => {
            println!("Config file removed successfully!");
            ExitCode::Success
        }
        Err(e) => {
            err!("{}", e);
            ExitCode::CleanFailed
        }
    }
}

#[derive(Debug)]
enum ExitCode {
    Success = 0,
    CliFailed = 1,
    CleanFailed = 2,
    SetupFailed = 3,
    ConfigFailed = 4,
    LocalRunFailed = 5,
    RemoteRunFailed = 6,
    RunFailed = 7,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code as i32
    }
}
