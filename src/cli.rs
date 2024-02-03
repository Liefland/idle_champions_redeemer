use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand, Clone)]
pub enum ConfigCommand {
    /// Prints the config directory
    Dir,

    /// Remove the config file
    Clean,

    /// Run the setup
    Setup,
}

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    /// Retrieve codes from remote API
    #[clap(short, long)]
    #[cfg(feature = "remote")]
    pub url: Option<String>,

    /// Prefer obtaining codes remotely (override)
    #[clap(long)]
    #[cfg(feature = "remote")]
    pub prefer_remote: bool,

    /// Unused
    #[cfg(not(feature = "remote"))]
    #[clap(hide = true)]
    pub url: Option<String>,

    /// Unused
    #[cfg(not(feature = "remote"))]
    #[clap(long)]
    #[clap(hide = true)]
    pub prefer_remote: bool,

    /// Provide codes manually
    #[clap(short, long)]
    pub codes: Vec<String>,

    /// Prints the config directory
    #[clap(subcommand)]
    pub config: Option<ConfigCommand>,

    /// Do not interact with the user (no pauses, no setup)
    #[clap(long)]
    pub no_interaction: bool,

    /// Perform actions slower (guarantees success on slower systems), no effect if config already set to slow
    #[clap(long)]
    pub slow: bool,

    /// VERBOSE output
    #[clap(long)]
    pub verbose: bool,
}

pub fn parser() -> Result<Args, &'static str> {
    validate(Args::parse())
}

pub fn validate(args: Args) -> Result<Args, &'static str> {
    if args.url.is_some() && !args.codes.is_empty() {
        return Err("Cannot use both --url and --codes");
    }

    Ok(args)
}
