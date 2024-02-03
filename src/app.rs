use crate::cli::Args;
use crate::config::{ConfigFile, Instructions};
use crate::interaction::Interactor;
use crate::{config, err, verbose, ExitCode};

pub(crate) enum RunInstructions {
    Local(LocalInstructions),
    Remote(RemoteInstructions),
}

#[allow(dead_code)] // Can be dead code if the feature is not enabled
pub(crate) struct RemoteInstructions {
    pub url: Option<String>,
    pub max_retries: u8,

    pub settings: Settings,
}

pub(crate) struct LocalInstructions {
    pub codes: Vec<String>,

    pub settings: Settings,
}

pub(crate) struct Settings {
    pub slow: bool,
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
        instructions.settings.slow,
        instructions.settings.verbose,
    );

    match interactor.redeem_many(instructions.codes) {
        Ok(_) => Ok(()),
        Err(failed_codes) => {
            err!("Failed to redeem codes: {:?}", failed_codes);
            Err("Failed to redeem at least 1 code")
        }
    }
}

#[cfg(not(feature = "remote"))]
fn run_remote(_instructions: RemoteInstructions) -> Result<(), &'static str> {
    Err("Remote feature not enabled")
}

#[cfg(feature = "remote")]
#[tokio::main(flavor = "current_thread")]
async fn run_remote(instructions: RemoteInstructions) -> Result<(), &'static str> {
    let client = licc::client::CodesClient::new_full(None, instructions.url, None);
    let codes: Vec<String> = crate::remote::get_codes(client, instructions.max_retries)
        .await
        .map(|codes| {
            codes
                .into_iter()
                .map(|code| code.code)
                .collect::<Vec<String>>()
        })?;

    verbose!(
        instructions.settings,
        "Retrieved codes: {}",
        codes.join(", ")
    );

    let mut interactor = Interactor::new(
        instructions.settings.instructions,
        instructions.settings.slow,
        instructions.settings.verbose,
    );

    match interactor.redeem_many(codes) {
        Ok(_) => Ok(()),
        Err(failed_codes) => {
            err!("Failed to redeem codes: {:?}", failed_codes);
            Err("Failed to redeem at least 1 code")
        }
    }
}

impl Settings {
    pub fn from(matches: &Args, config: &ConfigFile) -> Settings {
        Settings {
            slow: matches.slow,
            verbose: matches.verbose,
            instructions: config.instructions,
        }
    }
}

impl RunInstructions {
    pub fn create(matches: Args, config: ConfigFile) -> RunInstructions {
        let require_local_codes = config.remote.is_none()
            && (matches.prefer_remote || config.default_strategy != config::Strategy::Remote);
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
            let (url, max_retries) = match config.remote {
                None => match matches.url {
                    None => (None, 1),
                    Some(url) => (Some(url), 1),
                },
                Some(data) => (data.url.clone(), data.max_retries),
            };

            RunInstructions::Remote(RemoteInstructions {
                url,
                max_retries,
                settings,
            })
        }
    }
}
