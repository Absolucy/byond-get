use clap::{Parser, ValueEnum};
use color_eyre::eyre::{Context, Report, eyre};
use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	path::PathBuf,
	str::FromStr,
};

/// Quickly download a given BYOND build.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
	/// Only download the binaries.
	#[arg(short, long)]
	pub bin: bool,

	/// Which OS to download BYOND builds for.
	#[arg(short = 't', long, value_enum)]
	#[cfg_attr(windows, arg(default_value_t = OsType::Windows))]
	#[cfg_attr(not(windows), arg(default_value_t = OsType::Linux))]
	pub os: OsType,

	/// Version of BYOND to download, formatted like version.build, i.e
	/// 516.1657.
	pub version: Version,

	/// Output directory for the BYOND files.
	/// Directory will be created if it doesn't exist.
	pub output: PathBuf,
}

impl Args {
	pub fn parse() -> Self {
		Parser::parse()
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Version {
	pub version: u16,
	pub build: u16,
}

impl FromStr for Version {
	type Err = Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (version, build) = s
			.trim()
			.split_once('.')
			.ok_or_else(|| eyre!("Version must be in format 'version.build', got '{s}'"))?;

		let version = version
			.parse::<u16>()
			.wrap_err_with(|| format!("Invalid version: {version}"))?;
		let build = build
			.parse::<u16>()
			.wrap_err_with(|| format!("Invalid build: {build}"))?;

		Ok(Version { version, build })
	}
}

impl Display for Version {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}.{}", self.version, self.build)
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum OsType {
	Windows,
	Linux,
}

impl From<OsType> for byond_get::OsType {
	fn from(value: OsType) -> Self {
		match value {
			OsType::Windows => Self::Windows,
			OsType::Linux => Self::Linux,
		}
	}
}

impl Display for OsType {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", match self {
			Self::Windows => "Windows",
			Self::Linux => "Linux",
		})
	}
}
