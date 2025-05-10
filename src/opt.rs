//! Parse command-line arguments.

use std::path::PathBuf;

use clap::Parser;

use crate::config;
use crate::view;

#[derive(Parser)]
#[clap(name = "wiremix", about = "PipeWire mixer")]
#[command(version)]
pub struct Opt {
    #[clap(
        short = 'c',
        long,
        value_name = "FILE",
        help = "Override default config file path"
    )]
    pub config: Option<PathBuf>,

    #[clap(
        short,
        long,
        value_name = "NAME",
        help = "The name of the remote to connect to"
    )]
    pub remote: Option<String>,

    #[clap(short, long, help = "Target frames per second (or 0 for unlimited)")]
    pub fps: Option<f32>,

    #[clap(
        short = 's',
        long,
        value_name = "NAME",
        help = "Character set to use\n[built-in sets: default, compat, extracompat]"
    )]
    pub char_set: Option<String>,

    #[clap(
        short,
        long,
        value_name = "NAME",
        help = "Theme to use\n[built-in themes: default, nocolor, plain]"
    )]
    pub theme: Option<String>,

    #[clap(
        short,
        long,
        value_parser = clap::value_parser!(config::Peaks),
        help = "Audio peak meters"
    )]
    pub peaks: Option<config::Peaks>,

    #[clap(long, conflicts_with = "mouse", help = "Disable mouse support")]
    pub no_mouse: bool,

    #[clap(long, conflicts_with = "no_mouse", help = "Enable mouse support")]
    pub mouse: bool,

    /// Initial tab view
    #[clap(short = 'v', long, value_name = "VIEW", value_enum, default_value="playback", value_parser = clap::value_parser!(view::NodeType))]
    pub tab: Option<view::NodeType>,

    #[cfg(debug_assertions)]
    #[clap(short, long, help = "Dump events without showing interface")]
    pub dump_events: bool,
}

impl Opt {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
