use std::path::PathBuf;
use structopt::StructOpt;
use crate::Platform;

#[derive(Debug, StructOpt)]
#[structopt(
name = "Spotify-Importer",
about = "Imports from various music providers into spotify",
version = "v0.0.1",
author = "Donovan Dall - awesomealpineibex@gmail.com"
)]
pub struct Opts {
    /// Turn the app to debug mode (logs stuff)
    #[structopt(short, long)]
    pub debug: bool,

    /// The platform to import
    #[structopt(short = "x", long = "platform", default_value = "tidal")]
    pub platform: Platform,

    /// The playlist to import to
    #[structopt(short = "p", long = "playlist")]
    pub playlist: String,

    /// The location of the file to import
    #[structopt(long = "file", short = "f", parse(from_os_str))]
    pub file: PathBuf,

    //TODO make some fields optional depending on platform
}

pub fn get_opts_args() -> Opts {
    let opts = Opts::from_args();
    opts
}
