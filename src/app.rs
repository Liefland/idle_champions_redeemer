use crate::cli::Args;
use crate::config::{ConfigFile, Instructions};
use crate::interaction::Interactor;
use crate::{app, config, err, verbose, ExitCode};

pub(crate) enum RunInstructions {
    Local(LocalInstructions),
    Remote(RemoteInstructions),
}

pub(crate) struct RemoteInstructions {
    pub url: String,
    pub max_retries: u8,
    pub timeout_ms: u32,

    pub settings: Settings,
}

pub(crate) struct LocalInstructions {
    pub codes: Vec<String>,

    pub settings: Settings,
}

pub(crate) struct Settings {
    pub sleep_ms: u64,
    pub verbose: bool,

    pub instructions: Instructions,
}

pub fn run(instructions: RunInstructions) -> Result<(), &'static str> {
    match instructions {
        RunInstructions::Local(local) => {
            verbose!(local.settings, "Running local..");
            run_local(local)
        }
        RunInstructions::Remote(remote) => {
            verbose!(remote.settings, "Running remote..");
            run_remote(remote)
        }
    }
}

fn run_local(instructions: LocalInstructions) -> Result<(), &'static str> {
    let mut interactor = Interactor::new(
        instructions.settings.instructions,
        instructions.settings.sleep_ms,
        instructions.settings.verbose,
    );

    match interactor.redeem_many(instructions.codes) {
        Ok(_) => {}
        Err(failed_codes) => {
            err!("Failed to redeem codes: {:?}", failed_codes);
        }
    };

    Ok(())
}

fn run_remote(_instructions: RemoteInstructions) -> Result<(), &'static str> {
    Err("Remote not implemented yet")
}

impl Settings {
    pub fn from(matches: &Args, config: &ConfigFile) -> Settings {
        Settings {
            sleep_ms: matches.sleep.unwrap_or(config.sleep_ms),
            verbose: matches.verbose,
            instructions: config.instructions,
        }
    }
}

impl RunInstructions {
    pub fn create(matches: Args, config: ConfigFile) -> RunInstructions {
        let require_local_codes = config.remote.is_none()
            && (matches.prefer_remote || config.default_strategy == config::Strategy::Remote);
        let is_codes_empty = matches.codes.is_empty();

        let settings = Settings::from(&matches, &config);

        if !is_codes_empty || require_local_codes {
            if is_codes_empty {
                err!(
                "No codes provided, pass --codes or permit remote retrieval of codes in the setup"
            );
                std::process::exit(ExitCode::LocalRunFailed.into());
            }

            RunInstructions::Local(LocalInstructions {
                codes: matches.codes,
                settings,
            })
        } else {
            let (url, max_retries, timeout_ms) = match config.remote {
                None => match matches.url {
                    None => {
                        err!("No remote configuration provided, pass an --url or run the setup");
                        std::process::exit(ExitCode::RemoteRunFailed.into());
                    }
                    Some(url) => (url, 1, 4000),
                },
                Some(data) => (data.url.clone(), data.max_retries, data.timeout_ms),
            };

            RunInstructions::Remote(app::RemoteInstructions {
                url,
                max_retries,
                timeout_ms,
                settings,
            })
        }
    }
}
