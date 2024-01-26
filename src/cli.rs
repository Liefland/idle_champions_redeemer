use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    /// Retrieve codes from remote API
    #[clap(short, long)]
    pub url: Option<String>,

    /// Provide codes manually
    #[clap(short, long)]
    pub codes: Vec<String>,

    /// Run the setup
    #[clap(long)]
    pub setup: bool,

    /// Prefer obtaining codes remotely (override)
    #[clap(long)]
    pub prefer_remote: bool,

    /// Do not interact with the user (no pauses, no setup)
    #[clap(long)]
    pub no_interaction: bool,

    /// Remove the config file
    #[clap(long)]
    pub clean: bool,

    /// Sleep time in milliseconds between actions
    #[clap(long)]
    pub sleep: Option<u64>,

    /// VERBOSE output
    #[clap(long)]
    pub verbose: bool,
}

pub fn parser() -> Result<Args, &'static str> {
    validate(Args::parse())
}

pub fn validate(args: Args) -> Result<Args, &'static str> {
    if args.url.is_some() && args.codes.len() > 0 {
        return Err("Cannot use both --url and --codes");
    }

    Ok(args)
}
