// SPDX-License-Identifier: Zlib
use crate::ByondGetError;
use std::{
	fmt::{Display, Formatter, Result as DisplayResult},
	str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OsType {
	Windows,
	Linux,
}

impl OsType {
	pub fn as_str(&self) -> &'static str {
		match self {
			OsType::Windows => "windows",
			OsType::Linux => "linux",
		}
	}

	pub fn url(&self, version: u16, build: u16) -> String {
		match self {
			OsType::Windows => {
				format!(
					"https://www.byond.com/download/build/{version}/{version}.{build}_byond.zip",
				)
			}
			OsType::Linux => format!(
				"https://www.byond.com/download/build/{version}/{version}.{build}_byond_linux.zip",
			),
		}
	}
}

impl AsRef<str> for OsType {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl FromStr for OsType {
	type Err = ByondGetError;

	fn from_str(string: &str) -> Result<Self, Self::Err> {
		let string = string.trim();
		if string.eq_ignore_ascii_case("linux") {
			Ok(OsType::Linux)
		} else if string.eq_ignore_ascii_case("windows")
			|| string.eq_ignore_ascii_case("win")
			|| string.eq_ignore_ascii_case("win32")
		{
			Ok(OsType::Windows)
		} else {
			Err(ByondGetError::BadOs(string.to_owned()))
		}
	}
}

impl Display for OsType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.as_str())
	}
}
