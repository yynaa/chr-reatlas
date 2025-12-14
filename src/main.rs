#[cfg(feature = "bin")]
use std::{
  fs::{File, read_to_string},
  io::{Read, Seek, SeekFrom},
};

#[cfg(feature = "bin")]
use chr_reatlas::{
  atlas::{parse_atlas, render_image_from_atlas},
  chr::read_bytes,
  render::{ChrPalette, render_patterns_with_graduations},
};
#[cfg(feature = "bin")]
use clap::{Args, Parser, Subcommand};
#[cfg(feature = "bin")]
use clap_num::maybe_hex;

#[cfg(feature = "bin")]
#[derive(Parser)]
#[command(
  version,
  about,
  long_about = None,
  propagate_version = true
)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[cfg(feature = "bin")]
#[derive(Subcommand)]
enum Commands {
  /// Get raw CHR to a png
  Get(GetArgs),

  /// Parse .rlts file
  Atlas(AtlasArgs),
}

#[cfg(feature = "bin")]
#[derive(Args)]
struct GetArgs {
  /// path to rom, parsed with nes_rom crate
  #[arg()]
  bin_path: String,

  /// output path
  #[arg()]
  output_path: String,

  /// position in hexadecimal to start reading from
  #[arg(short, value_parser=maybe_hex::<u64>, default_value="0x0000")]
  position: u64,

  /// length of bytes to read
  #[arg(short, value_parser=maybe_hex::<usize>, default_value="0x1000")]
  length: usize,
}

#[cfg(feature = "bin")]
#[derive(Args)]
struct AtlasArgs {
  /// atlas to parse
  #[arg()]
  atlas_path: String,

  /// output path
  #[arg()]
  output_path: String,
}

#[cfg(feature = "bin")]
pub fn main() {
  pretty_env_logger::init_timed();

  let cli = Cli::parse();

  match &cli.command {
    Commands::Get(args) => {
      let mut file = File::open(args.bin_path.clone()).unwrap();
      file.seek(SeekFrom::Start(args.position)).unwrap();
      let mut buf = vec![0; args.length];
      file.read_exact(&mut buf).unwrap();

      let chrs = read_bytes(buf).unwrap();
      let pals = vec![ChrPalette::default(); chrs.len()];
      render_patterns_with_graduations(args.output_path.clone(), chrs, pals).unwrap();
    }

    Commands::Atlas(args) => {
      let atlas_str = read_to_string(args.atlas_path.clone()).unwrap();
      let a = parse_atlas(&atlas_str).unwrap();
      render_image_from_atlas(a, args.output_path.clone()).unwrap();
    }
  }
}

#[cfg(not(feature = "bin"))]
pub fn main() {
  eprintln!("cli requires feature 'bin' to be enabled");
  std::process::exit(1);
}
